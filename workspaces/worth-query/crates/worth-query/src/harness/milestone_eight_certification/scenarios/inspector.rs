use super::*;

pub(in crate::harness::milestone_eight_certification) fn inspector_bundle(
    descriptor: ViewShapeDescriptor,
) -> MilestoneEightCertificationBundle {
    let canonical = GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .unwrap();
    let plan = view_plan(&canonical, detail_schema_view(), descriptor);
    let bound_identity = match plan.delivery_metadata().identity_consumption() {
        crate::view_shape::ViewShapeIdentityConsumption::None => None,
        crate::view_shape::ViewShapeIdentityConsumption::InspectorIdentitySummary => {
            Some(inspector_identity_artifact_for_classification(
                InspectorIdentityClassification::IdentitySummary,
            ))
        }
        crate::view_shape::ViewShapeIdentityConsumption::FocusedInspectorIdentityClassification(
            classification,
        ) => Some(inspector_identity_artifact_for_classification(
            *classification,
        )),
    };
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        bound_identity,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &crate::live::BridgeChangeSummary::default().with_field_delta(
            crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ),
        ),
    )
    .unwrap();
    let (inspector_identity_digest, inspector_identity_classification) = match execution
        .patch_envelope()
        .payload()
    {
        crate::view_shape_live::ViewShapePatchPayload::ObservedInspectorPatch(patch) => patch
            .inspector_identity()
            .map(|identity| {
                (
                    identity.digest().as_str().to_string(),
                    identity.classification().as_str().to_string(),
                )
            })
            .unwrap_or_else(|| ("none".to_string(), "none".to_string())),
        crate::view_shape_live::ViewShapePatchPayload::FocusedInspectorAspectPatch(patch) => patch
            .inspector_identity()
            .map(|identity| {
                (
                    identity.digest().as_str().to_string(),
                    identity.classification().as_str().to_string(),
                )
            })
            .unwrap_or_else(|| ("none".to_string(), "none".to_string())),
        _ => ("none".to_string(), "none".to_string()),
    };

    bundle_from_view_execution_with_identity(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "patch_family:{:?}",
                execution.patch_envelope().patch_family()
            ),
            format!(
                "observed_delivery_width:{}",
                execution.counters().observed_inspector_delivery_width()
            ),
            format!(
                "focused_projection_width:{}",
                execution.counters().focused_inspector_projection_width()
            ),
            format!(
                "identity_consumption:{}",
                plan.delivery_metadata().identity_consumption().as_str()
            ),
        ],
        "artifact:none".to_string(),
        "support:none".to_string(),
        plan.delivery_metadata()
            .identity_consumption()
            .digest()
            .as_str()
            .to_string(),
        inspector_identity_digest,
        inspector_identity_classification,
    )
}

pub(in crate::harness::milestone_eight_certification) fn identity_query_digest(
    label: &str,
) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("milestone-eight:{label}")])
}

pub(in crate::harness::milestone_eight_certification) fn identity_basis_digest(
    label: &str,
) -> BasisDigest {
    BasisDigest::from_parts(&[format!("milestone-eight:{label}")])
}

pub(in crate::harness::milestone_eight_certification) fn inspector_identity_artifact_for_classification(
    classification: InspectorIdentityClassification,
) -> InspectorIdentityArtifact {
    let (context, scenario) = match classification {
        InspectorIdentityClassification::IdentitySummary => (
            IdentityEvolutionQueryContext::lineage_traversal_for_test(
                identity_query_digest("identity-summary"),
                identity_basis_digest("identity-summary-basis"),
                LineageTraversalDescriptor::direct_split_successors("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::AuthoritativeContinuity => (
            IdentityEvolutionQueryContext::lineage_traversal_for_test(
                identity_query_digest("authoritative"),
                identity_basis_digest("authoritative-basis"),
                LineageTraversalDescriptor::direct_replacement("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::AdvisoryCandidates => (
            IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
                identity_query_digest("advisory"),
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                identity_basis_digest("left"),
                identity_basis_digest("right"),
                CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::IdentityBreak => (
            IdentityEvolutionQueryContext::lineage_traversal_for_test(
                identity_query_digest("identity-break"),
                identity_basis_digest("identity-break-basis"),
                LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ),
        other => panic!("milestone eight helper does not support '{other:?}'"),
    };
    let admitted = admit_identity_evolution_query_for_scenario(context, scenario)
        .expect("milestone eight identity artifact should admit");
    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("milestone eight identity artifact should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    InspectorIdentityArtifact::from_result_evidence(&evidence)
}
