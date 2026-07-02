#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BatchAdmissionPlannerRouteFamily {
    BatchAdmissionRoute,
}

impl BatchAdmissionPlannerRouteFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BatchAdmissionRoute => "batch-admission-route",
        }
    }
}
