use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};
use forge_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSelectorPerturbationCase {
    name: &'static str,
    matching_selector: ForgeQueryGraphTouchSelector,
    matching_touch: ForgeQueryGraphTouchDescriptor,
    non_matching_selector: ForgeQueryGraphTouchSelector,
    non_matching_touch: ForgeQueryGraphTouchDescriptor,
}

impl ForgeQueryGraphObligationSelectorPerturbationCase {
    pub fn milestone_9_9_selector_axis_cases() -> Vec<Self> {
        vec![
            Self::new(
                "collection",
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                mutation_touch("topology.edge", ForgeQueryMutationFamily::Update),
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                mutation_touch("topology.face", ForgeQueryMutationFamily::Update),
            ),
            Self::new(
                "aspect path",
                ForgeQueryGraphTouchSelector::aspect_touch(capacity_aspect_touch()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_capacity_operation(),
                    capacity_aspect_touch(),
                ),
                ForgeQueryGraphTouchSelector::aspect_touch(capacity_aspect_touch()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_boundary_operation(),
                    boundary_aspect_touch(),
                ),
            ),
            Self::new(
                "declared operation",
                ForgeQueryGraphTouchSelector::declared_aspect_operation(set_capacity_operation()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_capacity_operation(),
                    capacity_aspect_touch(),
                ),
                ForgeQueryGraphTouchSelector::declared_aspect_operation(set_capacity_operation()),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    ForgeQueryAspectMutationOperation::clear(capacity_aspect_touch()),
                    capacity_aspect_touch(),
                ),
            ),
            Self::new(
                "mutation family",
                ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Update),
                mutation_touch("topology.edge", ForgeQueryMutationFamily::Update),
                ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Insert),
                mutation_touch("topology.edge", ForgeQueryMutationFamily::Update),
            ),
            Self::new(
                "read verb",
                ForgeQueryGraphTouchSelector::read_verb(
                    ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
                ),
                read_touch(ForgeQueryGraphTouchReadVerb::ObservesRelationKind),
                ForgeQueryGraphTouchSelector::read_verb(
                    ForgeQueryGraphTouchReadVerb::RequiresPolicyBasis,
                ),
                read_touch(ForgeQueryGraphTouchReadVerb::ObservesRelationKind),
            ),
        ]
    }

    fn new(
        name: &'static str,
        matching_selector: ForgeQueryGraphTouchSelector,
        matching_touch: ForgeQueryGraphTouchDescriptor,
        non_matching_selector: ForgeQueryGraphTouchSelector,
        non_matching_touch: ForgeQueryGraphTouchDescriptor,
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

    pub fn matching_selector(&self) -> ForgeQueryGraphTouchSelector {
        self.matching_selector.clone()
    }

    pub fn matching_touch(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.matching_touch
    }

    pub fn non_matching_selector(&self) -> ForgeQueryGraphTouchSelector {
        self.non_matching_selector.clone()
    }

    pub fn non_matching_touch(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.non_matching_touch
    }
}

fn mutation_touch(
    collection: &str,
    family: ForgeQueryMutationFamily,
) -> ForgeQueryGraphTouchDescriptor {
    mutation_touch_with_aspect(
        collection,
        family,
        set_capacity_operation(),
        capacity_aspect_touch(),
    )
}

fn mutation_touch_with_aspect(
    collection: &str,
    family: ForgeQueryMutationFamily,
    declared_aspect_operation: ForgeQueryAspectMutationOperation,
    touched_aspect: ForgeQueryAspectTouch,
) -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        collection,
        family,
        None,
        [declared_aspect_operation],
        [touched_aspect],
    )
    .expect("static selector perturbation touch is valid")
}

fn set_capacity_operation() -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::set(capacity_aspect_touch())
}

fn set_boundary_operation() -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::set(boundary_aspect_touch())
}

fn capacity_aspect_touch() -> ForgeQueryAspectTouch {
    whole_static_aspect_touch("capacity")
}

fn boundary_aspect_touch() -> ForgeQueryAspectTouch {
    whole_static_aspect_touch("boundary")
}

fn whole_static_aspect_touch(aspect_label: &'static str) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(
        AspectKey::new(aspect_label).expect("static selector aspect key should admit"),
    )
}

fn read_touch(verb: ForgeQueryGraphTouchReadVerb) -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::read_family("topology.edge", [verb])
        .expect("static selector perturbation read touch is valid")
}
