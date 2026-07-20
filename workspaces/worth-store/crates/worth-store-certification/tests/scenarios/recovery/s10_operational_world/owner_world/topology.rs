use worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioKind;

#[derive(Clone, Copy)]
pub(super) struct ScenarioOwnerTopology {
    pub(super) abandoned_backup: bool,
    pub(super) rollback: bool,
    pub(super) repair: bool,
    pub(super) replica: bool,
    pub(super) old_primary_rejoin: bool,
}

impl ScenarioOwnerTopology {
    pub(super) const fn for_kind(kind: S10OperationalScenarioKind) -> Self {
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
}
