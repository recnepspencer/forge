#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotComparisonBudget {
    maximum_structural_entries: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonBudgetDenial {
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualComparisonPixelPolicy {
    Omit,
    IfAlreadyRetained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualIdentityContinuity {
    Preserved,
    Rebound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotComparisonCost {
    structural_entries_examined: usize,
    retained_pixel_bytes_examined: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotComparison {
    predecessor_snapshot: u64,
    successor_snapshot: u64,
    predecessor_frame: u64,
    successor_frame: u64,
    predecessor_binding: u64,
    successor_binding: u64,
    continuity: UiVisualIdentityContinuity,
    predecessor_visible_regions: usize,
    successor_visible_regions: usize,
    retained_pixels_differ: Option<bool>,
    cost: UiVisualSnapshotComparisonCost,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotComparisonInput {
    pub snapshots: [u64; 2],
    pub frames: [u64; 2],
    pub bindings: [u64; 2],
    pub continuity: UiVisualIdentityContinuity,
    pub visible_regions: [usize; 2],
    pub retained_pixels_differ: Option<bool>,
    pub cost: [usize; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonOmission {
    NoRetainedPixelPair,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonExpiry {
    Predecessor,
    Successor,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonIncompatibility {
    ForeignSession,
    DisclosureMismatch,
    RebindHasNoMountedPublication,
    PredecessorFrameMismatch,
    PredecessorLineageMismatch,
    SuccessorFrameMismatch,
    SurfaceMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotComparisonDenial {
    kind: UiVisualSnapshotComparisonDenialKind,
    configured_structural_entries: usize,
    required_structural_entries: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonDenialKind {
    StructuralBudget,
    RetainedSnapshotCapacity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotComparisonOutcome {
    Compared(UiVisualSnapshotComparison),
    Omitted(UiVisualSnapshotComparisonOmission),
    Expired(UiVisualSnapshotComparisonExpiry),
    Incompatible(UiVisualSnapshotComparisonIncompatibility),
    Denied(UiVisualSnapshotComparisonDenial),
}

impl UiVisualSnapshotComparisonBudget {
    pub const fn bounded(
        maximum_structural_entries: usize,
    ) -> Result<Self, UiVisualSnapshotComparisonBudgetDenial> {
        if maximum_structural_entries == 0 {
            return Err(UiVisualSnapshotComparisonBudgetDenial::Empty);
        }
        Ok(Self {
            maximum_structural_entries,
        })
    }

    pub const fn maximum_structural_entries(self) -> usize {
        self.maximum_structural_entries
    }
}

impl UiVisualSnapshotComparison {
    #[doc(hidden)]
    pub const fn from_runtime_projection(input: UiVisualSnapshotComparisonInput) -> Self {
        Self {
            predecessor_snapshot: input.snapshots[0],
            successor_snapshot: input.snapshots[1],
            predecessor_frame: input.frames[0],
            successor_frame: input.frames[1],
            predecessor_binding: input.bindings[0],
            successor_binding: input.bindings[1],
            continuity: input.continuity,
            predecessor_visible_regions: input.visible_regions[0],
            successor_visible_regions: input.visible_regions[1],
            retained_pixels_differ: input.retained_pixels_differ,
            cost: UiVisualSnapshotComparisonCost {
                structural_entries_examined: input.cost[0],
                retained_pixel_bytes_examined: input.cost[1],
            },
        }
    }

    pub const fn snapshot_identities(self) -> [u64; 2] {
        [self.predecessor_snapshot, self.successor_snapshot]
    }

    pub const fn frame_identities(self) -> [u64; 2] {
        [self.predecessor_frame, self.successor_frame]
    }

    pub const fn binding_generations(self) -> [u64; 2] {
        [self.predecessor_binding, self.successor_binding]
    }

    pub const fn continuity(self) -> UiVisualIdentityContinuity {
        self.continuity
    }

    pub const fn visible_region_counts(self) -> [usize; 2] {
        [
            self.predecessor_visible_regions,
            self.successor_visible_regions,
        ]
    }

    pub const fn retained_pixels_differ(self) -> Option<bool> {
        self.retained_pixels_differ
    }

    pub const fn cost(self) -> UiVisualSnapshotComparisonCost {
        self.cost
    }
}

impl UiVisualSnapshotComparisonCost {
    pub const fn structural_entries_examined(self) -> usize {
        self.structural_entries_examined
    }

    pub const fn retained_pixel_bytes_examined(self) -> usize {
        self.retained_pixel_bytes_examined
    }
}

impl UiVisualSnapshotComparisonDenial {
    #[doc(hidden)]
    pub const fn budget_exceeded(configured: usize, required: usize) -> Self {
        Self {
            kind: UiVisualSnapshotComparisonDenialKind::StructuralBudget,
            configured_structural_entries: configured,
            required_structural_entries: required,
        }
    }

    #[doc(hidden)]
    pub const fn retained_snapshot_capacity(configured: usize, required: usize) -> Self {
        Self {
            kind: UiVisualSnapshotComparisonDenialKind::RetainedSnapshotCapacity,
            configured_structural_entries: configured,
            required_structural_entries: required,
        }
    }

    pub const fn kind(self) -> UiVisualSnapshotComparisonDenialKind {
        self.kind
    }

    pub const fn configured_structural_entries(self) -> usize {
        self.configured_structural_entries
    }

    pub const fn required_structural_entries(self) -> usize {
        self.required_structural_entries
    }
}

#[cfg(test)]
mod visual_snapshot_comparison_tests {
    use super::*;

    #[test]
    fn visual_snapshot_comparison_budget_rejects_unbounded_zero() {
        assert_eq!(
            UiVisualSnapshotComparisonBudget::bounded(0),
            Err(UiVisualSnapshotComparisonBudgetDenial::Empty)
        );
        assert_eq!(
            UiVisualSnapshotComparisonBudget::bounded(128)
                .unwrap()
                .maximum_structural_entries(),
            128
        );
    }
}
