pub(super) use super::super::*;
pub(super) use crate::facade::{
    BridgeExecutionPolicyClass, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeRuntimePolicy, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackErrorKind, BridgeWritebackFailureClass, BridgeWritebackFamilyKind,
    BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackLoopDisposition, BridgeWritebackRequestMode, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyCompatibilityDisposition,
};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub(super) struct RejectingWritebackAuthority {
    pub(super) failure_class: BridgeWritebackFailureClass,
}

#[derive(Clone)]
pub(super) struct FailingWritebackAuthority;

#[derive(Clone)]
pub(super) struct PanickingWritebackAuthority;

#[derive(Clone, Default)]
pub(super) struct InspectingWritebackAuthority {
    last_request: Arc<RwLock<Option<crate::adapter::TruthWritebackRequest>>>,
}

#[derive(Clone)]
pub(super) struct MismatchedReceiptWritebackAuthority;

#[derive(Clone)]
pub(super) struct MalformedRejectedReceiptWritebackAuthority;

#[derive(Clone)]
pub(super) struct MalformedSuccessfulReceiptWritebackAuthority;

impl InspectingWritebackAuthority {
    pub(super) fn take_last_request(&self) -> Option<crate::adapter::TruthWritebackRequest> {
        self.last_request
            .write()
            .expect("inspecting writeback authority lock poisoned")
            .take()
    }
}

impl crate::adapter::TruthWritebackAuthority for RejectingWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        Ok(
            crate::adapter::TruthWritebackReceipt::new_with_failure_class(
                crate::facade::BridgeWritebackOutcomeClass::Rejected,
                Some(self.failure_class),
                format!("authoritative-rejection:{}", request.digest()),
                &request,
            ),
        )
    }
}

impl crate::adapter::TruthWritebackAuthority for FailingWritebackAuthority {
    fn execute_writeback(
        &self,
        _request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        Err(crate::adapter::TruthWritebackAuthorityError::new(
            "writeback authority transport failure",
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for PanickingWritebackAuthority {
    fn execute_writeback(
        &self,
        _request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        panic!("writeback strategy panic");
    }
}

impl crate::adapter::TruthWritebackAuthority for InspectingWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        *self
            .last_request
            .write()
            .expect("inspecting writeback authority lock poisoned") = Some(request.clone());
        Ok(crate::adapter::TruthWritebackReceipt::new(
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            format!("authoritative-artifact:{}", request.digest()),
            &request,
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for MismatchedReceiptWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        let mismatched_request = crate::adapter::TruthWritebackRequest::new(
            request.family_kind(),
            "contract:sha256:mismatched",
            "candidate:sha256:mismatched",
            request.mapped_input_digest(),
            request.mapper_witness_digest(),
            request.derived_effect_digest(),
            request.proposed_effect_digest(),
            request.effect_class(),
            request.strategy_class(),
            request.feedback_provenance_digest(),
            request.loop_prevention_digest(),
            request.loop_prevention_disposition(),
            request.strategy_compatibility_digest(),
            "causality:sha256:mismatched",
            request.idempotence_digest(),
            request.idempotence_class(),
            request.strategy_descriptor_digest(),
        );
        Ok(crate::adapter::TruthWritebackReceipt::new(
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            format!("authoritative-artifact:{}", request.digest()),
            &mismatched_request,
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for MalformedRejectedReceiptWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        Ok(crate::adapter::TruthWritebackReceipt::new(
            crate::facade::BridgeWritebackOutcomeClass::Rejected,
            format!("authoritative-rejection:{}", request.digest()),
            &request,
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for MalformedSuccessfulReceiptWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        Ok(
            crate::adapter::TruthWritebackReceipt::new_with_failure_class(
                crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
                Some(BridgeWritebackFailureClass::StrategyFailed),
                format!("authoritative-artifact:{}", request.digest()),
                &request,
            ),
        )
    }
}

pub(super) fn lowered_policy(
    runtime: &RuntimeBridge,
) -> crate::facade::LoweredBridgeExecutionPolicy {
    let contract = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:writeback"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .expect("authoritative writeback policy should admit");
    runtime.lower_admitted_policy(&contract)
}

pub(super) fn writeback_declaration(
    declaration_identity: &str,
    request_kind: BridgeRequestKind,
    request_mode: BridgeWritebackRequestMode,
    strategy_descriptor_digest: &str,
) -> BridgeWritebackDeclaration {
    match request_mode {
        BridgeWritebackRequestMode::ReadOnly => BridgeWritebackDeclaration::read_only(
            BridgeWritebackDeclarationIdentity::new(declaration_identity),
            request_kind,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
        BridgeWritebackRequestMode::WritebackCapable => {
            BridgeWritebackDeclaration::writeback_capable(
                BridgeWritebackDeclarationIdentity::new(declaration_identity),
                request_kind,
                BridgeWritebackFamilyKind::ProjectedStateDiff,
                BridgeWritebackEffectClass::ProjectedStateDiff,
                BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                strategy_descriptor_digest,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            )
        }
    }
}

pub(super) fn writeback_declaration_with_shape(
    declaration_identity: &str,
    request_kind: BridgeRequestKind,
    request_mode: BridgeWritebackRequestMode,
    effect_class: BridgeWritebackEffectClass,
    strategy_descriptor_digest: &str,
    idempotence_class: BridgeWritebackIdempotenceClass,
) -> BridgeWritebackDeclaration {
    match request_mode {
        BridgeWritebackRequestMode::ReadOnly => BridgeWritebackDeclaration::read_only(
            BridgeWritebackDeclarationIdentity::new(declaration_identity),
            request_kind,
            effect_class,
            idempotence_class,
        ),
        BridgeWritebackRequestMode::WritebackCapable => {
            BridgeWritebackDeclaration::writeback_capable(
                BridgeWritebackDeclarationIdentity::new(declaration_identity),
                request_kind,
                match effect_class {
                    BridgeWritebackEffectClass::ProjectedStateDiff => {
                        BridgeWritebackFamilyKind::ProjectedStateDiff
                    }
                    BridgeWritebackEffectClass::AspectReconciliation => {
                        BridgeWritebackFamilyKind::AspectReconciliation
                    }
                },
                effect_class,
                match effect_class {
                    BridgeWritebackEffectClass::ProjectedStateDiff => {
                        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
                    }
                    BridgeWritebackEffectClass::AspectReconciliation => {
                        BridgeWritebackStrategyClass::AspectReconciliationCommit
                    }
                },
                strategy_descriptor_digest,
                idempotence_class,
            )
        }
    }
}

pub(super) fn causality_basis(
    identity: &str,
    truth_trigger_digest: &str,
) -> crate::facade::BridgeWritebackCausalityBasis {
    crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(identity),
        truth_trigger_digest,
        "route:sha256:analysis",
        "evaluation:sha256:analysis",
        "truth-view:sha256:analysis",
    )
}
