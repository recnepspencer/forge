use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;

use super::{
    validate_planar_recovery_posture_basis, validate_planar_recovery_source_authority,
    PlanarRecoveryPostureDenial, PlanarRecoverySource, PlanarRecoverySourceKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryBlockerKind {
    ProjectionBasis,
    RetainedOrProjectionBasis,
    DirtyInput,
    UnsupportedPlanarClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoverySourcePosture {
    Denied,
    Dirty,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryAction {
    InspectProjectionBasis,
    InspectRetainedOrProjectionBasis,
    InspectTopologyAndInputCleanliness,
    ClassifyWithoutBoundedConversion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryTargetScope {
    ProjectionBasisInspection,
    RetainedProjectionBasisInspection,
    InputCleanlinessInspection,
    SupportReadiness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryTruthEffect {
    DoesNotChangePlanarTruth,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPostureBasis {
    source: PlanarRecoverySource,
    blocker_kind: PlanarRecoveryBlockerKind,
    source_posture: PlanarRecoverySourcePosture,
    recovery_action: PlanarRecoveryAction,
    target_scope: PlanarRecoveryTargetScope,
    truth_effect: PlanarRecoveryTruthEffect,
    retained_planar_facts: Option<RetainedPlanarFactsReceipt>,
    projection_consumed_facts: Option<ProjectionConsumedPlanarFactsReceipt>,
}

impl PlanarRecoveryPostureBasis {
    pub fn builder(source: PlanarRecoverySource) -> PlanarRecoveryPostureBuilder {
        PlanarRecoveryPostureBuilder::new(source)
    }

    pub(crate) fn from_builder(
        builder: PlanarRecoveryPostureBuilder,
    ) -> Result<Self, PlanarRecoveryPostureDenial> {
        validate_planar_recovery_source_authority(&builder.source)?;
        let classification = classify_recovery_posture_source(&builder.source);
        let basis = Self {
            source: builder.source,
            blocker_kind: classification.blocker_kind,
            source_posture: classification.source_posture,
            recovery_action: classification.recovery_action,
            target_scope: classification.target_scope,
            truth_effect: PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth,
            retained_planar_facts: builder.retained_planar_facts,
            projection_consumed_facts: builder.projection_consumed_facts,
        };
        validate_planar_recovery_posture_basis(&basis)?;
        Ok(basis)
    }

    pub fn source(&self) -> &PlanarRecoverySource {
        &self.source
    }

    pub fn blocker_kind(&self) -> PlanarRecoveryBlockerKind {
        self.blocker_kind
    }

    pub fn source_posture(&self) -> PlanarRecoverySourcePosture {
        self.source_posture
    }

    pub fn recovery_action(&self) -> PlanarRecoveryAction {
        self.recovery_action
    }

    pub fn target_scope(&self) -> PlanarRecoveryTargetScope {
        self.target_scope
    }

    pub fn truth_effect(&self) -> PlanarRecoveryTruthEffect {
        self.truth_effect
    }

    pub fn retained_planar_facts(&self) -> Option<&RetainedPlanarFactsReceipt> {
        self.retained_planar_facts.as_ref()
    }

    pub fn projection_consumed_facts(&self) -> Option<&ProjectionConsumedPlanarFactsReceipt> {
        self.projection_consumed_facts.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPostureBuilder {
    source: PlanarRecoverySource,
    retained_planar_facts: Option<RetainedPlanarFactsReceipt>,
    projection_consumed_facts: Option<ProjectionConsumedPlanarFactsReceipt>,
}

impl PlanarRecoveryPostureBuilder {
    fn new(source: PlanarRecoverySource) -> Self {
        Self {
            source,
            retained_planar_facts: None,
            projection_consumed_facts: None,
        }
    }

    pub fn retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.retained_planar_facts = Some(receipt);
        self
    }

    pub fn projection_consumed_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumed_facts = Some(receipt);
        self
    }

    pub fn build(self) -> Result<PlanarRecoveryPostureBasis, PlanarRecoveryPostureDenial> {
        PlanarRecoveryPostureBasis::from_builder(self)
    }
}

struct PlanarRecoveryPostureClassification {
    blocker_kind: PlanarRecoveryBlockerKind,
    source_posture: PlanarRecoverySourcePosture,
    recovery_action: PlanarRecoveryAction,
    target_scope: PlanarRecoveryTargetScope,
}

fn classify_recovery_posture_source(
    source: &PlanarRecoverySource,
) -> PlanarRecoveryPostureClassification {
    match source.kind() {
        PlanarRecoverySourceKind::ProjectionBasisDenial => PlanarRecoveryPostureClassification {
            blocker_kind: PlanarRecoveryBlockerKind::ProjectionBasis,
            source_posture: PlanarRecoverySourcePosture::Denied,
            recovery_action: PlanarRecoveryAction::InspectProjectionBasis,
            target_scope: PlanarRecoveryTargetScope::ProjectionBasisInspection,
        },
        PlanarRecoverySourceKind::RetainedOrProjectionBasisDenial => {
            PlanarRecoveryPostureClassification {
                blocker_kind: PlanarRecoveryBlockerKind::RetainedOrProjectionBasis,
                source_posture: PlanarRecoverySourcePosture::Denied,
                recovery_action: PlanarRecoveryAction::InspectRetainedOrProjectionBasis,
                target_scope: PlanarRecoveryTargetScope::RetainedProjectionBasisInspection,
            }
        }
        PlanarRecoverySourceKind::DirtyPlanarInput => PlanarRecoveryPostureClassification {
            blocker_kind: PlanarRecoveryBlockerKind::DirtyInput,
            source_posture: PlanarRecoverySourcePosture::Dirty,
            recovery_action: PlanarRecoveryAction::InspectTopologyAndInputCleanliness,
            target_scope: PlanarRecoveryTargetScope::InputCleanlinessInspection,
        },
        PlanarRecoverySourceKind::UnboundedPlanarClass => PlanarRecoveryPostureClassification {
            blocker_kind: PlanarRecoveryBlockerKind::UnsupportedPlanarClass,
            source_posture: PlanarRecoverySourcePosture::Unsupported,
            recovery_action: PlanarRecoveryAction::ClassifyWithoutBoundedConversion,
            target_scope: PlanarRecoveryTargetScope::SupportReadiness,
        },
        PlanarRecoverySourceKind::KernelSummary => unreachable!(
            "planar recovery source authority is validated before posture classification"
        ),
    }
}
