use super::super::{
    admit_requested_foundational_profile, foundational_profile_progression_authority,
    materialize_admitted_foundational_profile, AdmittedFoundationalProfileArtifact,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionOutcome,
    FoundationalProfileSet, MaterializedFoundationalProfileArtifact,
    RequestedFoundationalProfileArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalProfileProgressionFrontDoor;

impl FoundationalProfileProgressionFrontDoor {
    pub fn admit_as(
        self,
        requested: RequestedFoundationalProfileArtifact,
        admitted: FoundationalProfileSet,
        narrowing: Option<FoundationalProfileNarrowingRecord>,
    ) -> FoundationalProfileProgressionOutcome<AdmittedFoundationalProfileArtifact> {
        admit_requested_foundational_profile(
            requested,
            admitted,
            narrowing,
            foundational_profile_progression_authority(),
        )
    }

    pub fn admit_same(
        self,
        requested: RequestedFoundationalProfileArtifact,
    ) -> FoundationalProfileProgressionOutcome<AdmittedFoundationalProfileArtifact> {
        let admitted = *requested.payload().requested();
        self.admit_as(requested, admitted, None)
    }

    pub fn materialize_as(
        self,
        admitted: AdmittedFoundationalProfileArtifact,
        materialized: FoundationalProfileSet,
        narrowing: Option<FoundationalProfileNarrowingRecord>,
    ) -> FoundationalProfileProgressionOutcome<MaterializedFoundationalProfileArtifact> {
        materialize_admitted_foundational_profile(
            admitted,
            materialized,
            narrowing,
            foundational_profile_progression_authority(),
        )
    }

    pub fn materialize_same(
        self,
        admitted: AdmittedFoundationalProfileArtifact,
    ) -> FoundationalProfileProgressionOutcome<MaterializedFoundationalProfileArtifact> {
        let materialized = *admitted.payload().admitted();
        self.materialize_as(admitted, materialized, None)
    }
}
