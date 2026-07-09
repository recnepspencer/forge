use super::{
    compose_admitted_configured_domain_handle_identity, WorthQueryAdmittedWorldBasis,
    WorthQueryDomainOperatingContext,
};
use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfig,
    WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker, WorthQuerySignalConfig,
};
use crate::runtime::tests::support::stateful_bridge_task_runtime;
use crate::runtime::{
    runtime_state_snapshot_basis_label_identity,
    runtime_state_snapshot_result_shape_label_identity, WorthQueryRuntimeStateKind,
    WorthQueryRuntimeStateTarget,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::QueryComposition];
const WORLD_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::HistoricalEvaluation];
const WORLD_SECTIONS: &[WorthQueryConfigSectionFamily] = &[
    WorthQueryConfigSectionFamily::Query,
    WorthQueryConfigSectionFamily::Relational,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl WorthQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "test.geometry.world-basis"
    }

    fn display_name(&self) -> &'static str {
        "GeometryWorldBasisDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryWorld(&'static str);

impl WorthQueryDomainOperatingContext<GeometryDomainEntry> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        WORLD_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        WORLD_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.world-basis.{}", self.0)
    }
}

fn admitted_world_basis(
    facade: &WorthQueryApplicationFacade,
    regime: &'static str,
) -> WorthQueryAdmittedWorldBasis {
    facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryWorld(regime))
        .validate()
        .expect("world should validate")
        .admit()
        .expect("world should admit")
        .retained_world_basis()
}

#[test]
fn retained_world_basis_matches_admitted_handle_identity_and_support() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let handle = facade
        .domain(GeometryDomainEntry)
        .with_operating_context(GeometryWorld("collaborative"))
        .validate()
        .expect("world should validate")
        .admit()
        .expect("world should admit");
    let basis = handle.retained_world_basis();
    let support = crate::query_basis_lifecycle::query_basis_lifecycle_support_report();

    assert_eq!(basis.domain_key(), handle.domain_key());
    assert_eq!(basis.display_name(), handle.display_name());
    assert_eq!(
        basis.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    assert_eq!(
        basis.handle_identity(),
        &compose_admitted_configured_domain_handle_identity(&handle)
    );
    assert_eq!(
        basis.support_snapshot_digest(),
        handle.support_snapshot().snapshot_digest()
    );
    assert_eq!(
        basis.basis_lifecycle_support_identity(),
        &support.report_identity()
    );
}

#[test]
fn equivalent_handles_produce_equivalent_retained_world_bases() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let left = admitted_world_basis(&facade, "collaborative");
    let right = admitted_world_basis(&facade, "collaborative");

    assert_eq!(left, right);
}

#[test]
fn changing_operating_context_changes_world_identity_digests() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let collaborative = admitted_world_basis(&facade, "collaborative");
    let restricted = admitted_world_basis(&facade, "restricted");

    assert_ne!(
        collaborative.operating_context_identity_digest(),
        restricted.operating_context_identity_digest()
    );
    assert_ne!(
        collaborative.handle_identity(),
        restricted.handle_identity()
    );
}

#[test]
fn changing_support_snapshot_changes_support_snapshot_digest_without_rebinding_world_identity() {
    let default_facade = WorthQueryApplicationFacade::runtime_backed_default();
    let signal_disabled_facade = WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default().with_signal(WorthQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled config remains valid");
    let default_basis = admitted_world_basis(&default_facade, "collaborative");
    let signal_disabled_basis = admitted_world_basis(&signal_disabled_facade, "collaborative");

    assert_eq!(
        default_basis.operating_context_identity_digest(),
        signal_disabled_basis.operating_context_identity_digest()
    );
    assert_ne!(
        default_basis.support_snapshot_digest(),
        signal_disabled_basis.support_snapshot_digest()
    );
    assert_eq!(
        default_basis.basis_lifecycle_support_identity(),
        signal_disabled_basis.basis_lifecycle_support_identity()
    );
    assert_ne!(
        default_basis.handle_identity(),
        signal_disabled_basis.handle_identity()
    );
}

#[test]
fn retained_world_basis_projects_into_runtime_state_without_raw_world_reconstruction() {
    let basis = admitted_world_basis(
        &WorthQueryApplicationFacade::runtime_backed_default(),
        "collaborative",
    );
    let runtime = stateful_bridge_task_runtime();
    let state = (&basis)
        .into_state_snapshot(&runtime)
        .expect("retained world basis should project into runtime state");

    assert_eq!(state.kind(), WorthQueryRuntimeStateKind::Ready);
    assert_eq!(
        state.basis_for_reporting(),
        runtime_state_snapshot_basis_label_identity(basis.basis_lifecycle_support_identity())
            .as_str()
    );
    assert_eq!(
        state.result_shape_for_reporting(),
        runtime_state_snapshot_result_shape_label_identity(basis.handle_identity()).as_str()
    );
}
