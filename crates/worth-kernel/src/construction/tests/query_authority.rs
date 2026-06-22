use forge_query::facade::consumer_kit::{
    project_workspace_support_snapshot, ForgeQuerySupportPinningErrorKind,
};
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainOperatingContext, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput,
};
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::authoring::{
    require_default_primitive_construction_query_authority,
    require_primitive_construction_query_authority, PrimitiveConstructionOperatingContext,
    PrimitiveConstructionQueryAuthorityRequest, PrimitiveConstructionQueryDeclarationInput,
    PrimitiveConstructionQueryDomain,
};
use crate::construction::request::PrimitiveConstructionFamily;

#[test]
fn construction_query_authority_receipt_carries_configured_operating_context_identity() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-fifteen.query-authority".to_string(),
    )
    .expect("workspace");
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveConstructionQueryDomain)
        .with_operating_context(PrimitiveConstructionOperatingContext::current_head_authoritative())
        .validate()
        .expect("construction context should validate")
        .admit()
        .expect("construction handle should admit");

    let receipt = require_default_primitive_construction_query_authority(&workspace)
        .expect("construction query authority should be admitted");
    let evaluated_support_snapshot = project_workspace_support_snapshot(&workspace);

    assert_eq!(
        receipt.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    assert_eq!(
        receipt.handle_identity_digest(),
        handle.handle_identity_digest()
    );
    assert_eq!(
        receipt.configured_handle_support_snapshot_digest(),
        handle.support_snapshot().snapshot_digest()
    );
    assert_eq!(
        receipt.validated_config_digest(),
        handle.support_snapshot().validated_config_digest()
    );
    assert_eq!(receipt.purpose(), "projection-consumption-surface");
    assert_eq!(
        receipt.authority_basis_digest(),
        workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
    );
    assert!(receipt
        .subject()
        .starts_with("worth-kernel.primitive-construction.current-head:"));
    assert!(receipt
        .request_digest()
        .starts_with("worth-kernel.v1:artifact-identity:sha256:"));
    assert!(receipt
        .support_pin_contract_digest()
        .starts_with("forge.query.evidence-identity.v1:"));
    assert!(receipt
        .support_pin_report_digest()
        .starts_with("forge.query.evidence-identity.v1:"));
    assert_eq!(
        receipt.evaluated_support_snapshot_digest(),
        evaluated_support_snapshot.snapshot_digest()
    );
    assert_eq!(
        receipt.evaluated_support_source_matrix_digest(),
        evaluated_support_snapshot.source_matrix_digest()
    );
    assert_ne!(
        receipt.configured_handle_support_snapshot_digest(),
        receipt.evaluated_support_snapshot_digest()
    );
    assert_eq!(receipt.evaluated_support_pin_count(), 2);
    assert_eq!(receipt.matched_support_pin_count(), 2);
    assert_eq!(receipt.support_pin_finding_count(), 0);
    assert_eq!(receipt.support_pin_blocking_finding_count(), 0);
    assert!(receipt.support_pins_satisfied());
    assert!(receipt
        .authority_receipt_digest()
        .starts_with("worth-kernel.v1:artifact-identity:sha256:"));
}

#[test]
fn construction_operating_modes_do_not_share_dispatch_identity() {
    let current = PrimitiveConstructionOperatingContext::current_head_authoritative();
    let replay = PrimitiveConstructionOperatingContext::certification_replay();

    assert_ne!(current, replay);
    assert_ne!(current.mode(), replay.mode());
    assert_ne!(
        current.context_identity_digest(),
        replay.context_identity_digest()
    );
}

#[test]
fn admitted_construction_handle_declares_runtime_requirements_explicitly() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveConstructionQueryDomain)
        .with_operating_context(PrimitiveConstructionOperatingContext::current_head_authoritative())
        .validate()
        .expect("construction context should validate")
        .admit()
        .expect("construction handle should admit");

    assert_eq!(
        handle.required_capability_families(),
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    );
    assert_eq!(
        handle.required_config_sections(),
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    );
}

#[test]
fn generic_handle_without_concrete_workspace_support_pins_is_rejected() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveConstructionQueryDomain)
        .with_operating_context(PrimitiveConstructionOperatingContext::current_head_authoritative())
        .validate()
        .expect("construction context should validate")
        .admit()
        .expect("construction handle should admit");
    let request = PrimitiveConstructionQueryAuthorityRequest::authority_probe(
        "generic-scaffold-without-topology-runtime",
    );

    let error = require_primitive_construction_query_authority(&handle, request)
        .expect_err("generic configured handle cannot replace pinned workspace support");

    assert_eq!(
        error.support_pinning_kind(),
        Some(ForgeQuerySupportPinningErrorKind::BlockingFindings)
    );
}

#[test]
fn production_construction_declaration_dispatch_carries_operating_context_identity_digest() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveConstructionQueryDomain)
        .with_operating_context(PrimitiveConstructionOperatingContext::current_head_authoritative())
        .validate()
        .expect("construction context should validate")
        .admit()
        .expect("construction handle should admit");
    let declaration = ForgeQueryGroupedDeclarationInput::local_neighborhood(
        PrimitiveConstructionQueryDeclarationInput::new(PrimitiveConstructionFamily::ShellWithHole),
    );

    let dispatch_context_digest = match handle.declare_grouped_checked(declaration) {
        ForgeQueryGroupedDeclarationChecked::Bound(artifact) => artifact
            .graph_obligation_dispatch()
            .expect("construction grouped artifact should retain dispatch")
            .operating_context_identity_digest()
            .to_string(),
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => stop
            .graph_obligation_dispatch()
            .expect("construction grouped stop should retain dispatch")
            .operating_context_identity_digest()
            .to_string(),
    };

    assert_eq!(
        dispatch_context_digest,
        handle.operating_context_identity_digest()
    );
}
