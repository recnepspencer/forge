use crate::PhysicalVocabularyError;

macro_rules! physical_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            pub fn from_raw(value: u64) -> Result<Self, PhysicalVocabularyError> {
                if value == 0 {
                    return Err(PhysicalVocabularyError::ZeroPhysicalIdentifier);
                }
                Ok(Self(value))
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

physical_id!(PhysicalSegmentId);
physical_id!(PhysicalPageId);
physical_id!(PhysicalExtentId);
physical_id!(PhysicalFrameId);
physical_id!(PhysicalGeneration);
physical_id!(PhysicalEpoch);
physical_id!(PhysicalRootReference);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRecordSlot(u16);

impl PhysicalRecordSlot {
    pub fn from_raw(value: u16) -> Result<Self, PhysicalVocabularyError> {
        if value == 0 {
            return Err(PhysicalVocabularyError::ZeroPhysicalIdentifier);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}
