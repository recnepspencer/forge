#[cfg(test)]
use super::super::digest::digest_owned_parts;
#[cfg(test)]
use super::super::request::PrimitiveConstructionFamily;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoveryAction {
    CorrectRequestFamilyOrCounts,
    ReviseGeometryScaffold,
    EscalateRealizationConditioning,
    CorrectBirthAttachment,
    RetryTopologyExecution,
}

#[cfg(test)]
impl GeometryRecoveryAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CorrectRequestFamilyOrCounts => "correct_request_family_or_counts",
            Self::ReviseGeometryScaffold => "revise_geometry_scaffold",
            Self::EscalateRealizationConditioning => "escalate_realization_conditioning",
            Self::CorrectBirthAttachment => "correct_birth_attachment",
            Self::RetryTopologyExecution => "retry_topology_execution",
        }
    }
}

#[cfg(test)]
pub type PrimitiveConstructionRecoveryAction = GeometryRecoveryAction;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoverySourcePosture {
    RejectedConstructionOutcome,
}

#[cfg(test)]
impl GeometryRecoverySourcePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RejectedConstructionOutcome => "rejected_construction_outcome",
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRecoveryTargetScope {
    RequestFamilyOrCounts,
    GeometryScaffold,
    RealizationConditioning,
    BirthAttachment,
    TopologyExecution,
}

#[cfg(test)]
impl GeometryRecoveryTargetScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequestFamilyOrCounts => "request_family_or_counts",
            Self::GeometryScaffold => "geometry_scaffold",
            Self::RealizationConditioning => "realization_conditioning",
            Self::BirthAttachment => "birth_attachment",
            Self::TopologyExecution => "topology_execution",
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryRecoveryActionFactReceipt {
    recovery_action_kind: GeometryRecoveryAction,
    source_posture: GeometryRecoverySourcePosture,
    source_family: PrimitiveConstructionFamily,
    recovery_target_scope: GeometryRecoveryTargetScope,
    resulting_binding_identity: Option<String>,
    resulting_target_identity: Option<String>,
    fact_digest: String,
}

#[cfg(test)]
impl GeometryRecoveryActionFactReceipt {
    fn new(
        recovery_action_kind: GeometryRecoveryAction,
        source_family: PrimitiveConstructionFamily,
        recovery_target_scope: GeometryRecoveryTargetScope,
    ) -> Self {
        let resulting_binding_identity = None;
        let resulting_target_identity = None;
        let source_posture = GeometryRecoverySourcePosture::RejectedConstructionOutcome;
        let fact_digest = digest_owned_parts(&[
            recovery_action_kind.as_str().to_string(),
            source_posture.as_str().to_string(),
            source_family.as_str().to_string(),
            recovery_target_scope.as_str().to_string(),
        ]);
        Self {
            recovery_action_kind,
            source_posture,
            source_family,
            recovery_target_scope,
            resulting_binding_identity,
            resulting_target_identity,
            fact_digest,
        }
    }

    #[cfg(test)]
    pub fn recovery_action_kind(&self) -> GeometryRecoveryAction {
        self.recovery_action_kind
    }

    #[cfg(test)]
    pub fn source_posture(&self) -> GeometryRecoverySourcePosture {
        self.source_posture
    }

    #[cfg(test)]
    pub fn source_family(&self) -> PrimitiveConstructionFamily {
        self.source_family
    }

    #[cfg(test)]
    pub fn recovery_target_scope(&self) -> GeometryRecoveryTargetScope {
        self.recovery_target_scope
    }

    #[cfg(test)]
    pub fn resulting_binding_identity(&self) -> Option<&str> {
        self.resulting_binding_identity.as_deref()
    }

    #[cfg(test)]
    pub fn resulting_target_identity(&self) -> Option<&str> {
        self.resulting_target_identity.as_deref()
    }

    #[cfg(test)]
    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }
}

#[cfg(test)]
pub fn geometry_recovery_actions_for_rejection_class(
    rejection_class: super::outcome_rejection::PrimitiveConstructionRejectionClass,
) -> &'static [GeometryRecoveryAction] {
    match rejection_class {
        super::outcome_rejection::PrimitiveConstructionRejectionClass::InvalidRequest => {
            &[GeometryRecoveryAction::CorrectRequestFamilyOrCounts]
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::GeometryScaffold => {
            &[GeometryRecoveryAction::ReviseGeometryScaffold]
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::ConditioningExhaustion => {
            &[GeometryRecoveryAction::EscalateRealizationConditioning]
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::SpatialBirth
        | super::outcome_rejection::PrimitiveConstructionRejectionClass::ImpossibleBirthAttachment => {
            &[GeometryRecoveryAction::CorrectBirthAttachment]
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::TopologyExecution => {
            &[GeometryRecoveryAction::RetryTopologyExecution]
        }
    }
}

#[cfg(test)]
pub fn geometry_recovery_receipts_for_rejection_class(
    family: PrimitiveConstructionFamily,
    rejection_class: super::outcome_rejection::PrimitiveConstructionRejectionClass,
) -> Vec<GeometryRecoveryActionFactReceipt> {
    let target_scope = match rejection_class {
        super::outcome_rejection::PrimitiveConstructionRejectionClass::InvalidRequest => {
            GeometryRecoveryTargetScope::RequestFamilyOrCounts
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::GeometryScaffold => {
            GeometryRecoveryTargetScope::GeometryScaffold
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::ConditioningExhaustion => {
            GeometryRecoveryTargetScope::RealizationConditioning
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::SpatialBirth
        | super::outcome_rejection::PrimitiveConstructionRejectionClass::ImpossibleBirthAttachment => {
            GeometryRecoveryTargetScope::BirthAttachment
        }
        super::outcome_rejection::PrimitiveConstructionRejectionClass::TopologyExecution => {
            GeometryRecoveryTargetScope::TopologyExecution
        }
    };
    geometry_recovery_actions_for_rejection_class(rejection_class)
        .iter()
        .copied()
        .map(|action| GeometryRecoveryActionFactReceipt::new(action, family, target_scope))
        .collect()
}
