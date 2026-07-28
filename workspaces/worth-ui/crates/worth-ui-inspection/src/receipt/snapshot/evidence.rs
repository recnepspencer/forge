#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotArtifactPosture {
    GeometryOnly,
    PixelsOptionalOmitted,
    PixelsOptionalCaptured,
    PixelsRequiredCaptured,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiVisualSnapshotEvidence {
    schema_version: u16,
    affinity: super::UiVisualSnapshotAffinity,
    coordinates: super::UiVisualCoordinateObservation,
    visible_index: super::UiVisibleRegionIndexIdentity,
    hit_test_index: super::UiHitTestRegionIndexIdentity,
    artifact: UiVisualSnapshotArtifactPosture,
    disclosure: crate::UiVisualInspectionDisclosure,
    query_budget: super::UiVisualQueryBudget,
    cost: super::UiVisualInspectionCostReceipt,
}

#[doc(hidden)]
pub struct UiVisualSnapshotEvidenceInput {
    pub affinity: super::UiVisualSnapshotAffinity,
    pub coordinates: super::UiVisualCoordinateObservation,
    pub visible_index: super::UiVisibleRegionIndexIdentity,
    pub hit_test_index: super::UiHitTestRegionIndexIdentity,
    pub artifact: UiVisualSnapshotArtifactPosture,
    pub disclosure: crate::UiVisualInspectionDisclosure,
    pub query_budget: super::UiVisualQueryBudget,
    pub cost: super::UiVisualInspectionCostReceipt,
}

impl UiVisualSnapshotEvidence {
    pub const SCHEMA_VERSION: u16 = 1;

    #[doc(hidden)]
    pub const fn from_runtime_projection(input: UiVisualSnapshotEvidenceInput) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            affinity: input.affinity,
            coordinates: input.coordinates,
            visible_index: input.visible_index,
            hit_test_index: input.hit_test_index,
            artifact: input.artifact,
            disclosure: input.disclosure,
            query_budget: input.query_budget,
            cost: input.cost,
        }
    }

    pub const fn schema_version(self) -> u16 {
        self.schema_version
    }

    pub const fn affinity(self) -> super::UiVisualSnapshotAffinity {
        self.affinity
    }

    pub const fn coordinates(self) -> super::UiVisualCoordinateObservation {
        self.coordinates
    }

    pub const fn visible_index(self) -> super::UiVisibleRegionIndexIdentity {
        self.visible_index
    }

    pub const fn hit_test_index(self) -> super::UiHitTestRegionIndexIdentity {
        self.hit_test_index
    }

    pub const fn artifact(self) -> UiVisualSnapshotArtifactPosture {
        self.artifact
    }

    pub const fn disclosure(self) -> crate::UiVisualInspectionDisclosure {
        self.disclosure
    }

    pub const fn query_budget(self) -> super::UiVisualQueryBudget {
        self.query_budget
    }

    pub const fn cost(self) -> super::UiVisualInspectionCostReceipt {
        self.cost
    }
}
