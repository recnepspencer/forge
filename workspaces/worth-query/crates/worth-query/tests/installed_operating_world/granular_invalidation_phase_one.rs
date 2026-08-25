use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, read};

use super::conditional_node_contract::node;
use super::dependency_impact::bind_direct;
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_public_workspace_with, GeometryDomain,
    ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn phase_one_red_control_separates_direct_truth_from_later_signal_execution() {
    let conditional = node(
        "curve-usd-rates-5y",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location =
        domain::WorthQueryConditionalNodeLocation::operation(conditional.identity()).unwrap();
    let (installation, request, snapshots) = conditional_installation_with_change(&conditional);
    let compute_contacts = Arc::new(AtomicUsize::new(0));
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "granular-invalidation-phase-one",
        conditional,
        installation,
        CountedCompute(Arc::clone(&compute_contacts)),
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(&workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let settled = bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    assert_eq!(compute_contacts.load(Ordering::SeqCst), 1);
    let live = match settled.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the settled projection must promote into the live maintenance owner"),
    };
    let installed_manifest = live
        .snapshot()
        .semantic_aspect_dependency_closure()
        .invalidation_manifest();
    assert!(installed_manifest.dependency_count() >= 1);
    assert_eq!(installed_manifest.conditional_truth_count(), 1);
    assert_eq!(
        installed_manifest.role_count(domain::WorthQuerySemanticDependencyRole::ProjectedValue),
        5
    );

    let TransitionOutcome::Success(delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location, 0, request,
            ),
        )
        .unwrap()
    else {
        panic!("the authoritative curve change must reach Bridge delivery")
    };

    assert_eq!(delivery.truth_targets_admitted(), 1);
    assert_eq!(delivery.signal_seeds_emitted(), 1);
    assert_eq!(compute_contacts.load(Ordering::SeqCst), 1);
    assert_eq!(
        delivery.change_set().dependency().locality(),
        &worth_runtime_bridge::facade::BridgeSemanticLocality::SourceRecord
    );

    let granular =
        worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(&delivery);
    let candidates = domain::select_invalidation_candidates(
        live.snapshot().semantic_aspect_dependency_closure(),
        granular,
    )
    .unwrap();
    assert_eq!(candidates.index_lookups(), 5);
    assert_eq!(
        candidates.roles(),
        &[domain::WorthQuerySemanticDependencyRole::ProjectedValue]
    );
    let direct_impact =
        domain::admit_current_invalidation_impact(live.snapshot(), candidates).unwrap();
    assert_eq!(
        direct_impact.class(),
        domain::WorthQueryImpactClass::ValuePatch
    );
    assert_eq!(compute_contacts.load(Ordering::SeqCst), 1);

    let direct_truth_denial = live
        .snapshot()
        .classify_authoritative_impact(&delivery, &live.snapshot().conditional_provenance()[0])
        .unwrap_err();
    assert_eq!(
        direct_truth_denial.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::ConditionalDeliveryMismatch
    );
    assert_eq!(compute_contacts.load(Ordering::SeqCst), 1);

    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let granular =
        worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(&delivery);
    let duplicate =
        worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(&delivery);
    let maintained = domain::maintain_granular_invalidation_deliveries(
        &live,
        &mut workspace,
        [duplicate, granular],
    )
    .unwrap_or_else(|error| {
        panic!("the admitted exact change should complete Query maintenance: {error:?}")
    });
    let domain::WorthQueryPrimaryGranularMaintenanceOutcome::NoRelevantChange(maintained) =
        maintained
    else {
        panic!("the unchanged fixture projection must suppress publication after refresh")
    };
    assert_eq!(maintained.duplicate_delivery_count(), 1);
    assert_eq!(maintained.suppressed_impact_count(), 1);
    assert_eq!(compute_contacts.load(Ordering::SeqCst), 2);

    let fresh_conditional = node(
        "curve-usd-rates-5y",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let (fresh_installation, _fresh_request, fresh_snapshots) =
        conditional_installation_with_change(&fresh_conditional);
    let fresh_harness = PublicBridgeRuntimeHarness::new();
    fresh_harness.set_relational_snapshot(
        fresh_snapshots[0].snapshot_id(),
        fresh_snapshots[0].version_id(),
    );
    let mut fresh_workspace = conditional_public_workspace_with(
        "granular-invalidation-fresh-runtime",
        fresh_conditional,
        fresh_installation,
        CountedCompute(Arc::new(AtomicUsize::new(0))),
        &fresh_harness,
    )
    .unwrap();
    let fresh_installed = fresh_workspace.domain(GeometryDomain).unwrap();
    let fresh_bound = bind_direct(&fresh_workspace, &fresh_installed);
    let fresh_consumer = fresh_bound.consumer_projection_contract().unwrap();
    let fresh_settled = fresh_bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &fresh_workspace,
        )
        .unwrap()
        .execute(&mut fresh_workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(fresh_consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    let stale = worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(&delivery);
    let fresh_live = match fresh_settled.into_lifecycle().promote(&mut fresh_workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the fresh projection must promote into its own live owner"),
    };
    let stale_denial = domain::maintain_granular_invalidation_deliveries(
        &fresh_live,
        &mut fresh_workspace,
        [stale],
    )
    .err()
    .expect("a delivery from the prior runtime must require readmission");
    let domain::WorthQueryPrimaryGranularMaintenanceDenial::Admission(stale_denial) = stale_denial
    else {
        panic!("stale runtime truth must fail during Query admission")
    };
    assert!(matches!(
        stale_denial.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::ForeignRuntime
            | domain::WorthQueryImpactAdmissionDenialKind::StaleInstallation
    ));
}

struct CountedCompute(Arc<AtomicUsize>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CountedCompute
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        crate::suite::installed_operation_fixture::execution_resource_support()
    }

    fn compute(
        &self,
        _context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                1,
            )]),
        ))
    }
}
