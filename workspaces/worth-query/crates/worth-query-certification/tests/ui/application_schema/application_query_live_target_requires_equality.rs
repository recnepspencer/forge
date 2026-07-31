use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryDefinitionBuilder, ApplicationQueryLiveCauseBinding,
        ApplicationQueryLiveResourceContract, ApplicationQueryResultFieldRef,
    },
    application_schema::{
        ApplicationEffectRef, EqualityPredicate, NoApplicationCurrency, NoEqualityPredicate,
        ReadOnly,
    },
};

struct Schema;
struct Query;
struct Parameters;
struct Result;
struct Scope;
struct Target;
struct ScopeSlot;
struct TargetSlot;
struct ScopeAspect;
struct TargetAspect;
struct ScopeField;
struct TargetField;
struct Effect;
struct Binding;

impl ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target> for Binding {
    type Effect = Effect;
    type Payload = String;
    type ScopeIdentity = String;
    type TargetIdentity = String;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
        ApplicationEffectRef::from_schema_identifier("effect")
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.clone()
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.clone()
    }
}

fn declare_live_with_unindexed_target(
    builder: ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, Result, Scope>,
    scope: ApplicationQueryResultFieldRef<
        Query,
        ScopeSlot,
        Schema,
        Scope,
        ScopeAspect,
        ScopeField,
        String,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >,
    target: ApplicationQueryResultFieldRef<
        Query,
        TargetSlot,
        Schema,
        Target,
        TargetAspect,
        TargetField,
        String,
        ReadOnly,
        NoEqualityPredicate,
        NoApplicationCurrency,
    >,
) {
    let _ = builder.live_by::<Target, Binding, _, _, _, _, _, _, _, _>(
        scope,
        target,
        ApplicationQueryLiveResourceContract::bounded(1, 1, 1),
    );
}

fn main() {}
