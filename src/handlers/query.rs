use async_graphql::{Context, FieldResult, Object};

use crate::common::{responses::SuccessQuery, Logger};
use crate::schemas::{IdPayloadIn, IdPayloadOut};

pub struct Query;

#[Object]
impl Query {
    pub async fn get(
        &self,
        ctx: &Context<'_>,
        input: IdPayloadIn,
    ) -> FieldResult<SuccessQuery<IdPayloadOut>> {
        let logger = ctx.data_unchecked::<Logger>();

        // ** do some cool stuff here (business logic, db fetch, etc.) ** //

        Ok(SuccessQuery {
            req_id: logger.get_req_id(),
            payload: IdPayloadOut { id: input.id },
        }
        .log(logger))
    }
}
