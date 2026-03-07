use super::context::SubscriberContext;
use super::error::SubscriberContextError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DataId {
    A,
}

#[test]
fn duplicate_stage_is_rejected() {
    let mut ctx: SubscriberContext<DataId> = SubscriberContext::new();
    assert!(ctx.stage(DataId::A, 1u32).is_ok());
    let err = ctx.stage(DataId::A, 2u32).unwrap_err();
    assert_eq!(
        err,
        SubscriberContextError::DuplicateStagedDataId { data_id: DataId::A }
    );
}
