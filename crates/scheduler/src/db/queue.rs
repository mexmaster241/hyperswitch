use common_utils::errors::CustomResult;
use diesel_models::process_tracker as storage;
use redis_interface::{errors::RedisError, RedisEntryId, SetnxReply};
use router_env::logger;
use storage_impl::{mock_db::MockDb, redis::kv_store::RedisConnInterface};

use crate::{errors::ProcessTrackerError, scheduler::Store};

#[async_trait::async_trait]
pub trait QueueInterface {
    async fn fetch_consumer_tasks(
        &self,
        stream_name: &str,
        group_name: &str,
        consumer_name: &str,
    ) -> CustomResult<Vec<storage::ProcessTracker>, ProcessTrackerError>;

    async fn consumer_group_create(
        &self,
        stream: &str,
        group: &str,
        id: &RedisEntryId,
    ) -> CustomResult<(), RedisError>;

    async fn acquire_pt_lock(
        &self,
        tag: &str,
        lock_key: &str,
        lock_val: &str,
        ttl: i64,
    ) -> CustomResult<bool, RedisError>;

    async fn release_pt_lock(&self, tag: &str, lock_key: &str) -> CustomResult<bool, RedisError>;

    async fn stream_append_entry(
        &self,
        stream: &str,
        entry_id: &RedisEntryId,
        fields: Vec<(&str, String)>,
    ) -> CustomResult<(), RedisError>;

    async fn get_key(&self, key: &str) -> CustomResult<Vec<u8>, RedisError>;
}

#[async_trait::async_trait]
impl QueueInterface for Store {
    async fn fetch_consumer_tasks(
        &self,
        stream_name: &str,
        group_name: &str,
        consumer_name: &str,
    ) -> CustomResult<Vec<storage::ProcessTracker>, ProcessTrackerError> {
        crate::consumer::fetch_consumer_tasks(
            self,
            &self
                .get_redis_conn()
                .map_err(ProcessTrackerError::ERedisError)?
                .clone(),
            stream_name,
            group_name,
            consumer_name,
        )
        .await
    }

    async fn consumer_group_create(
        &self,
        stream: &str,
        group: &str,
        id: &RedisEntryId,
    ) -> CustomResult<(), RedisError> {
        self.get_redis_conn()?
            .consumer_group_create(&stream.into(), group, id)
            .await
    }

    async fn acquire_pt_lock(
        &self,
        tag: &str,
        lock_key: &str,
        lock_val: &str,
        ttl: i64,
    ) -> CustomResult<bool, RedisError> {
        let conn = self.get_redis_conn()?.clone();
        let is_lock_acquired = conn
            .set_key_if_not_exists_with_expiry(&lock_key.into(), lock_val, None)
            .await;
        Ok(match is_lock_acquired {
            Ok(SetnxReply::KeySet) => match conn.set_expiry(&lock_key.into(), ttl).await {
                Ok(()) => true,

                #[allow(unused_must_use)]
                Err(error) => {
                    logger::error!(?error);
                    conn.delete_key(&lock_key.into()).await;
                    false
                }
            },
            Ok(SetnxReply::KeyNotSet) => {
                logger::error!(%tag, "Lock not acquired, previous fetch still in progress");
                false
            }
            Err(error) => {
                logger::error!(?error, %tag, "Error while locking");
                false
            }
        })
    }

    async fn release_pt_lock(&self, tag: &str, lock_key: &str) -> CustomResult<bool, RedisError> {
        let is_lock_released = self.get_redis_conn()?.delete_key(&lock_key.into()).await;
        Ok(match is_lock_released {
            Ok(_del_reply) => true,
            Err(error) => {
                logger::error!(?error, %tag, "Error while releasing lock");
                false
            }
        })
    }

    async fn stream_append_entry(
        &self,
        stream: &str,
        entry_id: &RedisEntryId,
        fields: Vec<(&str, String)>,
    ) -> CustomResult<(), RedisError> {
        self.get_redis_conn()?
            .stream_append_entry(&stream.into(), entry_id, fields)
            .await
    }

    async fn get_key(&self, key: &str) -> CustomResult<Vec<u8>, RedisError> {
        self.get_redis_conn()?.get_key::<Vec<u8>>(&key.into()).await
    }
}

#[async_trait::async_trait]
impl QueueInterface for MockDb {
    async fn fetch_consumer_tasks(
        &self,
        _stream_name: &str,
        _group_name: &str,
        _consumer_name: &str,
    ) -> CustomResult<Vec<storage::ProcessTracker>, ProcessTrackerError> {
        // [#172]: Implement function for `MockDb`
        Err(ProcessTrackerError::ResourceFetchingFailed {
            resource_name: "consumer_tasks".to_string(),
        })?
    }

    async fn consumer_group_create(
        &self,
        _stream: &str,
        _group: &str,
        _id: &RedisEntryId,
    ) -> CustomResult<(), RedisError> {
        // [#172]: Implement function for `MockDb`
        Err(RedisError::ConsumerGroupCreateFailed)?
    }

    async fn acquire_pt_lock(
        &self,
        _tag: &str,
        _lock_key: &str,
        _lock_val: &str,
        _ttl: i64,
    ) -> CustomResult<bool, RedisError> {
        // [#172]: Implement function for `MockDb`
        Ok(false)
    }

    async fn release_pt_lock(&self, _tag: &str, _lock_key: &str) -> CustomResult<bool, RedisError> {
        // [#172]: Implement function for `MockDb`
        Ok(false)
    }

    async fn stream_append_entry(
        &self,
        _stream: &str,
        _entry_id: &RedisEntryId,
        _fields: Vec<(&str, String)>,
    ) -> CustomResult<(), RedisError> {
        // [#172]: Implement function for `MockDb`
        Err(RedisError::StreamAppendFailed)?
    }

    async fn get_key(&self, _key: &str) -> CustomResult<Vec<u8>, RedisError> {
        Err(RedisError::RedisConnectionError.into())
    }
}
