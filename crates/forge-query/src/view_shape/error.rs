#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeFailureClass {
    DeferredViewFamily,
    IncompatibleCanonicalFamily,
    FocusAspectRequired,
    GroupingAspectRequired,
    AdmissionInvariantRejected,
    ValidationRejected,
    PlanningInvariantRejected,
    PlanningRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeError {
    failure_class: ViewShapeFailureClass,
    message: String,
}

impl ViewShapeError {
    pub(crate) fn new(failure_class: ViewShapeFailureClass, message: impl Into<String>) -> Self {
        Self {
            failure_class,
            message: message.into(),
        }
    }

    pub fn failure_class(&self) -> &ViewShapeFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
