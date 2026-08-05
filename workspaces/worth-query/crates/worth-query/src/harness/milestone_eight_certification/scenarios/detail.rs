use super::*;

pub(in crate::harness::milestone_eight_certification) fn detail_live_bundle(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> MilestoneEightCertificationBundle {
    let plan = view_plan(
        canonical,
        detail_schema_view(),
        ViewShapeDescriptor::detail(),
    );
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
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
    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "view_patch_width:{}",
                execution.counters().view_patch_width()
            ),
            format!(
                "view_delivery_width:{}",
                execution.counters().view_delivery_width()
            ),
            format!(
                "focused_widening_denial:{}",
                execution
                    .counters()
                    .focused_inspector_widening_denial_count()
            ),
        ],
        "artifact:none".to_string(),
        "support:none".to_string(),
    )
}

pub(in crate::harness::milestone_eight_certification) fn direct_detail_bundle(
) -> MilestoneEightCertificationBundle {
    detail_live_bundle(&direct_detail_canonical("Alice"))
}

pub(in crate::harness::milestone_eight_certification) fn template_detail_bundle(
) -> MilestoneEightCertificationBundle {
    let predicate_slot = TemplateParameterSlot::predicate("name_filter");
    let template = QueryTemplateDescriptor::detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .with_slot(predicate_slot.clone());
    let bindings = TemplateBindingSet::new().bind_predicate(
        &predicate_slot,
        crate::authoring::PredicateSelector::Equality(
            EqualityPredicate::new(
                "profile",
                "display_name",
                WorthQueryPredicateOperand::string("Alice".to_string()),
            )
            .unwrap(),
        ),
    );
    let (_template_artifact, expanded_template) =
        GuidedCompositionPath::instantiate_detail_template(template, bindings).unwrap();
    let template_canonical =
        GuidedCompositionPath::canonicalize_expanded(expanded_template).unwrap();
    detail_live_bundle(template_canonical.canonical())
}

pub(in crate::harness::milestone_eight_certification) fn scope_detail_bundle(
) -> MilestoneEightCertificationBundle {
    let scope = QueryScopeDescriptor::predicate(
        "named_filter",
        [crate::authoring::PredicateSelector::Equality(
            EqualityPredicate::new(
                "profile",
                "display_name",
                WorthQueryPredicateOperand::string("Alice".to_string()),
            )
            .unwrap(),
        )],
    );
    let (_scope_artifact, expanded_scope) = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
        [scope],
    )
    .unwrap();
    let scope_canonical = GuidedCompositionPath::canonicalize_expanded(expanded_scope).unwrap();
    detail_live_bundle(scope_canonical.canonical())
}
