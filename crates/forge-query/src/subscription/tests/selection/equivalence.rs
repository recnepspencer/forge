use super::*;
use crate::live::LiveQueryFamily;

#[test]
fn construction_source_does_not_change_exact_reuse_subscription_meaning() {
    let mut selections = [
        QuerySubscriptionConstructionSource::Direct,
        QuerySubscriptionConstructionSource::ScopeExpanded,
        QuerySubscriptionConstructionSource::TemplateInstantiated,
        QuerySubscriptionConstructionSource::SavedExactReuse,
        QuerySubscriptionConstructionSource::FacadeLive,
    ]
    .into_iter()
    .map(|source| {
        let input = LiveQueryAdmissionArtifact::for_test(LiveQueryFamily::Detail, None, source);
        select_query_subscription_family(input, roomy_budget()).unwrap()
    });
    let first = selections.next().unwrap();

    for selection in selections {
        assert_eq!(first.family(), selection.family());
        assert_eq!(
            first.equivalence_basis().digest().as_str(),
            selection.equivalence_basis().digest().as_str()
        );
    }
}

#[test]
fn meaning_digest_changes_for_policy_tenant_or_relationship_proof_context() {
    let baseline = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
    );
    let changed_policy = LiveQueryAdmissionArtifact::for_test_with_context(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
        QuerySubscriptionBasisPosture::CurrentHead,
        QuerySubscriptionFutureSelection::ordinary(),
        Some("policy-v2".to_string()),
        Some("tenant".to_string()),
        Some("relationship-proof".to_string()),
        QuerySubscriptionRelationshipProofPosture::Admitted,
    );
    let changed_tenant = LiveQueryAdmissionArtifact::for_test_with_context(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
        QuerySubscriptionBasisPosture::CurrentHead,
        QuerySubscriptionFutureSelection::ordinary(),
        Some("policy".to_string()),
        Some("tenant-beta".to_string()),
        Some("relationship-proof".to_string()),
        QuerySubscriptionRelationshipProofPosture::Admitted,
    );
    let changed_proof = LiveQueryAdmissionArtifact::for_test_with_context(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
        QuerySubscriptionBasisPosture::CurrentHead,
        QuerySubscriptionFutureSelection::ordinary(),
        Some("policy".to_string()),
        Some("tenant".to_string()),
        Some("relationship-proof-v2".to_string()),
        QuerySubscriptionRelationshipProofPosture::Admitted,
    );

    let baseline = select_query_subscription_family(baseline, roomy_budget()).unwrap();
    let changed_policy = select_query_subscription_family(changed_policy, roomy_budget()).unwrap();
    let changed_tenant = select_query_subscription_family(changed_tenant, roomy_budget()).unwrap();
    let changed_proof = select_query_subscription_family(changed_proof, roomy_budget()).unwrap();

    assert_ne!(
        baseline.equivalence_basis().digest().as_str(),
        changed_policy.equivalence_basis().digest().as_str()
    );
    assert_ne!(
        baseline.equivalence_basis().digest().as_str(),
        changed_tenant.equivalence_basis().digest().as_str()
    );
    assert_ne!(
        baseline.equivalence_basis().digest().as_str(),
        changed_proof.equivalence_basis().digest().as_str()
    );
}
