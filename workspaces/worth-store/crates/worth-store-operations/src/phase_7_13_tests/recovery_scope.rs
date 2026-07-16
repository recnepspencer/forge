pub(crate) fn recovery_security_scope(
    backup: &crate::ProductionRestoreAdmissibleBackupBundle,
) -> crate::OperationalSecurityScope {
    crate::OperationalSecurityScope::from_admission(backup.custody().custody_receipt())
}
