#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshAdmissionClass {
    WidthOverflow,
}

impl RefreshAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WidthOverflow => "width_overflow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshAdmissionMatrix {
    admitted_classes: Vec<RefreshAdmissionClass>,
}

impl RefreshAdmissionMatrix {
    pub fn admitted_classes(&self) -> &[RefreshAdmissionClass] {
        &self.admitted_classes
    }

    pub fn admits(&self, class: &RefreshAdmissionClass) -> bool {
        self.admitted_classes.contains(class)
    }

    pub(in crate::live) fn detail_family() -> Self {
        Self {
            admitted_classes: Vec::new(),
        }
    }

    pub(in crate::live) fn ordered_collection_family() -> Self {
        Self {
            admitted_classes: Vec::new(),
        }
    }

    pub(in crate::live) fn bounded_materialization_family() -> Self {
        Self {
            admitted_classes: vec![RefreshAdmissionClass::WidthOverflow],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshFallback {
    pub(in crate::live) admission_class: RefreshAdmissionClass,
    pub(in crate::live) cost_class: crate::live_performance::RefreshCostClass,
    pub(in crate::live) admission_status: crate::live_performance::RefreshAdmissionStatus,
}

impl RefreshFallback {
    pub fn admission_class(&self) -> &RefreshAdmissionClass {
        &self.admission_class
    }

    pub fn cost_class(&self) -> &crate::live_performance::RefreshCostClass {
        &self.cost_class
    }

    pub fn admission_status(&self) -> &crate::live_performance::RefreshAdmissionStatus {
        &self.admission_status
    }
}
