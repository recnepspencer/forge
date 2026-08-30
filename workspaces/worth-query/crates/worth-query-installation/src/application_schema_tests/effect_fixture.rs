use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationEffectMarkerIdentity, ApplicationEffectPayload,
};

#[derive(Clone)]
pub(super) struct TestPayload;

pub(super) struct TestEffect<Schema>(PhantomData<Schema>);

worth_query_declaration::worth_query_portable_type!(
    TestPayload => "worth.query.installation-test.effect-payload"
);

impl ApplicationEffectPayload for TestPayload {
    fn retained_bytes(&self) -> u64 {
        0
    }
}

impl<Schema> ApplicationEffectMarkerIdentity for TestEffect<Schema> {
    type Schema = Schema;
    type Payload = TestPayload;
    const IDENTIFIER: &'static str = "TestEffect";
}
