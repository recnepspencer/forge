use super::{BaselineLsmCounterObservation, BaselineLsmLookupAdmission};
use forge_store_wal::BlobWalRecordIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineLsmLookupDisposition {
    Memtable,
    SortedRun,
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupAbsence {
    plan_binding: crate::planning::AccessPlanIdentity,
    request_identity: crate::keyspace::AdmittedPhysicalAccessIdentity,
    probe_sequence: u64,
    current_materialization: crate::CurrentLayoutMaterialization,
    tombstone_blocks_older: bool,
}

impl BaselineLsmLookupAbsence {
    fn issue(
        admission: &BaselineLsmLookupAdmission,
        probe_sequence: u64,
        tombstone_blocks_older: bool,
    ) -> Self {
        Self {
            plan_binding: admission.plan_binding().clone(),
            request_identity: admission.request_identity(),
            probe_sequence,
            current_materialization: admission.current_materialization().clone(),
            tombstone_blocks_older,
        }
    }

    pub const fn plan_binding(&self) -> &crate::planning::AccessPlanIdentity {
        &self.plan_binding
    }
    pub const fn request_identity(&self) -> crate::keyspace::AdmittedPhysicalAccessIdentity {
        self.request_identity
    }
    pub const fn probe_sequence(&self) -> u64 {
        self.probe_sequence
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
    pub const fn tombstone_blocks_older(&self) -> bool {
        self.tombstone_blocks_older
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupExecution {
    admission: BaselineLsmLookupAdmission,
    probe_sequence: u64,
    counters: BaselineLsmCounterObservation,
    observation: BaselineLsmLookupObservation,
}

macro_rules! define_lsm_lookup_cases {
    ($( $variant:ident($payload:ty) => $name:literal ),+ $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum BaselineLsmLookupObservation {
            $( $variant($payload), )+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct BaselineLsmLookupCaseId(&'static str);

        impl BaselineLsmLookupCaseId {
            pub const fn name(self) -> &'static str {
                self.0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum BaselineLsmLookupView<'a> {
            $( $variant(&'a $payload), )+
        }

        impl BaselineLsmLookupObservation {
            const fn case_id(&self) -> BaselineLsmLookupCaseId {
                match self {
                    $( Self::$variant(_) => BaselineLsmLookupCaseId($name), )+
                }
            }
        }

        pub fn baseline_lsm_lookup_cases() -> impl Iterator<Item = BaselineLsmLookupCaseId> {
            [$( BaselineLsmLookupCaseId($name), )+].into_iter()
        }
    };
}

define_lsm_lookup_cases!(
    Memtable(BlobWalRecordIdentity) => "layout.lsm_lookup.result.memtable",
    SortedRun(BlobWalRecordIdentity) => "layout.lsm_lookup.result.sorted_run",
    Absent(BaselineLsmLookupAbsence) => "layout.lsm_lookup.result.absent",
);

impl BaselineLsmLookupExecution {
    pub(super) fn new(
        admission: BaselineLsmLookupAdmission,
        probe_sequence: u64,
        disposition: BaselineLsmLookupDisposition,
        memtable_record: BlobWalRecordIdentity,
        sorted_run_record: BlobWalRecordIdentity,
        tombstone_blocks_older: bool,
    ) -> Self {
        let observation = match disposition {
            BaselineLsmLookupDisposition::NotFound => BaselineLsmLookupObservation::Absent(
                BaselineLsmLookupAbsence::issue(&admission, probe_sequence, tombstone_blocks_older),
            ),
            BaselineLsmLookupDisposition::Memtable => {
                BaselineLsmLookupObservation::Memtable(memtable_record)
            }
            BaselineLsmLookupDisposition::SortedRun => {
                BaselineLsmLookupObservation::SortedRun(sorted_run_record)
            }
        };
        Self {
            admission,
            probe_sequence,
            counters: BaselineLsmCounterObservation::lookup(),
            observation,
        }
    }

    pub const fn plan_binding(&self) -> &crate::planning::AccessPlanIdentity {
        self.admission.plan_binding()
    }
    pub const fn request_identity(&self) -> crate::keyspace::AdmittedPhysicalAccessIdentity {
        self.admission.request_identity()
    }
    pub const fn probe_sequence(&self) -> u64 {
        self.probe_sequence
    }
    pub const fn disposition(&self) -> BaselineLsmLookupDisposition {
        match &self.observation {
            BaselineLsmLookupObservation::Memtable(_) => BaselineLsmLookupDisposition::Memtable,
            BaselineLsmLookupObservation::SortedRun(_) => BaselineLsmLookupDisposition::SortedRun,
            BaselineLsmLookupObservation::Absent(_) => BaselineLsmLookupDisposition::NotFound,
        }
    }
    pub const fn case_id(&self) -> BaselineLsmLookupCaseId {
        self.observation.case_id()
    }
    pub const fn probe_visible_in_newer_run(&self) -> bool {
        matches!(self.observation, BaselineLsmLookupObservation::Memtable(_))
    }
    pub const fn probe_visible_in_older_run(&self) -> bool {
        !matches!(self.observation, BaselineLsmLookupObservation::Memtable(_))
    }
    pub const fn tombstone_blocks_older(&self) -> bool {
        match &self.observation {
            BaselineLsmLookupObservation::Absent(absence) => absence.tombstone_blocks_older(),
            _ => false,
        }
    }
    pub const fn counters(&self) -> BaselineLsmCounterObservation {
        self.counters
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        self.admission.current_materialization()
    }
    pub const fn view(&self) -> BaselineLsmLookupView<'_> {
        match &self.observation {
            BaselineLsmLookupObservation::Memtable(record) => {
                BaselineLsmLookupView::Memtable(record)
            }
            BaselineLsmLookupObservation::SortedRun(record) => {
                BaselineLsmLookupView::SortedRun(record)
            }
            BaselineLsmLookupObservation::Absent(absence) => BaselineLsmLookupView::Absent(absence),
        }
    }
}
