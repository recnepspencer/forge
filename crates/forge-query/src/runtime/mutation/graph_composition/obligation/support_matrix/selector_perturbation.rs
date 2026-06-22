use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

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
                ForgeQueryGraphTouchSelector::aspect_touch(aspect_touch("capacity")),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_operation("capacity"),
                    aspect_touch("capacity"),
                ),
                ForgeQueryGraphTouchSelector::aspect_touch(aspect_touch("capacity")),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_operation("boundary"),
                    aspect_touch("boundary"),
                ),
            ),
            Self::new(
                "declared operation",
                ForgeQueryGraphTouchSelector::declared_aspect_operation(set_operation("capacity")),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    set_operation("capacity"),
                    aspect_touch("capacity"),
                ),
                ForgeQueryGraphTouchSelector::declared_aspect_operation(set_operation("capacity")),
                mutation_touch_with_aspect(
                    "topology.edge",
                    ForgeQueryMutationFamily::Update,
                    ForgeQueryAspectMutationOperation::clear(aspect_touch("capacity")),
                    aspect_touch("capacity"),
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
        set_operation("capacity"),
        aspect_touch("capacity"),
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

fn set_operation(aspect_path: &str) -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::set(aspect_touch(aspect_path))
}

fn aspect_touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::from_authoring_path(aspect_path)
        .expect("static selector aspect path should admit")
}

fn read_touch(verb: ForgeQueryGraphTouchReadVerb) -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::read_family("topology.edge", [verb])
        .expect("static selector perturbation read touch is valid")
}
