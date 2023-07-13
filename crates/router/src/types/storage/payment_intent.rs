use async_bb8_diesel::AsyncRunQueryDsl;
use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
pub use diesel_models::{
    errors,
    payment_intent::{
        PaymentIntent, PaymentIntentNew, PaymentIntentUpdate, PaymentIntentUpdateInternal,
    },
    schema::payment_intent::dsl,
};
use error_stack::{IntoReport, ResultExt};
use router_env::{instrument, tracing};

use crate::{connection::PgPooledConn, core::errors::CustomResult, types::api};

#[async_trait::async_trait]
pub trait PaymentIntentDbExt: Sized {
    async fn filter_by_constraints(
        conn: &PgPooledConn,
        merchant_id: &str,
        pc: &api::PaymentListConstraints,
    ) -> CustomResult<Vec<Self>, errors::DatabaseError>;

    async fn filter_by_time_constraints(
        conn: &PgPooledConn,
        merchant_id: &str,
        pc: &api::TimeRange,
    ) -> CustomResult<Vec<Self>, errors::DatabaseError>;
}

#[async_trait::async_trait]
impl PaymentIntentDbExt for PaymentIntent {
    #[instrument(skip(conn))]
    async fn filter_by_constraints(
        conn: &PgPooledConn,
        merchant_id: &str,
        pc: &api::PaymentListConstraints,
    ) -> CustomResult<Vec<Self>, errors::DatabaseError> {
        let customer_id = &pc.customer_id;
        let starting_after = &pc.starting_after;
        let ending_before = &pc.ending_before;

        //[#350]: Replace this with Boxable Expression and pass it into generic filter
        // when https://github.com/rust-lang/rust/issues/52662 becomes stable
        let mut filter = <Self as HasTable>::table()
            .filter(dsl::merchant_id.eq(merchant_id.to_owned()))
            .order(dsl::created_at.desc())
            .into_boxed();

        if let Some(customer_id) = customer_id {
            filter = filter.filter(dsl::customer_id.eq(customer_id.to_owned()));
        }
        if let Some(created) = pc.created {
            filter = filter.filter(dsl::created_at.eq(created));
        }
        if let Some(created_lt) = pc.created_lt {
            filter = filter.filter(dsl::created_at.lt(created_lt));
        }
        if let Some(created_gt) = pc.created_gt {
            filter = filter.filter(dsl::created_at.gt(created_gt));
        }
        if let Some(created_lte) = pc.created_lte {
            filter = filter.filter(dsl::created_at.le(created_lte));
        }
        if let Some(created_gte) = pc.created_gte {
            filter = filter.filter(dsl::created_at.gt(created_gte));
        }
        if let Some(starting_after) = starting_after {
            let id = Self::find_by_payment_id_merchant_id(conn, starting_after, merchant_id)
                .await?
                .id;
            filter = filter.filter(dsl::id.gt(id));
        }
        if let Some(ending_before) = ending_before {
            let id = Self::find_by_payment_id_merchant_id(conn, ending_before, merchant_id)
                .await?
                .id;
            filter = filter.filter(dsl::id.lt(id.to_owned()));
        }

        filter = filter.limit(pc.limit);

        crate::logger::debug!(query = %diesel::debug_query::<diesel::pg::Pg, _>(&filter).to_string());

        filter
            .get_results_async(conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::NotFound)
            .attach_printable_lazy(|| "Error filtering records by predicate")
    }

    #[instrument(skip(conn))]
    async fn filter_by_time_constraints(
        conn: &PgPooledConn,
        merchant_id: &str,
        time_range: &api::TimeRange,
    ) -> CustomResult<Vec<Self>, errors::DatabaseError> {
        let start_time = time_range.start_time;
        let end_time = time_range
            .end_time
            .unwrap_or_else(common_utils::date_time::now);

        //[#350]: Replace this with Boxable Expression and pass it into generic filter
        // when https://github.com/rust-lang/rust/issues/52662 becomes stable
        let mut filter = <Self as HasTable>::table()
            .filter(dsl::merchant_id.eq(merchant_id.to_owned()))
            .order(dsl::modified_at.desc())
            .into_boxed();

        filter = filter.filter(dsl::created_at.ge(start_time));

        filter = filter.filter(dsl::created_at.le(end_time));

        filter
            .get_results_async(conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::Others)
            .attach_printable("Error filtering records by time range")
    }
}
