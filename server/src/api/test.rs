use super::*;

use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TestQuery;

#[Object]
impl TestQuery {
    async fn test(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TestSubscription;

#[Subscription]
impl TestSubscription {
    async fn test(&self) -> impl Stream<Item = i32> {
        let mut value = 0;
        let period = StdDuration::from_secs(1);
        IntervalStream::new(interval(period)).map(move |_| {
            value += 1;
            value
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TestMutation;

#[Object]
impl TestMutation {
    async fn test(&self, ctx: &Context<'_>, input: TestInput) -> FieldResult<TestPayload> {
        self.resolve_test(ctx, input).await.into_field_result()
    }

    async fn test_failure(
        &self,
        #[graphql(name = "input")] _input: TestInput,
    ) -> FieldResult<TestPayload> {
        let error = FieldError::new("something went wrong");
        Err(error)
    }
}

impl TestMutation {
    async fn resolve_test(&self, _: &Context<'_>, input: TestInput) -> Result<TestPayload> {
        let TestInput { value } = input;
        let payload = TestPayload { ok: true, value };
        Ok(payload)
    }
}

#[derive(Debug, Clone, Serialize, InputObject)]
pub(super) struct TestInput {
    pub value: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub(super) struct TestPayload {
    pub value: String,
    pub ok: bool,
}
