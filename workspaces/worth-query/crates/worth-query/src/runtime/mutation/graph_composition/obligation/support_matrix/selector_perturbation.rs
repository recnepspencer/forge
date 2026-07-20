use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector, WorthQueryMutationFamily,
};
use worth_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSelectorPerturbationCase {
    name: &'static str,
    matching_selector: WorthQueryGraphTouchSelector,
    matching_touch: WorthQueryGraphTouchDescriptor,
    non_matching_selector: WorthQueryGraphTouchSelector,
    non_matching_touch: WorthQueryGraphTouchDescriptor,
}

impl WorthQueryGraphObligationSelectorPerturbationCase {
    pub fn milestone_9_9_selector_axis_cases() -> Vec<Self> {
        vec![
            Self::new(
                "collection",
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                mutation_touch("topology.edge", WorthQueryMutationFamily::Update),
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                mutation_touch("topology.face", WorthQueryMutationFamily::Update),
            ),
            Self::new(
                "aspect path",
                WorthQueryGraphTouchSelector::aspect_touch(capacity_aspect_touch()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    WorthQueryMutationFamily::Update,
                    set_capacity_operation(),
                    capacity_aspect_touch(),
                ),
                WorthQueryGraphTouchSelector::aspect_touch(capacity_aspect_touch()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    WorthQueryMutationFamily::Update,
                    set_boundary_operation(),
                    boundary_aspect_touch(),
                ),
            ),
            Self::new(
                "declared operation",
                WorthQueryGraphTouchSelector::declared_aspect_operation(set_capacity_operation()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    WorthQueryMutationFamily::Update,
                    set_capacity_operation(),
                    capacity_aspect_touch(),
                ),
                WorthQueryGraphTouchSelector::declared_aspect_operation(set_capacity_operation()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    WorthQueryMutationFamily::Update,
                    WorthQueryAspectMutationOperation::clear(capacity_aspect_touch()),
                    capacity_aspect_touch(),
                ),
            ),
            Self::new(
                "mutation family",
                WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Update),
                mutation_touch("topology.edge", WorthQueryMutationFamily::Update),
                WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Insert),
                mutation_touch("topology.edge", WorthQueryMutationFamily::Update),
            ),
            Self::new(
                "read verb",
                WorthQueryGraphTouchSelector::read_verb(
                    WorthQueryGraphTouchReadVerb::ObservesRelationKind,
                ),
                read_touch(WorthQueryGraphTouchReadVerb::ObservesRelationKind),
                WorthQueryGraphTouchSelector::read_verb(
                    WorthQueryGraphTouchReadVerb::RequiresPolicyBasis,
                ),
                read_touch(WorthQueryGraphTouchReadVerb::ObservesRelationKind),
            ),
        ]
    }

    fn new(
        name: &'static str,
        matching_selector: WorthQueryGraphTouchSelector,
        matching_touch: WorthQueryGraphTouchDescriptor,
        non_matching_selector: WorthQueryGraphTouchSelector,
        non_matching_touch: WorthQueryGraphTouchDescriptor,
    ) -> Self {
        Self {
            name,
            matching_selector,
            matching_touch,
            non_matching_selector,
            non_matching_touch,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn matching_selector(&self) -> WorthQueryGraphTouchSelector {
        self.matching_selector.clone()
    }

    pub fn matching_touch(&self) -> &WorthQueryGraphTouchDescriptor {
        &self.matching_touch
    }

    pub fn non_matching_selector(&self) -> WorthQueryGraphTouchSelector {
        self.non_matching_selector.clone()
    }

    pub fn non_matching_touch(&self) -> &WorthQueryGraphTouchDescriptor {
        &self.non_matching_touch
    }
}

fn mutation_touch(
    collection: &str,
    family: WorthQueryMutationFamily,
) -> WorthQueryGraphTouchDescriptor {
    mutation_touch_with_aspect(
        collection,
        family,
        set_capacity_operation(),
        capacity_aspect_touch(),
    )
}

fn mutation_touch_with_aspect(
    collection: &str,
    family: WorthQueryMutationFamily,
    declared_aspect_operation: WorthQueryAspectMutationOperation,
    touched_aspect: WorthQueryAspectTouch,
) -> WorthQueryGraphTouchDescriptor {
    WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        collection,
        family,
        None,
        [declared_aspect_operation],
        [touched_aspect],
    )
    .expect("static selector perturbation touch is valid")
}

fn set_capacity_operation() -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(capacity_aspect_touch())
}

fn set_boundary_operation() -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(boundary_aspect_touch())
}

fn capacity_aspect_touch() -> WorthQueryAspectTouch {
    whole_static_aspect_touch("capacity")
}

fn boundary_aspect_touch() -> WorthQueryAspectTouch {
    whole_static_aspect_touch("boundary")
}

fn whole_static_aspect_touch(aspect_label: &'static str) -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::whole_aspect(
        AspectKey::new(aspect_label).expect("static selector aspect key should admit"),
    )
}

fn read_touch(verb: WorthQueryGraphTouchReadVerb) -> WorthQueryGraphTouchDescriptor {
    WorthQueryGraphTouchDescriptor::read_family("topology.edge", [verb])
        .expect("static selector perturbation read touch is valid")
}
