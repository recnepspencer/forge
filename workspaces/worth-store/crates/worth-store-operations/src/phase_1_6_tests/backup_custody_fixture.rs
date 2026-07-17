use worth_store_security::StoreKeyVersionPosture;

use crate::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupExportCustodyReadiness,
};

pub(crate) fn backup_custody(
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> BackupExportCustodyReadiness {
    let admission = BackupExportCustodyDeclaration::native(
        authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .expect("custody declaration")
    .admit_with_current_authority(authority)
    .expect("custody admission");
    BackupExportCustodyReadiness::from_admitted_custody(admission).expect("custody readiness")
}
