use worth_proof::TransitionOutcome;
use worth_query::facade::domain;

use crate::suite::conditional_node_contract::node;
use crate::suite::installed_operation_fixture::{
    conditional_installation_with_change, conditional_public_workspace_with,
    DirectConditionalCompute, GeometryDomain, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn granular_publication_revalidates_the_exact_shared_consumer_lease() {
    let mut scenario = granular_publication_scenario("granular-consumer-publication");
    let expected_lease = scenario.lease.lease_identity();
    let published = scenario
        .lease
        .maintain_granular_invalidation_for_consumer(
            worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(
                &scenario.delivery,
            ),
            &mut scenario.workspace,
        )
        .unwrap_or_else(|_| panic!("current exact consumer invalidation did not publish"));
    assert_eq!(published.authority().lease_identity(), expected_lease);
    assert_eq!(
        published.roles(),
        [domain::WorthQuerySemanticDependencyRole::ProjectedValue]
    );
    assert_eq!(
        published.consequence_classes(),
        [domain::WorthQueryImpactClass::ValuePatch]
    );
    assert_eq!(published.maintenance_ordinal(), 1);
    assert!(!published.delivery_identity().is_empty());
}

struct GranularPublicationScenario {
    workspace: worth_query::facade::runtime::WorthQueryWorkspace,
    lease: crate::suite::installed_operation_fixture::InvalidationLease,
    delivery: worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
}

fn granular_publication_scenario(name: &str) -> GranularPublicationScenario {
    let declaration = node(
        name,
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location =
        domain::WorthQueryConditionalNodeLocation::operation(declaration.identity()).unwrap();
    let (installation, request, snapshots) = conditional_installation_with_change(&declaration);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        name,
        declaration,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let live = match super::settle(&mut workspace)
        .into_lifecycle()
        .promote(&mut workspace)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("granular publication subject did not promote"),
    };
    let lease = match live.into_managed_lease(&mut workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("granular publication lease stopped: {}", stop.detail())
        }
    };
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
        panic!("granular consumer change did not reach Bridge")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    GranularPublicationScenario {
        workspace,
        lease,
        delivery,
    }
}
