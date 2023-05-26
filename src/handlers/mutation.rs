use async_graphql::{Context, FieldResult, Object};

use crate::common::{responses::SuccessMutation, Logger};
use crate::schemas::{MessageIn, MessageOut};

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn post(
        &self,
        ctx: &Context<'_>,
        input: MessageIn,
    ) -> FieldResult<SuccessMutation<MessageOut>> {
        let logger = ctx.data_unchecked::<Logger>();
        logger.info(&input);

        // ** do some cool stuff here (business logic, db write, etc.) ** //

        Ok(SuccessMutation {
            req_id: logger.get_req_id(),
            payload: MessageOut {
                message: input.message,
            },
        }
        .log(logger))
    }
}
