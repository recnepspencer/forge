use super::super::super::{PhysicalWorkOperationFamily, ResourceAdmittedPhysicalWork};
use super::types::{
    require_family, PhysicalExecutorCommand, PhysicalExecutorCommandDenial,
    PhysicalWalReclamationExecutorCommand,
};

impl PhysicalExecutorCommand {
    pub(in crate::physical_runtime) fn wal_reclamation(
        work: ResourceAdmittedPhysicalWork,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::WalReclamation)?;
        work.intent()
            .scope()
            .wal_reclamation_target()
            .ok_or(PhysicalExecutorCommandDenial::WalReclamationCommandRequiresWalScope)?;
        Ok(Self::WalReclamation(
            PhysicalWalReclamationExecutorCommand { work },
        ))
    }
}
