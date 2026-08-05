use super::*;

pub(in crate::harness::milestone_eight_certification) fn saved_query_bundle(
    composed: bool,
) -> MilestoneEightCertificationBundle {
    let support_profile_digest =
        crate::composition::runtime_backed_query_composition_support_profile()
            .profile_digest()
            .to_string();
    if composed {
        let scope = QueryScopeDescriptor::predicate("noop", Vec::new());
        let (_artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
            detail_query_with_name_filter("Alice"),
            detail_shape(),
            [scope],
        )
        .unwrap();
        let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();
        let plan = view_plan(
            composed.canonical(),
            detail_schema_view(),
            ViewShapeDescriptor::detail(),
        );
        let saved = freeze_composed_saved_query(
            &composed,
            &plan,
            SavedQueryFreezeContext::new(&support_profile_digest, "query_composition"),
        )
        .unwrap();
        bundle_from_view_execution(
            saved
                .metadata()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            plan.view_plan_digest().as_str().to_string(),
            saved
                .metadata()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            saved.digest().as_str().to_string(),
            vec![
                format!("template_slots:{}", saved.metadata().template_slot_count()),
                format!(
                    "composition:{}",
                    saved.metadata().composition_digest().as_str()
                ),
            ],
            saved.digest().as_str().to_string(),
            support_profile_digest,
        )
    } else {
        let canonical = direct_detail_canonical("Alice");
        let plan = view_plan(
            &canonical,
            detail_schema_view(),
            ViewShapeDescriptor::detail(),
        );
        let saved = freeze_direct_saved_query(
            &canonical,
            &plan,
            SavedQueryFreezeContext::new(&support_profile_digest, "query_direct"),
        )
        .unwrap();
        bundle_from_view_execution(
            saved
                .metadata()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            plan.view_plan_digest().as_str().to_string(),
            saved
                .metadata()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            saved.digest().as_str().to_string(),
            vec![
                format!("template_slots:{}", saved.metadata().template_slot_count()),
                format!(
                    "composition:{}",
                    saved.metadata().composition_digest().as_str()
                ),
            ],
            saved.digest().as_str().to_string(),
            support_profile_digest,
        )
    }
}
