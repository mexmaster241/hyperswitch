use diesel::{associations::HasTable, ExpressionMethods};
use router_env::{instrument, tracing};

use super::generics;
use crate::{
    payouts::{PayoutCreate, PayoutCreateNew, Payouts, PayoutsNew},
    schema::{payout_create::dsl, payouts::dsl as p_dsl},
    PgPooledConn, StorageResult,
};

impl PayoutCreateNew {
    #[instrument(skip(conn))]
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<PayoutCreate> {
        generics::generic_insert(conn, self).await
    }
}

impl PayoutsNew {
    #[instrument(skip(conn))]
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<Payouts> {
        generics::generic_insert(conn, self).await
    }
}

impl PayoutCreate {
    #[instrument(skip(conn))]
    pub async fn find_by_payout_id(conn: &PgPooledConn, payout_id: &str) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::payout_id.eq(payout_id.to_owned()),
        )
        .await
    }

    #[instrument(skip(conn))]
    pub async fn delete_by_payout_id(
        conn: &PgPooledConn,
        payout_id: String,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            dsl::payout_id.eq(payout_id),
        )
        .await
    }
}

impl Payouts {
    #[instrument(skip(conn))]
    pub async fn find_by_payout_id(conn: &PgPooledConn, payout_id: &str) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            p_dsl::payout_id.eq(payout_id.to_owned()),
        )
        .await
    }

    #[instrument(skip(conn))]
    pub async fn delete_by_payout_id(
        conn: &PgPooledConn,
        payout_id: String,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            p_dsl::payout_id.eq(payout_id),
        )
        .await
    }
}
