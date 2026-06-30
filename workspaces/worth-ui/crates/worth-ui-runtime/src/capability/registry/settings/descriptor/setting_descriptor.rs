use crate::capability::SettingId;

use super::{
    ArbitraryKeyValueSettingBag, SettingDefaultPosture, SettingEditorHint, SettingMigrationPosture,
    SettingOwnershipMetadata, SettingScope, SettingValidationPosture, SettingValueSchema,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingDescriptor {
    id: SettingId,
    scope: Option<SettingScope>,
    value_schema: Option<SettingValueSchema>,
    default_posture: Option<SettingDefaultPosture>,
    validation_posture: Option<SettingValidationPosture>,
    migration_posture: Option<SettingMigrationPosture>,
    editor_hint: Option<SettingEditorHint>,
    ownership_metadata: Option<SettingOwnershipMetadata>,
    arbitrary_key_value_bag: Option<ArbitraryKeyValueSettingBag>,
}

impl SettingDescriptor {
    pub fn typed(id: SettingId, scope: SettingScope, value_schema: SettingValueSchema) -> Self {
        Self {
            id,
            scope: Some(scope),
            value_schema: Some(value_schema),
            default_posture: None,
            validation_posture: None,
            migration_posture: None,
            editor_hint: None,
            ownership_metadata: None,
            arbitrary_key_value_bag: None,
        }
    }

    pub fn missing_scope_for_diagnostics(id: SettingId, value_schema: SettingValueSchema) -> Self {
        Self {
            id,
            scope: None,
            value_schema: Some(value_schema),
            default_posture: None,
            validation_posture: None,
            migration_posture: None,
            editor_hint: None,
            ownership_metadata: None,
            arbitrary_key_value_bag: None,
        }
    }

    pub fn missing_value_schema_for_diagnostics(id: SettingId, scope: SettingScope) -> Self {
        Self {
            id,
            scope: Some(scope),
            value_schema: None,
            default_posture: None,
            validation_posture: None,
            migration_posture: None,
            editor_hint: None,
            ownership_metadata: None,
            arbitrary_key_value_bag: None,
        }
    }

    pub fn arbitrary_key_value_bag_for_diagnostics(
        id: SettingId,
        bag: ArbitraryKeyValueSettingBag,
    ) -> Self {
        Self {
            id,
            scope: None,
            value_schema: None,
            default_posture: None,
            validation_posture: None,
            migration_posture: None,
            editor_hint: None,
            ownership_metadata: None,
            arbitrary_key_value_bag: Some(bag),
        }
    }

    pub fn with_default_posture(mut self, posture: SettingDefaultPosture) -> Self {
        self.default_posture = Some(posture);
        self
    }

    pub fn with_validation_posture(mut self, posture: SettingValidationPosture) -> Self {
        self.validation_posture = Some(posture);
        self
    }

    pub fn with_migration_posture(mut self, posture: SettingMigrationPosture) -> Self {
        self.migration_posture = Some(posture);
        self
    }

    pub fn with_editor_hint(mut self, hint: SettingEditorHint) -> Self {
        self.editor_hint = Some(hint);
        self
    }

    pub fn with_ownership_metadata(mut self, metadata: SettingOwnershipMetadata) -> Self {
        self.ownership_metadata = Some(metadata);
        self
    }

    pub fn id(&self) -> &SettingId {
        &self.id
    }

    pub fn scope(&self) -> Option<&SettingScope> {
        self.scope.as_ref()
    }

    pub fn value_schema(&self) -> Option<&SettingValueSchema> {
        self.value_schema.as_ref()
    }

    pub fn default_posture(&self) -> Option<&SettingDefaultPosture> {
        self.default_posture.as_ref()
    }

    pub fn validation_posture(&self) -> Option<&SettingValidationPosture> {
        self.validation_posture.as_ref()
    }

    pub fn migration_posture(&self) -> Option<&SettingMigrationPosture> {
        self.migration_posture.as_ref()
    }

    pub fn editor_hint(&self) -> Option<&SettingEditorHint> {
        self.editor_hint.as_ref()
    }

    pub fn ownership_metadata(&self) -> Option<&SettingOwnershipMetadata> {
        self.ownership_metadata.as_ref()
    }

    pub(crate) fn has_arbitrary_key_value_bag(&self) -> bool {
        self.arbitrary_key_value_bag.is_some()
    }
}
