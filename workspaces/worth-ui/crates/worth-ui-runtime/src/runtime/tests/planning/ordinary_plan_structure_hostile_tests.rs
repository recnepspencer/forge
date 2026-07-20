use crate::runtime::plan_topology::{WorthUiPlanRegionStore, WorthUiPlanRegionStoreDenial};
use crate::runtime::planning::execution_plan_input::{
    WorthUiChildRangePlanMeaning, WorthUiComponentPlanMeaning, WorthUiPlanOrdinaryMeaning,
    WorthUiStateSlotMeaningDenial, WorthUiStateSlotPlanMeaning, WorthUiStateSlotSuccession,
};
use crate::runtime::{
    WorthUiDurableStateCarryForward, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationReceipt, WorthUiNodeLifecycleTransition, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput,
};

#[test]
fn swapped_ordinary_meaning_family_denies_before_store_publication() {
    let swapped = WorthUiPlanNodeInput::from_ordinary_row(
        "hostile.range".to_owned(),
        Some(11),
        WorthUiPlanNodeInputFamily::Command,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        None,
        WorthUiPlanOrdinaryMeaning::ChildRange(WorthUiChildRangePlanMeaning::new(
            "hostile.owner".to_owned(),
            Vec::new(),
        )),
    );

    assert_eq!(
        WorthUiPlanRegionStore::try_launch([swapped]).unwrap_err(),
        WorthUiPlanRegionStoreDenial::OrdinaryMeaningFamilyMismatch
    );
}

#[test]
fn duplicate_region_identity_denies_instead_of_overwriting_a_schema() {
    let seed = legacy_leaf("hostile.duplicate", WorthUiPlanNodeInputFamily::Command);
    let duplicate = seed
        .clone()
        .with_family_for_test(WorthUiPlanNodeInputFamily::TokenStyle);

    assert_eq!(
        WorthUiPlanRegionStore::try_launch([seed, duplicate]).unwrap_err(),
        WorthUiPlanRegionStoreDenial::DuplicateRegionIdentity
    );
}

#[test]
fn missing_duplicate_and_cyclic_child_locators_have_distinct_denials() {
    let missing = child_range("hostile.missing", ["hostile.absent"]);
    assert_eq!(
        WorthUiPlanRegionStore::try_launch([missing]).unwrap_err(),
        WorthUiPlanRegionStoreDenial::MissingLinkedRegion
    );

    let leaf = legacy_leaf("hostile.leaf", WorthUiPlanNodeInputFamily::TokenStyle);
    let duplicate = child_range("hostile.duplicate-range", ["hostile.leaf", "hostile.leaf"]);
    assert_eq!(
        WorthUiPlanRegionStore::try_launch([leaf, duplicate]).unwrap_err(),
        WorthUiPlanRegionStoreDenial::DuplicateChildTarget
    );

    let left = child_range("hostile.left", ["hostile.right"]);
    let right = child_range("hostile.right", ["hostile.left"]);
    assert_eq!(
        WorthUiPlanRegionStore::try_launch([left, right]).unwrap_err(),
        WorthUiPlanRegionStoreDenial::CyclicRegionDependency
    );
}

#[test]
fn semantic_sibling_order_is_preserved_in_executable_child_targets() {
    let left_first = ordered_owner_bundle(["hostile.left", "hostile.right"]);
    let right_first = ordered_owner_bundle(["hostile.right", "hostile.left"]);
    let left_store = WorthUiPlanRegionStore::try_launch(left_first)
        .expect("the left-first owner tree should seal")
        .into_store();
    let right_store = WorthUiPlanRegionStore::try_launch(right_first)
        .expect("the right-first owner tree should seal")
        .into_store();

    assert_eq!(
        child_target_identities(&left_store, "hostile.root::child-range"),
        ["hostile.left".to_owned(), "hostile.right".to_owned()]
    );
    assert_eq!(
        child_target_identities(&right_store, "hostile.root::child-range"),
        ["hostile.right".to_owned(), "hostile.left".to_owned()]
    );
    assert!(!left_store.semantically_matches(&right_store).0);
}

#[test]
fn overlapping_child_claims_deny_before_launch_publication() {
    let mut rows = ordered_owner_bundle(["hostile.branch", "hostile.leaf"]);
    let branch = rows
        .iter_mut()
        .find(|row| row.identity_basis() == "hostile.branch")
        .expect("the branch row is present");
    *branch = component_row(
        "hostile.branch",
        Some("hostile.root"),
        Some("hostile.branch::child-range"),
    );
    rows.push(owned_child_range(
        "hostile.branch::child-range",
        "hostile.root",
        "hostile.branch",
        ["hostile.leaf"],
    ));
    let root = rows
        .iter_mut()
        .find(|row| row.identity_basis() == "hostile.root")
        .expect("the owner root is present");
    root.set_owned_region_identity_bases(vec![
        "hostile.root::child-range".to_owned(),
        "hostile.branch".to_owned(),
        "hostile.leaf".to_owned(),
        "hostile.branch::child-range".to_owned(),
    ]);

    assert_eq!(
        WorthUiPlanRegionStore::try_launch(rows).unwrap_err(),
        WorthUiPlanRegionStoreDenial::OverlappingChildTarget
    );
}

