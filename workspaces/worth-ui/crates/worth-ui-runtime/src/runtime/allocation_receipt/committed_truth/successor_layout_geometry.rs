#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiCommittedSuccessorLayoutGeometry {
    components: [f32; 4],
    truth_revision: super::UiAllocationTruthRevision,
    surface: super::UiAllocationGeometrySurface,
}

impl UiCommittedSuccessorLayoutGeometry {
    pub(in crate::runtime) const fn from_allocation_commit(
        admitted: super::UiAdmittedAllocationGeometry,
        truth_revision: super::UiAllocationTruthRevision,
    ) -> Self {
        Self {
            components: admitted.components(),
            truth_revision,
            surface: admitted.surface(),
        }
    }

    pub(crate) const fn components(self) -> [f32; 4] {
        self.components
    }

    pub(crate) const fn truth_revision(self) -> super::UiAllocationTruthRevision {
        self.truth_revision
    }

    pub(crate) const fn surface(self) -> super::UiAllocationGeometrySurface {
        self.surface
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        components: [f32; 4],
        surface: super::UiAllocationGeometrySurface,
    ) -> Self {
        Self {
            components,
            truth_revision: super::UiAllocationTruthRevision::initial(),
            surface,
        }
    }
}
