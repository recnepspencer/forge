use super::*;

mod original_module {
    use super::*;

    pub(super) struct Parameters;
    pub(super) struct Result;
    pub(super) struct Scope;
    pub(super) struct LiveBinding;
    worth_query_portable_type!(Result => "worth.query.test.module-move.result.v1");
    worth_query_portable_type!(LiveBinding => "worth.query.test.module-move.live-binding.v1");
    crate::worth_query_application_query!(
        pub(super) Query in Schema,
        identity "worth.query.test.module-move.query.v1",
        parameters Parameters => "worth.query.test.module-move.parameters.v1",
        result Result => "worth.query.test.module-move.result.v1",
        scope Scope => "worth.query.test.module-move.scope.v1",
        name "module_move_query"
    );

    impl ApplicationQueryLiveCauseBinding<Schema, super::super::Query, Root, Child> for LiveBinding {
        type Effect = Effect;
        type Payload = Cause;
        type ScopeIdentity = u64;
        type TargetIdentity = u64;

        fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
            ApplicationEffectRef::from_declaration()
        }

        fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
            payload.root
        }

        fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
            payload.child
        }
    }
}

mod relocated_module {
    use super::*;

    pub(super) struct Parameters;
    pub(super) struct Result;
    pub(super) struct Scope;
    pub(super) struct LiveBinding;
    worth_query_portable_type!(Result => "worth.query.test.module-move.result.v1");
    worth_query_portable_type!(LiveBinding => "worth.query.test.module-move.live-binding.v1");
    crate::worth_query_application_query!(
        pub(super) Query in Schema,
        identity "worth.query.test.module-move.query.v1",
        parameters Parameters => "worth.query.test.module-move.parameters.v1",
        result Result => "worth.query.test.module-move.result.v1",
        scope Scope => "worth.query.test.module-move.scope.v1",
        name "module_move_query"
    );

    impl ApplicationQueryLiveCauseBinding<Schema, super::super::Query, Root, Child> for LiveBinding {
        type Effect = Effect;
        type Payload = Cause;
        type ScopeIdentity = u64;
        type TargetIdentity = u64;

        fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
            ApplicationEffectRef::from_declaration()
        }

        fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
            payload.root
        }

        fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
            payload.child
        }
    }
}

#[test]
fn every_query_marker_type_is_identity_bearing() {
    let baseline = typed_definition::<Query>();

    for (dimension, changed) in [
        ("query marker", typed_definition::<OtherQuery>()),
        (
            "parameter-set marker",
            typed_definition::<OtherParametersQuery>(),
        ),
        ("result marker", typed_definition::<OtherResultQuery>()),
        ("scope marker", typed_definition::<OtherScopeQuery>()),
    ] {
        assert_ne!(
            baseline.canonical_basis(),
            changed.canonical_basis(),
            "{dimension} must change the canonical query basis"
        );
    }
}

#[test]
fn continuation_presence_is_identity_bearing() {
    let without_continuation = collection_definition(false);
    let with_continuation = collection_definition(true);

    assert_ne!(
        without_continuation.canonical_basis(),
        with_continuation.canonical_basis()
    );
}

#[test]
fn live_binding_and_each_resource_bound_are_identity_bearing() {
    let baseline =
        live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(8, 64, 512));
    let variants = [
        (
            "binding type",
            live_definition::<OtherLiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 64, 512,
            )),
        ),
        (
            "buffered cause bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                9, 64, 512,
            )),
        ),
        (
            "delivery work bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 65, 512,
            )),
        ),
        (
            "retained payload bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 64, 513,
            )),
        ),
    ];

    for (dimension, changed) in variants {
        assert_ne!(
            baseline.canonical_basis(),
            changed.canonical_basis(),
            "{dimension} must change the canonical query basis"
        );
    }
}

#[test]
fn rust_module_move_cannot_change_query_protocol_identity() {
    assert_ne!(
        std::any::type_name::<original_module::Query>(),
        std::any::type_name::<relocated_module::Query>()
    );
    assert_eq!(
        typed_definition::<original_module::Query>().canonical_basis(),
        typed_definition::<relocated_module::Query>().canonical_basis()
    );

    assert_ne!(
        std::any::type_name::<original_module::LiveBinding>(),
        std::any::type_name::<relocated_module::LiveBinding>()
    );
    assert_eq!(
        live_definition::<original_module::LiveBinding>(
            ApplicationQueryLiveResourceContract::bounded(8, 64, 512),
        )
        .canonical_basis(),
        live_definition::<relocated_module::LiveBinding>(
            ApplicationQueryLiveResourceContract::bounded(8, 64, 512),
        )
        .canonical_basis()
    );
}