#[test]
fn foreign_owner_and_family_state_succession_deny_during_meaning_construction() {
    use crate::capability::{MosaicStateSlotDescriptor, MosaicStateSlotId, MosaicStateSlotKind};

    let descriptor = MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new("hostile.state.scroll").unwrap(),
        MosaicStateSlotKind::scroll_position(),
    );
    let foreign_owner = WorthUiDurableStateReconciliationReceipt::from_carry_forward(
        WorthUiDurableStateCarryForward::new(
            "foreign.owner".to_owned(),
            WorthUiDurableStateFamilyId::ScrollAnchor,
            WorthUiNodeLifecycleTransition::Preserve,
        ),
    );
    assert_eq!(
        WorthUiStateSlotPlanMeaning::new(
            "active.owner".to_owned(),
            descriptor.clone(),
            WorthUiStateSlotSuccession::Reconciled(foreign_owner),
        ),
        Err(WorthUiStateSlotMeaningDenial::ForeignOwnerSuccession)
    );

    let foreign_family = WorthUiDurableStateReconciliationReceipt::from_carry_forward(
        WorthUiDurableStateCarryForward::new(
            "active.owner".to_owned(),
            WorthUiDurableStateFamilyId::FocusChain,
            WorthUiNodeLifecycleTransition::Preserve,
        ),
    );
    assert_eq!(
        WorthUiStateSlotPlanMeaning::new(
            "active.owner".to_owned(),
            descriptor,
            WorthUiStateSlotSuccession::Reconciled(foreign_family),
        ),
        Err(WorthUiStateSlotMeaningDenial::ForeignFamilySuccession)
    );
}

fn child_range<const N: usize>(identity: &str, children: [&str; N]) -> WorthUiPlanNodeInput {
    WorthUiPlanNodeInput::from_ordinary_row(
        identity.to_owned(),
        Some(17),
        WorthUiPlanNodeInputFamily::ChildRange,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        None,
        WorthUiPlanOrdinaryMeaning::ChildRange(WorthUiChildRangePlanMeaning::new(
            identity.to_owned(),
            children.into_iter().map(str::to_owned).collect(),
        )),
    )
}

fn ordered_owner_bundle<const N: usize>(children: [&str; N]) -> Vec<WorthUiPlanNodeInput> {
    let root_identity = "hostile.root";
    let range_identity = "hostile.root::child-range";
    let mut root = component_row(root_identity, None, Some(range_identity));
    root.set_owned_region_identity_bases(
        std::iter::once(range_identity.to_owned())
            .chain(children.iter().map(|identity| (*identity).to_owned()))
            .collect(),
    );
    let mut rows = vec![
        root,
        owned_child_range(range_identity, root_identity, root_identity, children),
    ];
    rows.extend(children.into_iter().map(|identity| {
        legacy_leaf(identity, WorthUiPlanNodeInputFamily::Command)
            .with_owner_identity_basis_for_test(root_identity)
    }));
    rows
}

fn owned_child_range<const N: usize>(
    identity: &str,
    owner_root: &str,
    range_owner: &str,
    children: [&str; N],
) -> WorthUiPlanNodeInput {
    WorthUiPlanNodeInput::from_ordinary_row(
        identity.to_owned(),
        Some(19),
        WorthUiPlanNodeInputFamily::ChildRange,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        Some(owner_root.to_owned()),
        WorthUiPlanOrdinaryMeaning::ChildRange(WorthUiChildRangePlanMeaning::new(
            range_owner.to_owned(),
            children.into_iter().map(str::to_owned).collect(),
        )),
    )
}

fn component_row(
    identity: &str,
    owner: Option<&str>,
    child_range: Option<&str>,
) -> WorthUiPlanNodeInput {
    use crate::capability::{
        ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
        ComponentStateOwnership,
    };
    WorthUiPlanNodeInput::from_ordinary_row(
        identity.to_owned(),
        Some(23),
        WorthUiPlanNodeInputFamily::ComponentInvocation,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiPlanNodeTopologyInput::empty(),
        owner.map(str::to_owned),
        WorthUiPlanOrdinaryMeaning::Component(WorthUiComponentPlanMeaning::new(
            ComponentDescriptor::new(
                ComponentId::new(identity).unwrap(),
                ComponentPropSchema::named(format!("{identity}.props")),
                ComponentChildPolicy::component_children(),
                ComponentStateOwnership::runtime_owned(),
            ),
            child_range.map(str::to_owned),
        )),
    )
}

fn child_target_identities(store: &WorthUiPlanRegionStore, range_identity: &str) -> Vec<String> {
    let identity =
        crate::runtime::plan_topology::WorthUiPlanRegionIdentity::from_exact_basis(range_identity);
    let targets = store
        .executable_for(&identity)
        .expect("the child range executable is present")
        .child_targets_rc();
    targets
        .iter()
        .map(|target| target.region_identity().exact_basis().to_owned())
        .collect()
}

fn legacy_leaf(identity: &str, family: WorthUiPlanNodeInputFamily) -> WorthUiPlanNodeInput {
    super::plan_topology_test_support::topology_fixture()
        .1
        .node_inputs()[0]
        .clone()
        .with_identity_basis_for_test(identity)
        .with_family_for_test(family)
}
