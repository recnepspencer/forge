use worth_foundational::facade::FoundationalPerformanceCounterName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryStructuralCounterContract {
    byte_counter: FoundationalPerformanceCounterName,
    element_counter: FoundationalPerformanceCounterName,
    structural_work_counter: FoundationalPerformanceCounterName,
}

impl WorthQueryStructuralCounterContract {
    pub fn new(
        byte_counter: FoundationalPerformanceCounterName,
        element_counter: FoundationalPerformanceCounterName,
        structural_work_counter: FoundationalPerformanceCounterName,
    ) -> Self {
        Self {
            byte_counter,
            element_counter,
            structural_work_counter,
        }
    }

    pub fn byte_counter(&self) -> &FoundationalPerformanceCounterName {
        &self.byte_counter
    }

    pub fn element_counter(&self) -> &FoundationalPerformanceCounterName {
        &self.element_counter
    }

    pub fn structural_work_counter(&self) -> &FoundationalPerformanceCounterName {
        &self.structural_work_counter
    }

    pub(crate) fn names_are_distinct(&self) -> bool {
        self.byte_counter != self.element_counter
            && self.byte_counter != self.structural_work_counter
            && self.element_counter != self.structural_work_counter
    }
}
