use super::{WorthQueryArtifactProtocolVersion, WorthQueryArtifactSchemaVersion};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactRetirementRule {
    Active,
    Retired,
    RetiredThroughSchema(WorthQueryArtifactSchemaVersion),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactDowngradePosture {
    Denied,
    SupportedBy { family: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactCompatibilityWindow {
    minimum_schema: WorthQueryArtifactSchemaVersion,
    maximum_schema: WorthQueryArtifactSchemaVersion,
    minimum_protocol: WorthQueryArtifactProtocolVersion,
    maximum_protocol: WorthQueryArtifactProtocolVersion,
}

impl WorthQueryArtifactCompatibilityWindow {
    pub const fn new(
        minimum_schema: WorthQueryArtifactSchemaVersion,
        maximum_schema: WorthQueryArtifactSchemaVersion,
        minimum_protocol: WorthQueryArtifactProtocolVersion,
        maximum_protocol: WorthQueryArtifactProtocolVersion,
    ) -> Self {
        Self {
            minimum_schema,
            maximum_schema,
            minimum_protocol,
            maximum_protocol,
        }
    }

    pub const fn minimum_schema(self) -> WorthQueryArtifactSchemaVersion {
        self.minimum_schema
    }

    pub const fn maximum_schema(self) -> WorthQueryArtifactSchemaVersion {
        self.maximum_schema
    }

    pub const fn minimum_protocol(self) -> WorthQueryArtifactProtocolVersion {
        self.minimum_protocol
    }

    pub const fn maximum_protocol(self) -> WorthQueryArtifactProtocolVersion {
        self.maximum_protocol
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactCompatibilityContract {
    window: WorthQueryArtifactCompatibilityWindow,
    migration_owners: Vec<String>,
    retirement: WorthQueryArtifactRetirementRule,
    downgrade: WorthQueryArtifactDowngradePosture,
}

impl WorthQueryArtifactCompatibilityContract {
    pub fn new(
        window: WorthQueryArtifactCompatibilityWindow,
        migration_owner: impl Into<String>,
        retirement: WorthQueryArtifactRetirementRule,
        downgrade: WorthQueryArtifactDowngradePosture,
    ) -> Self {
        Self {
            window,
            migration_owners: vec![migration_owner.into()],
            retirement,
            downgrade,
        }
    }

    pub fn migration_owner(mut self, owner: impl Into<String>) -> Self {
        self.migration_owners.push(owner.into());
        self.migration_owners.sort();
        self.migration_owners.dedup();
        self
    }

    pub const fn minimum_schema(&self) -> WorthQueryArtifactSchemaVersion {
        self.window.minimum_schema()
    }

    pub const fn maximum_schema(&self) -> WorthQueryArtifactSchemaVersion {
        self.window.maximum_schema()
    }

    pub const fn minimum_protocol(&self) -> WorthQueryArtifactProtocolVersion {
        self.window.minimum_protocol()
    }

    pub const fn maximum_protocol(&self) -> WorthQueryArtifactProtocolVersion {
        self.window.maximum_protocol()
    }

    pub fn migration_owners(&self) -> &[String] {
        &self.migration_owners
    }

    pub fn retirement(&self) -> &WorthQueryArtifactRetirementRule {
        &self.retirement
    }

    pub fn downgrade(&self) -> &WorthQueryArtifactDowngradePosture {
        &self.downgrade
    }
}
