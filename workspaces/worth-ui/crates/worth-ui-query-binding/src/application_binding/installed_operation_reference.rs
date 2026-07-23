use std::fmt;

use crate::{
    WorthUiInstalledQueryDomain, WorthUiQueryViewDefinition, WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
};

/// Exact installed operation selected while an authored UI view enters the
/// prepared application generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiInstalledSnapshotOperationReference {
    pub(crate) operation: WorthUiSnapshotMeasurement,
    pub(crate) family: WorthUiSnapshotMeasurementFamily,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthUiInstalledOperationReference {
    SnapshotMeasurement(WorthUiInstalledSnapshotOperationReference),
}

/// Compact binding-owned reference retained by Worth UI plan lowering.
///
/// The UI definition remains inspectable. Query installation authority and
/// the selected operation marker remain sealed, so later attempts consume the
/// operation chosen during preparation instead of rediscovering it from UI
/// lifecycle metadata.
#[derive(Clone)]
pub struct WorthUiInstalledQueryBindingReference {
    installed_domain: WorthUiInstalledQueryDomain,
    installed_operation: WorthUiInstalledOperationReference,
    definition: WorthUiQueryViewDefinition,
}

impl WorthUiInstalledQueryBindingReference {
    pub(crate) fn new(
        installed_domain: WorthUiInstalledQueryDomain,
        definition: WorthUiQueryViewDefinition,
    ) -> Self {
        let installed_operation = WorthUiInstalledOperationReference::SnapshotMeasurement(
            WorthUiInstalledSnapshotOperationReference {
                operation: WorthUiSnapshotMeasurement,
                family: WorthUiSnapshotMeasurementFamily,
            },
        );
        Self {
            installed_domain,
            installed_operation,
            definition,
        }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn installation_is_current(&self) -> bool {
        self.installed_domain.handle().installation_is_current()
    }

    pub(crate) fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        &self.installed_domain
    }

    pub(crate) fn snapshot_operation(&self) -> WorthUiInstalledSnapshotOperationReference {
        match self.installed_operation {
            WorthUiInstalledOperationReference::SnapshotMeasurement(operation) => operation,
        }
    }
}

impl fmt::Debug for WorthUiInstalledQueryBindingReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiInstalledQueryBindingReference")
            .field("definition", &self.definition)
            .field("installed_operation", &self.installed_operation)
            .field("installed_domain_authority", &"sealed")
            .finish()
    }
}

impl PartialEq for WorthUiInstalledQueryBindingReference {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
            && self.installed_operation == other.installed_operation
            && self
                .installed_domain
                .shares_authority_with(&other.installed_domain)
    }
}

impl Eq for WorthUiInstalledQueryBindingReference {}
