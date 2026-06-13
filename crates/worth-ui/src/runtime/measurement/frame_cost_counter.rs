use forge_foundational::FoundationalPerformanceWorkClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorthUiCounterValueKind {
    CountedWork,
    ElapsedTimeAuxiliary,
    UnattributedWorkBucket,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorthUiFrameCostCounter {
    name: &'static str,
    value: u64,
    value_kind: WorthUiCounterValueKind,
    work_class: FoundationalPerformanceWorkClass,
}

impl WorthUiFrameCostCounter {
    pub fn count(name: &'static str, value: u64) -> Self {
        Self {
            name,
            value,
            value_kind: WorthUiCounterValueKind::CountedWork,
            work_class: FoundationalPerformanceWorkClass::ValidationPlanning,
        }
    }

    pub fn authoritative_mutation_count(name: &'static str, value: u64) -> Self {
        Self {
            name,
            value,
            value_kind: WorthUiCounterValueKind::CountedWork,
            work_class: FoundationalPerformanceWorkClass::AuthoritativeMutation,
        }
    }

    pub fn elapsed_time_auxiliary(name: &'static str, micros: u64) -> Self {
        Self {
            name,
            value: micros,
            value_kind: WorthUiCounterValueKind::ElapsedTimeAuxiliary,
            work_class: FoundationalPerformanceWorkClass::SupportReportAssembly,
        }
    }

    #[cfg(test)]
    pub(crate) fn unattributed_work_bucket(value: u64) -> Self {
        Self {
            name: "unattributed.work",
            value,
            value_kind: WorthUiCounterValueKind::UnattributedWorkBucket,
            work_class: FoundationalPerformanceWorkClass::ForensicParity,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn value_kind(&self) -> WorthUiCounterValueKind {
        self.value_kind
    }

    pub fn work_class(&self) -> FoundationalPerformanceWorkClass {
        self.work_class
    }

    pub fn certifies_execution_work(&self) -> bool {
        self.value_kind == WorthUiCounterValueKind::CountedWork
    }
}
