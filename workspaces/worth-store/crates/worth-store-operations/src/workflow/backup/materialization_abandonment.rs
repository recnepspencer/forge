use worth_store_physical_backend::{
    PhysicalBackupMaterializationAbandonment, PhysicalBackupMaterializationAbandonmentDenial,
};
use worth_store_physical_isolation::{
    BackupCutAbandonmentReceipt, BackupReachabilityLeaseRegistry,
};

use crate::OperationalControlStorePort;

use super::admitted_online_backup::{record_durable_abandonment, BackupAbandonmentFailure};
use super::materialization_session::{BackupMaterializationSession, BackupPublicationSession};

#[derive(Debug)]
pub struct BackupMaterializationAbandonment {
    cut: BackupCutAbandonmentReceipt,
    physical_cleanup: Result<
        PhysicalBackupMaterializationAbandonment,
        PhysicalBackupMaterializationAbandonmentDenial,
    >,
}

pub enum BackupMaterializationAbandonmentRetry<'a> {
    Materialization(BackupMaterializationSession<'a>),
    Publication(BackupPublicationSession<'a>),
}

pub struct BackupMaterializationAbandonmentDenial<'a> {
    retry: BackupMaterializationAbandonmentRetry<'a>,
    source: BackupAbandonmentFailure,
}

impl std::fmt::Debug for BackupMaterializationAbandonmentRetry<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Materialization(_) => "Materialization(<retryable session>)",
            Self::Publication(_) => "Publication(<retryable session>)",
        })
    }
}

impl std::fmt::Debug for BackupMaterializationAbandonmentDenial<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BackupMaterializationAbandonmentDenial")
            .field("retry", &self.retry)
            .field("source", &self.source)
            .finish()
    }
}

impl BackupMaterializationAbandonment {
    pub const fn cut_receipt(&self) -> &BackupCutAbandonmentReceipt {
        &self.cut
    }

    pub fn physical_cleanup(
        &self,
    ) -> Result<
        &PhysicalBackupMaterializationAbandonment,
        &PhysicalBackupMaterializationAbandonmentDenial,
    > {
        self.physical_cleanup.as_ref()
    }

    pub fn into_physical_cleanup(
        self,
    ) -> Result<
        PhysicalBackupMaterializationAbandonment,
        PhysicalBackupMaterializationAbandonmentDenial,
    > {
        self.physical_cleanup
    }
}

impl<'a> BackupMaterializationAbandonmentDenial<'a> {
    pub fn into_retry(
        self,
    ) -> (
        BackupMaterializationAbandonmentRetry<'a>,
        BackupAbandonmentFailure,
    ) {
        (self.retry, self.source)
    }
}

impl<'a> BackupMaterializationSession<'a> {
    pub fn abandon(
        self,
        reason: impl Into<String>,
        control: &impl OperationalControlStorePort,
        leases: &BackupReachabilityLeaseRegistry,
    ) -> Result<BackupMaterializationAbandonment, BackupMaterializationAbandonmentDenial<'a>> {
        let reason = reason.into();
        let released = match record_durable_abandonment(
            &self.operation_id,
            &self.cut,
            &reason,
            control,
            leases,
        ) {
            Ok(released) => released,
            Err(source) => {
                return Err(BackupMaterializationAbandonmentDenial {
                    retry: BackupMaterializationAbandonmentRetry::Materialization(self),
                    source,
                });
            }
        };
        let Self {
            operation_id,
            cut,
            manifest,
            physical,
            control,
            format,
        } = self;
        let prepared =
            match worth_store_physical_isolation::prepare_backup_cut_abandonment(cut, released) {
                Ok(prepared) => prepared,
                Err(mismatch) => {
                    let (cut, released) = mismatch.into_parts();
                    return Err(BackupMaterializationAbandonmentDenial {
                        retry: BackupMaterializationAbandonmentRetry::Materialization(Self {
                            operation_id,
                            cut,
                            manifest,
                            physical,
                            control,
                            format,
                        }),
                        source: BackupAbandonmentFailure::ReleasedCutMismatch(released),
                    });
                }
            };
        Ok(BackupMaterializationAbandonment {
            cut: worth_store_physical_isolation::abandon_backup_cut(prepared, reason),
            physical_cleanup: physical.abandon(),
        })
    }
}

impl<'a> BackupPublicationSession<'a> {
    pub fn abandon(
        self,
        reason: impl Into<String>,
        control: &impl OperationalControlStorePort,
        leases: &BackupReachabilityLeaseRegistry,
    ) -> Result<BackupMaterializationAbandonment, BackupMaterializationAbandonmentDenial<'a>> {
        let reason = reason.into();
        let released = match record_durable_abandonment(
            &self.operation_id,
            &self.cut,
            &reason,
            control,
            leases,
        ) {
            Ok(released) => released,
            Err(source) => {
                return Err(BackupMaterializationAbandonmentDenial {
                    retry: BackupMaterializationAbandonmentRetry::Publication(self),
                    source,
                });
            }
        };
        let Self {
            operation_id,
            cut,
            physical,
            control,
            format,
        } = self;
        let prepared =
            match worth_store_physical_isolation::prepare_backup_cut_abandonment(cut, released) {
                Ok(prepared) => prepared,
                Err(mismatch) => {
                    let (cut, released) = mismatch.into_parts();
                    return Err(BackupMaterializationAbandonmentDenial {
                        retry: BackupMaterializationAbandonmentRetry::Publication(Self {
                            operation_id,
                            cut,
                            physical,
                            control,
                            format,
                        }),
                        source: BackupAbandonmentFailure::ReleasedCutMismatch(released),
                    });
                }
            };
        Ok(BackupMaterializationAbandonment {
            cut: worth_store_physical_isolation::abandon_backup_cut(prepared, reason),
            physical_cleanup: physical.abandon(),
        })
    }
}
