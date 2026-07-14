mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn mutation_journey_uses_only_mutation_capability_vocabulary() {
    use worth_query::facade::mutation::{
        authoritative, declare, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
        WorthQueryAuthorityLane,
    };

    let declaration = declare(|mutation| {
        mutation
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                WorthQueryAuthoredAspectValue::string("mutation-dx"),
            )
            .build_insert("Task")
    })
    .expect("mutation should declare");
    let mut workspace = PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace("public-mutation-dx")
        .expect("workspace should open");
    let context = authoritative(&workspace).expect("authority should admit");
    let outcome = declaration.using(context).run(&mut workspace);
    let completion = outcome.completed().expect("mutation should complete");
    assert_eq!(
        completion.aftermath().authority_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        completion
            .counters()
            .lower_runtime_execution_completed_count(),
        1
    );
}

#[test]
fn preview_journey_has_distinct_read_only_and_promotion_eligible_states() {
    use worth_query::facade::preview::{
        declare, declare_mutation, for_session, WorthQueryAspectTouch,
        WorthQueryAuthoredAspectValue, WorthQueryPreviewCloseoutKind, WorthQuerySessionLabel,
    };

    let mut workspace = PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace("public-preview-dx")
        .expect("workspace should open");
    let read_label = WorthQuerySessionLabel::scoped_strs("public-preview", ["read-only"])
        .expect("label should build");
    let read_context = for_session(&workspace, read_label.clone()).expect("context should admit");
    let read_outcome = declare(read_label)
        .using(read_context)
        .open_and_close(&mut workspace);
    assert_eq!(
        read_outcome
            .read_only_completion()
            .expect("read-only preview should complete")
            .aftermath()
            .closeout_kind(),
        WorthQueryPreviewCloseoutKind::Discarded
    );

    let promote_label = WorthQuerySessionLabel::scoped_strs("public-preview", ["promote"])
        .expect("label should build");
    let mutation = declare_mutation(|mutation| {
        mutation
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                WorthQueryAuthoredAspectValue::string("preview-dx"),
            )
            .build_insert("Task")
    })
    .expect("mutation should declare");
    let promote_context =
        for_session(&workspace, promote_label.clone()).expect("context should admit");
    let promote_outcome = declare(promote_label)
        .with_mutation(mutation)
        .using(promote_context)
        .open_and_close(&mut workspace);
    let completion = promote_outcome
        .promotion_completion()
        .expect("promotion should complete");
    assert_eq!(
        completion.aftermath().closeout_kind(),
        WorthQueryPreviewCloseoutKind::Promoted
    );
    assert!(!completion
        .promotion_eligibility()
        .identity_for_reporting()
        .is_empty());
}

#[test]
fn workflow_journey_uses_only_workflow_capability_vocabulary() {
    use worth_query::facade::workflow::{
        declare, declare_mutation, preview, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
        WorthQueryPreviewCloseoutKind, WorthQuerySessionLabel,
    };

    let mutation = declare_mutation(|mutation| {
        mutation
            .set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                WorthQueryAuthoredAspectValue::string("workflow-dx"),
            )
            .build_insert("Task")
    })
    .expect("mutation should declare");
    let label = WorthQuerySessionLabel::scoped_strs("public-workflow", ["promotion"])
        .expect("label should build");
    let declaration = declare(label.clone(), mutation);
    let mut workspace = PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace("public-workflow-dx")
        .expect("workspace should open");
    let context = preview(&workspace, label).expect("context should admit");
    let outcome = declaration.using(context).run(&mut workspace);
    assert_eq!(
        outcome
            .completed()
            .expect("workflow should complete")
            .aftermath()
            .closeout_kind(),
        WorthQueryPreviewCloseoutKind::Promoted
    );
}

#[test]
fn domain_journey_lowers_contributed_vocabulary_through_query() {
    use worth_query::facade::domain::{
        declare, declare_mutation, preview, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
        WorthQueryDomainWorkflowContribution, WorthQueryMutationDeclaration,
        WorthQueryMutationDeclarationStop, WorthQueryPreviewCloseoutKind, WorthQuerySessionLabel,
    };

    struct TaskContribution;

    impl WorthQueryDomainWorkflowContribution for TaskContribution {
        type Error = WorthQueryMutationDeclarationStop;

        fn contribute(&self) -> Result<WorthQueryMutationDeclaration, Self::Error> {
            declare_mutation(|mutation| {
                mutation
                    .set_aspect(
                        WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                        WorthQueryAuthoredAspectValue::string("domain-dx"),
                    )
                    .build_insert("Task")
            })
        }
    }

    let label = WorthQuerySessionLabel::scoped_strs("public-domain", ["promotion"])
        .expect("label should build");
    let declaration = declare(label.clone(), TaskContribution).expect("domain should contribute");
    let mut workspace = PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace("public-domain-dx")
        .expect("workspace should open");
    let context = preview(&workspace, label).expect("context should admit");
    let outcome = declaration.using(context).run(&mut workspace);
    assert_eq!(
        outcome
            .completed()
            .expect("domain workflow should complete")
            .workflow()
            .aftermath()
            .closeout_kind(),
        WorthQueryPreviewCloseoutKind::Promoted
    );
}
