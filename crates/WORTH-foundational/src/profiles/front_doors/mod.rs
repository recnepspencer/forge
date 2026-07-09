mod attachment;
mod certification;
mod materialization;
mod progression;
mod set;
mod vocabulary;

pub use attachment::{
    FoundationalProfileAttachmentFrontDoor, MaterializedBoundaryArtifactStep,
    MaterializedProofBearingArtifactStep, MaterializedSupportArtifactStep,
};
pub use certification::FoundationalProfileCertificationFrontDoor;
pub use materialization::FoundationalProfileMaterializationFrontDoor;
pub use progression::FoundationalProfileProgressionFrontDoor;
pub use set::FoundationalProfileSetFrontDoor;
pub use vocabulary::{
    FoundationalProfileFrontDoorConstructionDenial, FoundationalProfileFrontDoorFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProfilesFrontDoor;

impl ProfilesFrontDoor {
    pub const fn set(self) -> FoundationalProfileSetFrontDoor {
        FoundationalProfileSetFrontDoor::new()
    }

    pub const fn progression(self) -> FoundationalProfileProgressionFrontDoor {
        FoundationalProfileProgressionFrontDoor
    }

    pub const fn attach(self) -> FoundationalProfileAttachmentFrontDoor {
        FoundationalProfileAttachmentFrontDoor
    }

    pub const fn materialization(self) -> FoundationalProfileMaterializationFrontDoor {
        FoundationalProfileMaterializationFrontDoor
    }

    pub const fn certification(self) -> FoundationalProfileCertificationFrontDoor {
        FoundationalProfileCertificationFrontDoor
    }
}

pub fn profiles() -> ProfilesFrontDoor {
    ProfilesFrontDoor
}
