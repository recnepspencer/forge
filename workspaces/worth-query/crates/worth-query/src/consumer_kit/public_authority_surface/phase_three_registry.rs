use super::model::{
    WorthQueryPublicAuthorityOwner as Owner, WorthQueryPublicAuthoritySurfaceClass as Class,
    WorthQueryPublicAuthoritySurfaceRow as Row,
};

const SOURCE: &str = "src/basis_lifecycle/declarative.rs";
const FACADE: &str = "src/facade/exports_foundation.rs";

pub(super) fn phase_three_authority_surface_rows() -> &'static [Row] {
    PHASE_THREE_ROWS
}

#[rustfmt::skip]
const PHASE_THREE_ROWS: &[Row] = &[
    ordinary("BasisLifecycleIntentBuilder::branch_snapshot", "branch_snapshot", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentBuilder::preview", "preview", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentBuilder::runtime_snapshot", "runtime_snapshot", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentBuilder::historical_snapshot", "historical_snapshot", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentBuilder::historical_commit", "historical_commit", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentBuilder::tenant_scoped", "tenant_scoped", "BasisLifecycleIntentBuilder"),
    ordinary("BasisLifecycleIntentDraft::observe", "observe", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::prepare_mutation", "prepare_mutation", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::replay", "replay", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::inspect", "inspect", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::materialize", "materialize", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::declare_subscription", "declare_subscription", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::activate_subscription", "activate_subscription", "BasisLifecycleIntentDraft"),
    ordinary("BasisLifecycleIntentDraft::close_preview", "close_preview", "BasisLifecycleIntentDraft"),
    sealed("BasisLifecycleIntentDraft::for_replay", "for_replay", "ReplayBasisAdmissionPath"),
    sealed("BasisLifecycleIntentDraft::for_inspection", "for_inspection", "InspectionBasisAdmissionPath"),
    sealed("BasisLifecycleIntentDraft::for_materialization", "for_materialization", "MaterializationBasisAdmissionPath"),
    sealed("BasisLifecycleIntentDraft::for_subscription_declaration", "for_subscription_declaration", "SubscriptionDeclarationBasisAdmissionPath"),
    sealed("BasisLifecycleIntentDraft::for_subscription_activation", "for_subscription_activation", "SubscriptionActivationBasisAdmissionPath"),
    sealed("BasisLifecycleIntentDraft::for_preview_closeout", "for_preview_closeout", "PreviewCloseoutBasisAdmissionPath"),
];

const fn ordinary(symbol: &'static str, probe: &'static str, facade_probe: &'static str) -> Row {
    row(symbol, probe, facade_probe, Class::OrdinaryDeclarativeApi)
}

const fn sealed(symbol: &'static str, probe: &'static str, facade_probe: &'static str) -> Row {
    row(symbol, probe, facade_probe, Class::SealedPhaseApi)
}

const fn row(
    symbol: &'static str,
    probe: &'static str,
    facade_probe: &'static str,
    class: Class,
) -> Row {
    Row::new(
        symbol,
        SOURCE,
        probe,
        Some(FACADE),
        Some(facade_probe),
        "scoped basis capability production",
        Owner::BasisLifecycle,
        class,
        class,
        symbol,
    )
}
