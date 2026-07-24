use super::*;

#[test]
fn owner_refresh_contacts_only_the_receipt_conditional_when_a_sibling_shares_its_scope() {
    let first = node(
        "dependency-impact-owner-node",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let second = node(
        "dependency-impact-sibling-node",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let first_location =
        domain::WorthQueryConditionalNodeLocation::operation(first.identity()).unwrap();
    let first_contacts = Arc::new(AtomicUsize::new(0));
    let second_contacts = Arc::new(AtomicUsize::new(0));
    let harness = PublicBridgeRuntimeHarness::new();
    let (mut workspace, request, snapshots) = conditional_public_sibling_workspace_with_change(
        "dependency-impact-sibling-non-contact",
        first,
        second,
        CountedSiblingCompute(Arc::clone(&first_contacts)),
        CountedSiblingCompute(Arc::clone(&second_contacts)),
        &harness,
    )
    .unwrap();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
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
    let live = match settled.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the two-node settled projection should promote"),
    };
    let first_before = first_contacts.load(Ordering::SeqCst);
    let second_before = second_contacts.load(Ordering::SeqCst);
    assert_eq!(first_before, 1);
    assert_eq!(second_before, 1);

    let TransitionOutcome::Success(delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                first_location.clone(),
                0,
                request,
            ),
        )
        .unwrap()
    else {
        panic!("the owner change should reach node A")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let refreshed = live
        .refresh_owner_delivery(&delivery, &mut workspace)
        .unwrap();

    assert_eq!(first_contacts.load(Ordering::SeqCst), first_before + 1);
    assert_eq!(second_contacts.load(Ordering::SeqCst), second_before);
    assert_eq!(refreshed.work().conditional_dependency_checks(), 1);
    assert_eq!(refreshed.work().conditional_compute_contacts(), 1);
    assert_eq!(
        refreshed.impact().counters().conditional_outcomes_inspected,
        1
    );
}

struct CountedSiblingCompute(Arc<AtomicUsize>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CountedSiblingCompute
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
