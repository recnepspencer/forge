#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalArtifactResidueClassification {
    publication: crate::physical_runtime::record_serving::RecordPublicationResidueObservation,
}

impl PhysicalArtifactResidueClassification {
    pub(in crate::physical_runtime) const fn new(
        publication: crate::physical_runtime::record_serving::RecordPublicationResidueObservation,
    ) -> Self {
        Self { publication }
    }

    pub const fn publication(
        self,
    ) -> crate::physical_runtime::record_serving::RecordPublicationResidueObservation {
        self.publication
    }

    pub const fn requires_inspection(self) -> bool {
        !self.publication.is_empty()
    }
}
