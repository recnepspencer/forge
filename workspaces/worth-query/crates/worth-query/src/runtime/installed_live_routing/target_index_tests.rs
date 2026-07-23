use super::*;

#[test]
fn relevance_and_conditional_indexes_return_only_exact_installed_targets() {
    let mut index = WorthQueryInstalledLiveTargetIndex::default();
    let operation = operation_key();
    let location = worth_query_installation::facade::WorthQueryConditionalNodeLocation::operation(
        "indexed-node",
    )
    .unwrap();
    for ordinal in 0..64 {
        index.register(
            WorthQueryLiveArtifactTarget::from_view_name(format!("unrelated-{ordinal}")),
            operation,
            "Vertex".into(),
            selector("identity", "name", None),
        );
    }
    let first = WorthQueryLiveArtifactTarget::from_view_name("installed-first");
    let second = WorthQueryLiveArtifactTarget::from_view_name("installed-second");
    index.register(
        first.clone(),
        operation,
        "Vertex".into(),
        selector("identity", "id", Some(location.clone())),
    );
    index.register(
        second.clone(),
        operation,
        "Vertex".into(),
        selector("identity", "id", Some(location.clone())),
    );

    let expected = BTreeSet::from([first, second]);
    assert_eq!(
        index.affected_targets(&mutation("identity")).targets,
        expected
    );
    assert_eq!(
        index.conditional_targets(operation, &location),
        Some(&expected)
    );
}

#[test]
fn canonical_parent_path_selects_descendant_dependency_without_sibling_scan() {
    let mut index = WorthQueryInstalledLiveTargetIndex::default();
    let target = WorthQueryLiveArtifactTarget::from_view_name("nested-city");
    let mut nested = selector("profile", "city", None);
    nested.aspect_routes.clear();
    nested.field_routes.insert((
        worth_foundational::facade::AspectKey::new("profile").unwrap(),
        path(&["address", "city"]),
    ));
    index.register(target.clone(), operation_key(), "Vertex".into(), nested);

    assert_eq!(
        index.affected_targets(&path_mutation(&["address"])).targets,
        BTreeSet::from([target])
    );
    assert!(index
        .affected_targets(&path_mutation(&["address", "postal"]))
        .targets
        .is_empty());
}

fn operation_key() -> InstalledOperationKey {
    (
        std::any::TypeId::of::<u8>(),
        std::any::TypeId::of::<u16>(),
        std::any::TypeId::of::<u32>(),
    )
}

fn selector(
    aspect: &str,
    field: &str,
    location: Option<worth_query_installation::facade::WorthQueryConditionalNodeLocation>,
) -> crate::domain_installation::WorthQueryInstalledLiveRoutingSelector {
    let aspect = worth_foundational::facade::AspectKey::new(aspect).unwrap();
    crate::domain_installation::WorthQueryInstalledLiveRoutingSelector {
        aspect_routes: BTreeSet::from([aspect.clone()]),
        whole_aspect_routes: BTreeSet::new(),
        field_routes: BTreeSet::from([(
            aspect,
            worth_foundational::facade::CanonicalFieldPath::single(
                worth_foundational::facade::FieldKey::new(field).unwrap(),
            ),
        )]),
        structural_creation: false,
        broad: false,
        empty_touch: false,
        conditional_locations: location.into_iter().collect(),
    }
}

fn mutation(aspect: &str) -> WorthQueryMutationDelta {
    WorthQueryMutationDelta::from_touched_aspects(
        "Vertex",
        crate::memory_workspace::WorthQueryEntityIdentity::from_relational_record(
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
        ),
        WorthQueryMutationKind::Updated,
        vec![crate::runtime::WorthQueryAspectTouch::aspect_field_path(
            worth_foundational::facade::AspectKey::new(aspect).unwrap(),
            worth_foundational::facade::CanonicalFieldPath::single(
                worth_foundational::facade::FieldKey::new("id").unwrap(),
            ),
        )],
    )
}

fn path_mutation(fields: &[&str]) -> WorthQueryMutationDelta {
    WorthQueryMutationDelta::from_touched_aspects(
        "Vertex",
        crate::memory_workspace::WorthQueryEntityIdentity::from_relational_record(
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
        ),
        WorthQueryMutationKind::Updated,
        vec![crate::runtime::WorthQueryAspectTouch::aspect_field_path(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
            path(fields),
        )],
    )
}

fn path(fields: &[&str]) -> worth_foundational::facade::CanonicalFieldPath {
    worth_foundational::facade::CanonicalFieldPath::new(
        fields
            .iter()
            .map(|field| worth_foundational::facade::FieldKey::new(*field).unwrap()),
    )
    .unwrap()
}
