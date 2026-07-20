use worth_store_operations::{
    OperationalControlRecord, OperationalControlRecordKind, OperationalWorkflowKind,
};

use super::{S10OperationalScenarioKind, S10ScenarioCertificationDenial};

pub(super) fn require_scenario_owner_topology(
    kind: S10OperationalScenarioKind,
    records: &[OperationalControlRecord],
) -> Result<(), S10ScenarioCertificationDenial> {
    let observed = ScenarioOwnerTopology::observe(records);
    let expected = ScenarioOwnerTopology::for_kind(kind);
    if observed != expected {
        return Err(
            S10ScenarioCertificationDenial::ScenarioOwnerTopologyMismatch {
                expected: expected.bits(),
                observed: observed.bits(),
            },
        );
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ScenarioOwnerTopology {
    abandoned_backup: bool,
    rollback: bool,
    repair: bool,
    replica: bool,
    old_primary_rejoin: bool,
}

impl ScenarioOwnerTopology {
    const fn for_kind(kind: S10OperationalScenarioKind) -> Self {
        match kind {
            S10OperationalScenarioKind::BurningPrimary => Self {
                abandoned_backup: true,
                rollback: true,
                repair: false,
                replica: true,
                old_primary_rejoin: false,
            },
            S10OperationalScenarioKind::SplitBrainPromotion => Self {
                abandoned_backup: false,
                rollback: false,
                repair: true,
                replica: true,
                old_primary_rejoin: true,
            },
            S10OperationalScenarioKind::AuthorityRepairRollback => Self {
                abandoned_backup: false,
                rollback: true,
                repair: true,
                replica: false,
                old_primary_rejoin: false,
            },
        }
    }

    fn observe(records: &[OperationalControlRecord]) -> Self {
        Self {
            abandoned_backup: records.iter().any(|record| {
                matches!(
                    record.kind(),
                    OperationalControlRecordKind::BackupAbandoned { .. }
                )
            }),
            rollback: records.iter().any(|record| {
                matches!(
                    record.kind(),
                    OperationalControlRecordKind::WorkflowOpened {
                        workflow: OperationalWorkflowKind::Rollback
                    } | OperationalControlRecordKind::OperationalOwnerReceiptPersisted {
                        workflow: OperationalWorkflowKind::Rollback,
                        ..
                    } | OperationalControlRecordKind::AuthorizationConsumed {
                        operation_tag: 3,
                        ..
                    }
                )
            }),
            repair: records.iter().any(|record| {
                matches!(
                    record.kind(),
                    OperationalControlRecordKind::RepairExecutionOpened { .. }
                )
            }),
            replica: records.iter().any(|record| {
                matches!(
                    record.kind(),
                    OperationalControlRecordKind::ReplicaBootstrapTransferRecorded { .. }
                )
            }),
            old_primary_rejoin: records.iter().any(|record| {
                matches!(
                    record.kind(),
                    OperationalControlRecordKind::OldPrimaryRejoinPlanned { .. }
                )
            }),
        }
    }

    const fn bits(self) -> u8 {
        self.abandoned_backup as u8
            | (self.rollback as u8) << 1
            | (self.repair as u8) << 2
            | (self.replica as u8) << 3
            | (self.old_primary_rejoin as u8) << 4
    }
}
