#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedRetentionReport {
    classes: Box<[UiMountedRetentionClassReport]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedRetentionClassReport {
    class: crate::mounting::UiMountedRetentionClass,
    posture: UiMountedRetentionEvictionPosture,
    retained_items: usize,
    retained_structural_bytes: usize,
    active_leases: usize,
    lease_charged_structural_bytes: usize,
    evidence_budget: Option<crate::mounting::UiMountedRetentionClassBudget>,
    queue_budget: Option<UiMountedRetentionQueueBudget>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedRetentionQueueBudget {
    item_limit: usize,
    structural_byte_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedRetentionEvictionPosture {
    NonEvictable,
    LeaseProtected,
    EvictableUnlessLeased,
    AdmissionBounded,
    OmittedByPolicy,
    Reserved,
}

pub(crate) fn compose(
    mounted: crate::mounting::UiMountedFrameRetentionSnapshot,
    observations: crate::host_exchange::observation_report_validation::UiHostObservationRetentionSnapshot,
) -> UiMountedRetentionReport {
    let rows = vec![
        evidence_row(
            crate::mounting::UiMountedRetentionClass::Current,
            UiMountedRetentionEvictionPosture::NonEvictable,
            mounted.current,
            mounted.budget.current(),
        ),
        evidence_row(
            crate::mounting::UiMountedRetentionClass::InFlight,
            UiMountedRetentionEvictionPosture::NonEvictable,
            mounted.in_flight,
            mounted.budget.in_flight(),
        ),
        observation_basis_row(mounted, observations),
        evidence_row(
            crate::mounting::UiMountedRetentionClass::PredecessorInspection,
            UiMountedRetentionEvictionPosture::EvictableUnlessLeased,
            mounted.predecessor_inspection,
            mounted.budget.predecessor_inspection(),
        ),
        diagnostic_row(mounted),
        quarantine_row(observations),
        evidence_row(
            crate::mounting::UiMountedRetentionClass::FutureSnapshot,
            UiMountedRetentionEvictionPosture::Reserved,
            mounted.future_snapshot,
            mounted.budget.future_snapshot(),
        ),
    ];
    UiMountedRetentionReport {
        classes: rows.into_boxed_slice(),
    }
}

fn diagnostic_row(
    mounted: crate::mounting::UiMountedFrameRetentionSnapshot,
) -> UiMountedRetentionClassReport {
    let budget = mounted.budget.diagnostic();
    let posture = if budget.frame_limit() == 0 || budget.structural_byte_limit() == 0 {
        UiMountedRetentionEvictionPosture::OmittedByPolicy
    } else {
        UiMountedRetentionEvictionPosture::EvictableUnlessLeased
    };
    evidence_row(
        crate::mounting::UiMountedRetentionClass::Diagnostic,
        posture,
        mounted.diagnostic,
        budget,
    )
}

fn observation_basis_row(
    mounted: crate::mounting::UiMountedFrameRetentionSnapshot,
    observations: crate::host_exchange::observation_report_validation::UiHostObservationRetentionSnapshot,
) -> UiMountedRetentionClassReport {
    UiMountedRetentionClassReport {
        class: crate::mounting::UiMountedRetentionClass::ObservationBasis,
        posture: UiMountedRetentionEvictionPosture::LeaseProtected,
        retained_items: observations.retained_reports,
        retained_structural_bytes: observations.retained_bytes,
        active_leases: mounted.observation_basis.active_leases,
        lease_charged_structural_bytes: mounted.observation_basis.lease_charged_structural_bytes,
        evidence_budget: Some(mounted.budget.observation_basis()),
        queue_budget: Some(UiMountedRetentionQueueBudget::new(
            observations.retained_report_limit,
            observations.retained_byte_limit,
        )),
    }
}

fn quarantine_row(
    observations: crate::host_exchange::observation_report_validation::UiHostObservationRetentionSnapshot,
) -> UiMountedRetentionClassReport {
    UiMountedRetentionClassReport {
        class: crate::mounting::UiMountedRetentionClass::Quarantine,
        posture: UiMountedRetentionEvictionPosture::AdmissionBounded,
        retained_items: observations.quarantined_batches,
        retained_structural_bytes: observations.quarantined_bytes,
        active_leases: 0,
        lease_charged_structural_bytes: 0,
        evidence_budget: None,
        queue_budget: Some(UiMountedRetentionQueueBudget::new(
            observations.quarantine_count_limit,
            observations.quarantine_byte_limit,
        )),
    }
}

impl UiMountedRetentionReport {
    pub fn classes(&self) -> &[UiMountedRetentionClassReport] {
        &self.classes
    }

    pub fn class(
        &self,
        class: crate::mounting::UiMountedRetentionClass,
    ) -> &UiMountedRetentionClassReport {
        self.classes
            .iter()
            .find(|row| row.class == class)
            .expect("every declared retention class has exactly one report row")
    }
}

impl UiMountedRetentionClassReport {
    pub const fn class(&self) -> crate::mounting::UiMountedRetentionClass {
        self.class
    }

    pub const fn posture(&self) -> UiMountedRetentionEvictionPosture {
        self.posture
    }

    pub const fn retained_items(&self) -> usize {
        self.retained_items
    }

    pub const fn retained_structural_bytes(&self) -> usize {
        self.retained_structural_bytes
    }

    pub const fn active_leases(&self) -> usize {
        self.active_leases
    }

    pub const fn lease_charged_structural_bytes(&self) -> usize {
        self.lease_charged_structural_bytes
    }

    pub const fn evidence_budget(&self) -> Option<crate::mounting::UiMountedRetentionClassBudget> {
        self.evidence_budget
    }

    pub const fn queue_budget(&self) -> Option<UiMountedRetentionQueueBudget> {
        self.queue_budget
    }
}

impl UiMountedRetentionQueueBudget {
    const fn new(item_limit: usize, structural_byte_limit: usize) -> Self {
        Self {
            item_limit,
            structural_byte_limit,
        }
    }

    pub const fn item_limit(self) -> usize {
        self.item_limit
    }

    pub const fn structural_byte_limit(self) -> usize {
        self.structural_byte_limit
    }
}

fn evidence_row(
    class: crate::mounting::UiMountedRetentionClass,
    posture: UiMountedRetentionEvictionPosture,
    usage: crate::mounting::UiMountedRetentionUsageSnapshot,
    budget: crate::mounting::UiMountedRetentionClassBudget,
) -> UiMountedRetentionClassReport {
    UiMountedRetentionClassReport {
        class,
        posture,
        retained_items: usage.retained_items,
        retained_structural_bytes: usage.retained_structural_bytes,
        active_leases: usage.active_leases,
        lease_charged_structural_bytes: usage.lease_charged_structural_bytes,
        evidence_budget: Some(budget),
        queue_budget: None,
    }
}
