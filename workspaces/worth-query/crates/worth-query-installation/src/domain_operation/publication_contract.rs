#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationPublicationContract {
    NotRequired,
    DerivedProjection {
        projection_role: WorthQueryOperationProjectionRole,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryOperationProjectionRole(String);

impl WorthQueryOperationProjectionRole {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-operation-projection-role");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationProjectionConsumptionContract {
    NotRequired,
    QueryReadAuthority,
}
