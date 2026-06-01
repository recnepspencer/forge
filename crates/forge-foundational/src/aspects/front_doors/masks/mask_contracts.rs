use crate::AspectMaskContract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectMaskContractFrontDoor;

impl AspectMaskContractFrontDoor {
    pub const fn scalar(self) -> AspectMaskContract {
        AspectMaskContract::scalar()
    }

    pub const fn struct_fields(self) -> AspectMaskContract {
        AspectMaskContract::struct_fields()
    }

    pub const fn opaque_diagnostic_only(self) -> AspectMaskContract {
        AspectMaskContract::opaque_diagnostic_only()
    }
}
