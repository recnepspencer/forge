use crate::capability::MosaicSizingContractId;

use super::{
    MosaicMeasurementAuthority, MosaicOverflowBehavior, MosaicParentGrowthBehavior,
    MosaicResizePermission, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, RawLayoutMeasurementForDiagnostics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MosaicSizingContractDescriptor {
    id: MosaicSizingContractId,
    kind: MosaicSizingKind,
    named_measurement: Option<NamedMeasurementDefinition>,
    measurement_authority: Option<MosaicMeasurementAuthority>,
    resize_permission: Option<MosaicResizePermission>,
    persistence: Option<MosaicSizingPersistence>,
    overflow_behavior: Option<MosaicOverflowBehavior>,
    parent_growth_behavior: Option<MosaicParentGrowthBehavior>,
    viewport_constraint: Option<MosaicViewportConstraint>,
    raw_measurements_for_diagnostics: Vec<RawLayoutMeasurementForDiagnostics>,
    label: Option<String>,
}

impl MosaicSizingContractDescriptor {
    pub fn new(id: MosaicSizingContractId, kind: MosaicSizingKind) -> Self {
        Self {
            id,
            kind,
            named_measurement: None,
            measurement_authority: None,
            resize_permission: None,
            persistence: None,
            overflow_behavior: None,
            parent_growth_behavior: None,
            viewport_constraint: None,
            raw_measurements_for_diagnostics: Vec::new(),
            label: None,
        }
    }

    pub fn with_named_measurement(mut self, measurement: NamedMeasurementDefinition) -> Self {
        self.named_measurement = Some(measurement);
        self
    }

    pub fn with_measurement_authority(mut self, authority: MosaicMeasurementAuthority) -> Self {
        self.measurement_authority = Some(authority);
        self
    }

    pub fn with_resize_permission(mut self, permission: MosaicResizePermission) -> Self {
        self.resize_permission = Some(permission);
        self
    }

    pub fn with_persistence(mut self, persistence: MosaicSizingPersistence) -> Self {
        self.persistence = Some(persistence);
        self
    }

    pub fn with_overflow_behavior(mut self, behavior: MosaicOverflowBehavior) -> Self {
        self.overflow_behavior = Some(behavior);
        self
    }

    pub fn with_parent_growth_behavior(mut self, behavior: MosaicParentGrowthBehavior) -> Self {
        self.parent_growth_behavior = Some(behavior);
        self
    }

    pub fn with_viewport_constraint(mut self, constraint: MosaicViewportConstraint) -> Self {
        self.viewport_constraint = Some(constraint);
        self
    }

    pub fn with_raw_measurement_for_diagnostics(
        mut self,
        measurement: RawLayoutMeasurementForDiagnostics,
    ) -> Self {
        self.raw_measurements_for_diagnostics.push(measurement);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(&self) -> &MosaicSizingContractId {
        &self.id
    }

    pub fn kind(&self) -> &MosaicSizingKind {
        &self.kind
    }

    pub fn named_measurement(&self) -> Option<&NamedMeasurementDefinition> {
        self.named_measurement.as_ref()
    }

    pub fn measurement_authority(&self) -> Option<&MosaicMeasurementAuthority> {
        self.measurement_authority.as_ref()
    }

    pub fn resize_permission(&self) -> Option<&MosaicResizePermission> {
        self.resize_permission.as_ref()
    }

    pub fn persistence(&self) -> Option<&MosaicSizingPersistence> {
        self.persistence.as_ref()
    }

    pub fn overflow_behavior(&self) -> Option<&MosaicOverflowBehavior> {
        self.overflow_behavior.as_ref()
    }

    pub fn parent_growth_behavior(&self) -> Option<&MosaicParentGrowthBehavior> {
        self.parent_growth_behavior.as_ref()
    }

    pub fn viewport_constraint(&self) -> Option<&MosaicViewportConstraint> {
        self.viewport_constraint.as_ref()
    }

    pub fn raw_measurements_for_diagnostics(&self) -> &[RawLayoutMeasurementForDiagnostics] {
        &self.raw_measurements_for_diagnostics
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
