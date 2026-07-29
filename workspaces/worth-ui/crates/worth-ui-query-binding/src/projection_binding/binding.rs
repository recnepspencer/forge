use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use crate::declaration::{UiCollectionSchemaRequirement, UiScalarSchemaRequirement};

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionBinding {
    query_binding_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionBinding {
    pub fn query_binding_identity_for_reporting(&self) -> &str {
        self.query_binding_identity
            .terminal_projection_for_reporting()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionBinding {
    core: UiProjectionBinding,
    requirement: UiScalarSchemaRequirement,
}

impl UiScalarProjectionBinding {
    pub fn core(&self) -> &UiProjectionBinding {
        &self.core
    }

    pub fn requirement(&self) -> &UiScalarSchemaRequirement {
        &self.requirement
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionBinding {
    core: UiProjectionBinding,
    requirement: UiCollectionSchemaRequirement,
}

impl UiCollectionProjectionBinding {
    pub fn core(&self) -> &UiProjectionBinding {
        &self.core
    }

    pub fn requirement(&self) -> &UiCollectionSchemaRequirement {
        &self.requirement
    }
}
