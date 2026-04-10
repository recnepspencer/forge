use super::*;
use crate::facade::{
    BridgeExecutionPolicyClass, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeRuntimePolicy, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass,
    BridgeWritebackEffectIdentity, BridgeWritebackErrorKind, BridgeWritebackFailureClass,
    BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackLoopDisposition,
    BridgeWritebackRequestMode, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyCompatibilityDisposition,
};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
struct RejectingWritebackAuthority {
    failure_class: BridgeWritebackFailureClass,
}

#[derive(Clone)]
struct FailingWritebackAuthority;

#[derive(Clone)]
struct PanickingWritebackAuthority;

#[derive(Clone, Default)]
struct InspectingWritebackAuthority {
    last_request: Arc<RwLock<Option<crate::adapter::TruthWritebackRequest>>>,
}

#[derive(Clone)]
struct MismatchedReceiptWritebackAuthority;

#[derive(Clone)]
struct MalformedRejectedReceiptWritebackAuthority;

#[derive(Clone)]
struct MalformedSuccessfulReceiptWritebackAuthority;

impl InspectingWritebackAuthority {
    fn take_last_request(&self) -> Option<crate::adapter::TruthWritebackRequest> {
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
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
        Ok(crate::adapter::TruthWritebackReceipt::new_with_failure_class(
            crate::facade::BridgeWritebackOutcomeClass::Rejected,
            Some(self.failure_class),
            format!("authoritative-rejection:{}", request.digest()),
            &request,
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for FailingWritebackAuthority {
    fn execute_writeback(
        &self,
        _request: crate::adapter::TruthWritebackRequest,
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
        Err(crate::adapter::TruthWritebackAuthorityError::new(
            "writeback authority transport failure",
        ))
    }
}

impl crate::adapter::TruthWritebackAuthority for PanickingWritebackAuthority {
    fn execute_writeback(
        &self,
        _request: crate::adapter::TruthWritebackRequest,
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
        panic!("writeback strategy panic");
    }
}

impl crate::adapter::TruthWritebackAuthority for InspectingWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
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
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
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
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
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
    ) -> Result<
        crate::adapter::TruthWritebackReceipt,
        crate::adapter::TruthWritebackAuthorityError,
    > {
        Ok(crate::adapter::TruthWritebackReceipt::new_with_failure_class(
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            Some(BridgeWritebackFailureClass::StrategyFailed),
            format!("authoritative-artifact:{}", request.digest()),
            &request,
        ))
    }
}

fn lowered_policy(runtime: &RuntimeBridge) -> crate::facade::LoweredBridgeExecutionPolicy {
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

fn writeback_declaration(
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
        BridgeWritebackRequestMode::WritebackCapable => BridgeWritebackDeclaration::writeback_capable(
            BridgeWritebackDeclarationIdentity::new(declaration_identity),
            request_kind,
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            strategy_descriptor_digest,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
    }
}

fn writeback_declaration_with_shape(
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
        BridgeWritebackRequestMode::WritebackCapable => BridgeWritebackDeclaration::writeback_capable(
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
        ),
    }
}

fn causality_basis(
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

#[test]
fn runtime_rejects_preview_writeback_declarations() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = writeback_declaration(
        "writeback:preview",
        BridgeRequestKind::Preview,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:preview",
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("preview writeback must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::PreviewWritebackRejected);
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_strategy_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        "strategy:sha256:readonly",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind strategy digests");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::WritebackNotRequested);
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_strategy_class_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        "",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind strategy classes");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::WritebackNotRequested);
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_family_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly-family"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        "",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind writeback family");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::WritebackNotRequested);
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_strategy_descriptor() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = writeback_declaration(
        "writeback:missing-strategy",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "   ",
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind a non-empty strategy descriptor");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyDescriptorMismatch);
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_family_kind() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-family"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        "strategy:sha256:missing-family",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind an explicit writeback family");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::FamilyBindingMismatch);
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_strategy_class() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        "strategy:sha256:missing-class",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind an explicit strategy class");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyDescriptorMismatch);
}

#[test]
fn runtime_rejects_writeback_admission_when_runtime_disables_replay_artifacts() {
    let permissive_runtime = runtime(BridgeRuntimePolicy::default());
    let runtime = runtime(BridgeRuntimePolicy::operational().with_replay_artifacts(false));
    let lowered_policy = lowered_policy(&permissive_runtime);
    let declaration = writeback_declaration(
        "writeback:replay-disabled",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:authoritative",
    );

    let error = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect_err("writeback must fail closed when replay artifacts are unavailable");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::PolicyRejected);
}

#[test]
fn runtime_admits_family_distinct_aspect_reconciliation_writeback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration_with_shape(
        "writeback:aspect-reconciliation",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::AspectReconciliation,
        "strategy:sha256:aspect-reconciliation",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("phase 1 family admission should admit aspect reconciliation family");
    let family_basis = contract
        .validated_declaration()
        .family_basis()
        .expect("admitted writeback contract should preserve family basis");
    let causality = causality_basis("writeback:aspect-reconciliation:causality", "truth-trigger:aspect");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("writeback:aspect-reconciliation:effect"),
        "effect:sha256:aspect-reconciliation",
    );

    assert_eq!(family_basis.family_kind(), BridgeWritebackFamilyKind::AspectReconciliation);
    assert_eq!(effect.family_kind(), BridgeWritebackFamilyKind::AspectReconciliation);
    assert_eq!(effect.effect_class(), BridgeWritebackEffectClass::AspectReconciliation);
    let admission_record = runtime
        .diagnostics()
        .last_writeback_admission_record()
        .expect("writeback family admission should retain native admission record");
    let admission_explanation = runtime
        .diagnostics()
        .explain_last_writeback_admission_record()
        .expect("writeback family admission explanation should exist");
    assert_eq!(admission_record.contract_digest(), contract.digest());
    assert_eq!(admission_record.family_kind(), BridgeWritebackFamilyKind::AspectReconciliation);
    assert_eq!(admission_explanation.contract_digest(), contract.digest());
    assert_eq!(
        admission_explanation.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
}

#[test]
fn runtime_rejects_phase_1_unadmitted_repeated_authority_attempts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration_with_shape(
        "writeback:repeated-authority-attempt",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        "strategy:sha256:repeated-authority-attempt",
        BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt,
    );

    let error = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect_err("phase 1 writeback should reject repeated authority attempt admission");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::FamilyBindingMismatch);
    assert!(error
        .to_string()
        .contains("RequireSemanticNoopSuppression"));
}

#[test]
fn runtime_lowers_writeback_effect_with_canonical_causality_and_strategy_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:effect",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:canonical",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration.clone(), &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:effect", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:canonical"),
        "effect:sha256:update-profile",
    );

    assert_eq!(
        effect.strategy_descriptor_digest(),
        declaration.strategy_descriptor_digest()
    );
    assert_eq!(
        effect.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert!(contract
        .validated_declaration()
        .strategy_basis()
        .expect("admitted writeback declaration should preserve strategy basis")
        .digest()
        .starts_with("bridge-writeback-strategy-basis:sha256:"));
    assert_eq!(effect.effect_digest(), "effect:sha256:update-profile");
    assert_eq!(effect.causality_digest(), causality.digest());
    assert!(effect.digest().starts_with("bridge-derived-writeback-effect:sha256:"));
}

#[test]
fn runtime_maps_writeback_family_input_before_effect_lowering() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:mapped-family-input",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:mapped-family-input",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:mapped-family-input",
        "trigger:sha256:mapped-family-input",
    );
    let mapper_envelope = runtime.lower_writeback_mapper_envelope(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "evidence:sha256:mapped-family-input",
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "evidence:sha256:mapped-family-input",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mapped-family-input"),
        "effect:sha256:mapped-family-input",
    );
    let lowered_path_mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "bridge-mapper-evidence:none",
    );

    assert_eq!(mapper_envelope.contract_digest(), contract.digest());
    assert_eq!(
        mapper_envelope.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(mapper_envelope.causality_digest(), causality.digest());
    assert_eq!(
        mapper_envelope.domain_payload_digest(),
        "effect:sha256:mapped-family-input"
    );
    assert_eq!(
        mapper_envelope.domain_evidence_digest(),
        "evidence:sha256:mapped-family-input"
    );
    assert_eq!(mapped_input.mapper_envelope_digest(), mapper_envelope.digest());
    assert_eq!(mapped_input.contract_digest(), contract.digest());
    assert_eq!(
        mapped_input.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        mapped_input.effect_class(),
        BridgeWritebackEffectClass::ProjectedStateDiff
    );
    assert_eq!(
        mapped_input.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(mapped_input.causality_digest(), causality.digest());
    assert_eq!(
        mapped_input.domain_payload_digest(),
        "effect:sha256:mapped-family-input"
    );
    assert_eq!(
        mapped_input.domain_evidence_digest(),
        "evidence:sha256:mapped-family-input"
    );
    let retained_envelope = runtime
        .diagnostics()
        .writeback_mapper_envelope_for_digest(mapper_envelope.digest())
        .expect("runtime should retain mapper envelope records");
    assert_eq!(retained_envelope.digest(), mapper_envelope.digest());
    let mapper_envelope_explanation = runtime
        .diagnostics()
        .explain_writeback_mapper_envelope(&retained_envelope);
    assert_eq!(mapper_envelope_explanation.envelope_digest(), mapper_envelope.digest());
    assert_eq!(
        mapper_envelope_explanation.domain_payload_digest(),
        mapper_envelope.domain_payload_digest()
    );
    assert_eq!(
        mapper_envelope_explanation.domain_evidence_digest(),
        mapper_envelope.domain_evidence_digest()
    );
    assert_eq!(effect.contract_digest(), mapped_input.contract_digest());
    assert_eq!(effect.family_kind(), mapped_input.family_kind());
    assert_eq!(effect.effect_class(), mapped_input.effect_class());
    assert_eq!(effect.strategy_class(), mapped_input.strategy_class());
    assert_eq!(effect.causality_digest(), mapped_input.causality_digest());
    assert_eq!(effect.effect_digest(), mapped_input.domain_payload_digest());
    assert_eq!(
        effect.mapper_envelope_digest(),
        lowered_path_mapped_input.mapper_envelope_digest()
    );
    assert_eq!(effect.mapped_input_digest(), lowered_path_mapped_input.digest());
    assert_eq!(effect.contract_digest(), lowered_path_mapped_input.contract_digest());
    assert_eq!(effect.family_kind(), lowered_path_mapped_input.family_kind());
    assert_eq!(effect.effect_class(), lowered_path_mapped_input.effect_class());
    assert_eq!(effect.strategy_class(), lowered_path_mapped_input.strategy_class());
    assert_eq!(effect.causality_digest(), lowered_path_mapped_input.causality_digest());
    assert_eq!(
        effect.effect_digest(),
        lowered_path_mapped_input.domain_payload_digest()
    );
    assert!(effect
        .canonical_basis()
        .contains(lowered_path_mapped_input.digest()));
}

#[test]
fn runtime_classifies_writeback_idempotence_stably_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:idempotence",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:idempotence",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:idempotence", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:idempotence"),
        "effect:sha256:canonical-upsert",
    );

    let left = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let right = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:drifted",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_ne!(left.digest(), drifted.digest());
}

#[test]
fn runtime_validates_writeback_candidate_stably_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:candidate",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:candidate",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:candidate", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:candidate"),
        "effect:sha256:candidate",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:candidate",
        BridgeWritebackIdempotenceIdentity::new("idempotence:candidate"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let strategy_compatibility =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);

    let left = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should succeed");
    let right = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should remain stable");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_eq!(
        left.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
}

#[test]
fn runtime_classifies_strategy_compatibility_for_matching_shapes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:strategy-compatibility",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:strategy-compatibility",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:strategy-compatibility", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:strategy-compatibility"),
        "effect:sha256:strategy-compatibility",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:strategy-compatibility",
        BridgeWritebackIdempotenceIdentity::new("idempotence:strategy-compatibility"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let report = runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);

    assert_eq!(
        report.disposition(),
        BridgeWritebackStrategyCompatibilityDisposition::Compatible
    );
}

#[test]
fn runtime_replay_writeback_bundle_changes_when_outcome_changes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:replay",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:replay",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay"),
        "effect:sha256:authoritative-upsert",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let noop_outcome = crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&idempotence);
    let commit_outcome = crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
        &idempotence,
        "authoritative-artifact:sha256:commit-a",
    );

    let noop_bundle = runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &noop_outcome);
    let commit_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &commit_outcome);
    let noop_bundle_repeat =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &noop_outcome);

    assert_eq!(noop_bundle, noop_bundle_repeat);
    assert_eq!(noop_bundle.digest(), noop_bundle_repeat.digest());
    assert_eq!(
        noop_bundle.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        noop_bundle.strategy_descriptor_digest(),
        contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract should preserve strategy basis")
            .strategy_descriptor_digest()
    );
    assert_ne!(noop_bundle.semantic_digest(), commit_bundle.semantic_digest());
    assert_eq!(noop_bundle.causality_digest(), effect.causality_digest());
    assert_eq!(noop_bundle.lowered_policy_digest(), lowered_policy.digest());
    assert_eq!(
        noop_bundle.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(
        noop_bundle.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop
    );
    assert_eq!(
        commit_bundle.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        commit_bundle.authoritative_artifact_digest(),
        "authoritative-artifact:sha256:commit-a"
    );
    assert_ne!(noop_outcome.digest(), commit_outcome.digest());
    assert_ne!(noop_bundle.digest(), commit_bundle.digest());
}

#[test]
fn runtime_replay_writeback_bundle_changes_when_family_changes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let projected_declaration = writeback_declaration_with_shape(
        "writeback:replay-family:projected",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        "strategy:sha256:replay-family:projected",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_declaration = writeback_declaration_with_shape(
        "writeback:replay-family:aspect",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::AspectReconciliation,
        "strategy:sha256:replay-family:aspect",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let projected_contract = runtime
        .admit_writeback_declaration(projected_declaration, &lowered_policy)
        .expect("projected family declaration should admit");
    let aspect_contract = runtime
        .admit_writeback_declaration(aspect_declaration, &lowered_policy)
        .expect("aspect family declaration should admit");
    let projected_causality =
        causality_basis("causality:replay-family:projected", "trigger:sha256:shared");
    let aspect_causality =
        causality_basis("causality:replay-family:aspect", "trigger:sha256:shared");
    let projected_effect = runtime.lower_writeback_effect(
        &projected_contract,
        &projected_causality,
        BridgeWritebackEffectIdentity::new("effect:replay-family:projected"),
        "effect:sha256:shared",
    );
    let aspect_effect = runtime.lower_writeback_effect(
        &aspect_contract,
        &aspect_causality,
        BridgeWritebackEffectIdentity::new("effect:replay-family:aspect"),
        "effect:sha256:shared",
    );
    let projected_idempotence = runtime.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy,
        "truth-state:sha256:shared",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-family:projected"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy,
        "truth-state:sha256:shared",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-family:aspect"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let projected_outcome =
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&projected_idempotence);
    let aspect_outcome =
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&aspect_idempotence);
    let projected_bundle = runtime.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle =
        runtime.replay_writeback_bundle(&aspect_contract, &aspect_effect, &aspect_idempotence, &aspect_outcome);

    assert_eq!(projected_bundle.family_kind(), BridgeWritebackFamilyKind::ProjectedStateDiff);
    assert_eq!(aspect_bundle.family_kind(), BridgeWritebackFamilyKind::AspectReconciliation);
    assert_ne!(projected_bundle.semantic_digest(), aspect_bundle.semantic_digest());
    assert_ne!(projected_bundle.digest(), aspect_bundle.digest());
}

#[test]
fn runtime_executes_writeback_through_bound_authority() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:authority-execution",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:authority-execution",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:authority-execution", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:authority-execution"),
        "effect:sha256:authoritative-upsert",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:before",
        BridgeWritebackIdempotenceIdentity::new("idempotence:authority-execution"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let (outcome, receipt) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("bound writeback authority should execute");

    assert_eq!(
        receipt.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert!(
        receipt
            .authoritative_artifact_digest()
            .starts_with("authoritative-artifact:truth-writeback-request:sha256:")
    );
    assert_eq!(receipt.failure_class(), None);
    assert_eq!(outcome.digest().starts_with("bridge-writeback-authority-outcome:sha256:"), true);
}

#[test]
fn runtime_records_native_writeback_execution_record_on_success() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:execution-record-success",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:execution-record-success",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            "causality:execution-record-success",
            "trigger:sha256:commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-record-success"),
        "effect:sha256:execution-record-success",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:execution-record-success",
        BridgeWritebackIdempotenceIdentity::new("idempotence:execution-record-success"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let (outcome, receipt) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("authority execution should succeed");
    let record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("runtime should retain a native writeback execution record");

    assert_eq!(record.contract_digest(), contract.digest());
    assert_eq!(record.derived_effect_digest(), effect.digest());
    assert_eq!(record.proposed_effect_digest(), effect.effect_digest());
    assert_eq!(record.causality_digest(), effect.causality_digest());
    assert_eq!(record.outcome_digest(), Some(outcome.digest()));
    assert_eq!(
        record.outcome_class(),
        Some(crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit)
    );
    assert_eq!(record.request_digest(), Some(receipt.request_digest()));
    assert_eq!(record.receipt_digest(), Some(receipt.digest()));
    assert_eq!(record.failure_class(), None);
    assert_eq!(record.counters().writeback_request_count(), 1);
    assert_eq!(record.counters().writeback_commit_count(), 1);
    assert_eq!(record.counters().writeback_failure_count(), 0);
}

#[test]
fn runtime_records_native_writeback_execution_record_on_pre_authority_failure() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:execution-record-failure",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:execution-record-failure",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            "causality:execution-record-failure",
            "trigger:sha256:commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-record-failure"),
        "effect:sha256:execution-record-failure",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:execution-record-failure",
        BridgeWritebackIdempotenceIdentity::new("idempotence:execution-record-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some("feedback-provenance:sha256:contradictory"),
            None::<&str>,
        )
        .expect_err("partial feedback should fail closed before authority execution");
    let record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("runtime should retain a native failure execution record");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert_eq!(
        record.failure_class(),
        Some(crate::facade::BridgeWritebackFailureClass::InvariantRejected)
    );
    assert_eq!(record.outcome_digest(), None);
    assert_eq!(record.request_digest(), None);
    assert_eq!(record.receipt_digest(), None);
    assert_eq!(record.counters().writeback_failure_count(), 1);
    assert_eq!(record.counters().writeback_validation_rejection_count(), 1);
}

#[test]
fn runtime_passes_explicit_semantic_fields_to_bound_authority() {
    let authority = InspectingWritebackAuthority::default();
    let runtime =
        runtime_with_custom_writeback_authority(BridgeRuntimePolicy::default(), authority.clone());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:authority-request-shape",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:authority-request-shape",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration.clone(), &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:authority-request-shape",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:authority-request-shape"),
        "effect:sha256:authority-request-shape",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:authority-request-shape",
        BridgeWritebackIdempotenceIdentity::new("idempotence:authority-request-shape"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let strategy_compatibility =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);
    let candidate = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("writeback candidate validation should succeed");

    runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("bound writeback authority should execute");

    let request = authority
        .take_last_request()
        .expect("authority should have received exactly one request");
    assert_eq!(request.contract_digest(), contract.digest());
    assert_eq!(request.derived_effect_digest(), effect.digest());
    assert_eq!(request.proposed_effect_digest(), effect.effect_digest());
    assert_eq!(request.family_kind(), effect.family_kind());
    assert_eq!(request.effect_class(), effect.effect_class());
    assert_eq!(request.strategy_class(), effect.strategy_class());
    assert_eq!(request.candidate_digest(), candidate.digest());
    assert_eq!(request.mapped_input_digest(), effect.mapped_input_digest());
    assert_eq!(request.causality_digest(), idempotence.causality_digest());
    assert_eq!(request.idempotence_digest(), idempotence.digest());
    assert_eq!(request.idempotence_class(), idempotence.idempotence_class());
    assert_eq!(
        request.strategy_descriptor_digest(),
        declaration.strategy_descriptor_digest()
    );
    assert_eq!(request.loop_prevention_digest(), loop_prevention.digest());
    assert_eq!(
        request.loop_prevention_disposition(),
        BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    );
    assert_eq!(
        request.strategy_compatibility_digest(),
        strategy_compatibility.digest()
    );
    let mapper_record = runtime
        .diagnostics()
        .last_writeback_mapper_record()
        .expect("runtime should retain a native writeback mapper record");
    assert_eq!(request.mapper_witness_digest(), mapper_record.witness_digest());
    assert_eq!(mapper_record.candidate_digest(), candidate.digest());
    assert_eq!(mapper_record.family_kind(), effect.family_kind());
    assert_eq!(mapper_record.effect_class(), effect.effect_class());
    assert_eq!(mapper_record.strategy_class(), effect.strategy_class());
    assert_eq!(mapper_record.mapper_envelope_digest(), effect.mapper_envelope_digest());
    assert_eq!(mapper_record.mapped_input_digest(), effect.mapped_input_digest());
    let mapper_explanation = runtime
        .diagnostics()
        .explain_last_writeback_mapper_record()
        .expect("writeback mapper explanation should exist");
    assert_eq!(mapper_explanation.witness_digest(), mapper_record.witness_digest());
    assert_eq!(mapper_explanation.candidate_digest(), candidate.digest());
    assert_eq!(mapper_explanation.envelope_digest(), effect.mapper_envelope_digest());
    assert_eq!(mapper_explanation.mapped_input_digest(), effect.mapped_input_digest());
    let mapper_envelope = runtime
        .diagnostics()
        .writeback_mapper_envelope_for_digest(effect.mapper_envelope_digest())
        .expect("runtime should retain mapper envelope for effect lineage");
    assert_eq!(mapper_envelope.causality_digest(), effect.causality_digest());
    let execution_explanation = runtime
        .diagnostics()
        .explain_last_writeback_execution_record()
        .expect("writeback execution explanation should exist");
    assert_eq!(
        execution_explanation.mapper_record_digest(),
        Some(mapper_record.digest())
    );
}

#[test]
fn runtime_rejects_authority_execution_when_no_writeback_authority_is_bound() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:missing-authority",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:missing-authority",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:missing-authority", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:missing-authority"),
        "effect:sha256:missing-authority",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:before",
        BridgeWritebackIdempotenceIdentity::new("idempotence:missing-authority"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("missing authority binding must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::AuthorityBypassRejected);
    assert!(error.to_string().contains("no truth writeback authority"));
}

#[test]
fn runtime_classifies_matching_feedback_as_canonical_noop() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:loop-classification",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:loop-classification",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:loop-classification", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:loop-classification"),
        "effect:sha256:loop-classification",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:before",
        BridgeWritebackIdempotenceIdentity::new("idempotence:loop-classification"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        Some(feedback_provenance.digest()),
        Some(causality.digest()),
    );

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
}

#[test]
fn runtime_suppresses_matching_feedback_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:feedback-suppression",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:feedback-suppression",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:feedback-suppression", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:feedback-suppression"),
        "effect:sha256:feedback-suppression",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:feedback-suppression"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let (loop_prevention, outcome, receipt) = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            Some(causality.digest()),
        )
        .expect("matching feedback should suppress before authority execution");

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
    assert_eq!(
        outcome,
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&idempotence)
    );
    assert!(receipt.is_none());
}

#[test]
fn runtime_rejects_partial_feedback_context_as_unsafe() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:unsafe-feedback",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:unsafe-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:unsafe-feedback", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:unsafe-feedback"),
        "effect:sha256:unsafe-feedback",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:unsafe-feedback"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            None::<std::sync::Arc<str>>,
        )
        .expect_err("partial feedback context must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}

#[test]
fn runtime_rejects_contradictory_feedback_context_as_unsafe() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:contradictory-feedback",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:contradictory-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:contradictory-feedback", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:contradictory-feedback"),
        "effect:sha256:contradictory-feedback",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:contradictory-feedback"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            Some("truth-trigger:sha256:other-commit"),
        )
        .expect_err("contradictory feedback context must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}

#[test]
fn runtime_maps_typed_authority_rejection_into_bridge_error_kind() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        RejectingWritebackAuthority {
            failure_class: BridgeWritebackFailureClass::StaleTruthBasis,
        },
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:typed-rejection",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:typed-rejection",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:typed-rejection", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:typed-rejection"),
        "effect:sha256:typed-rejection",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:typed-rejection",
        BridgeWritebackIdempotenceIdentity::new("idempotence:typed-rejection"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("typed authority rejection should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StaleTruthBasis);
    assert!(error.to_string().contains("StaleTruthBasis"));
}

#[test]
fn runtime_maps_authority_transport_failure_into_strategy_failed() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        FailingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:transport-failure",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:transport-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:transport-failure", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:transport-failure"),
        "effect:sha256:transport-failure",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:transport-failure",
        BridgeWritebackIdempotenceIdentity::new("idempotence:transport-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority transport failure should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyFailed);
    assert!(error.to_string().contains("transport failure"));
}

#[test]
fn runtime_maps_authority_panic_into_strategy_panicked() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        PanickingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:panic-failure",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:panic-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:panic-failure", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:panic-failure"),
        "effect:sha256:panic-failure",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:panic-failure",
        BridgeWritebackIdempotenceIdentity::new("idempotence:panic-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority panic should surface as typed bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyPanicked);
    assert!(error.to_string().contains("writeback strategy panic"));
}

#[test]
fn runtime_rejects_receipt_with_mismatched_request_digest() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MismatchedReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:mismatched-receipt",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:mismatched-receipt",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:mismatched-receipt", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mismatched-receipt"),
        "effect:sha256:mismatched-receipt",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:mismatched-receipt",
        BridgeWritebackIdempotenceIdentity::new("idempotence:mismatched-receipt"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("mismatched receipt request digests must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error.to_string().contains("returned receipt"));
}

#[test]
fn runtime_rejects_rejected_receipt_without_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedRejectedReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:rejected-without-failure-class",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:rejected-without-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:rejected-without-failure-class",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:rejected-without-failure-class"),
        "effect:sha256:rejected-without-failure-class",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:rejected-without-failure-class",
        BridgeWritebackIdempotenceIdentity::new("idempotence:rejected-without-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("rejected receipts without failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error.to_string().contains("without a failure class"));
}

#[test]
fn runtime_rejects_successful_receipt_with_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedSuccessfulReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:success-with-failure-class",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:success-with-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:success-with-failure-class",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:success-with-failure-class"),
        "effect:sha256:success-with-failure-class",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:success-with-failure-class",
        BridgeWritebackIdempotenceIdentity::new("idempotence:success-with-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("successful receipts with failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error
        .to_string()
        .contains("non-rejected receipt"));
}

#[test]
fn runtime_rejects_strategy_compatibility_mismatch_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let contract_a = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:compatibility-a",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:compatibility-a",
            ),
            &lowered_policy,
        )
        .expect("first writeback declaration should admit");
    let contract_b = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:compatibility-b",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:compatibility-b",
            ),
            &lowered_policy,
        )
        .expect("second writeback declaration should admit");
    let effect_a = runtime.lower_writeback_effect(
        &contract_a,
        &causality_basis("causality:compatibility-a", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:compatibility-a"),
        "effect:sha256:compatibility-a",
    );
    let effect_b = runtime.lower_writeback_effect(
        &contract_b,
        &causality_basis("causality:compatibility-b", "trigger:sha256:commit-b"),
        BridgeWritebackEffectIdentity::new("effect:compatibility-b"),
        "effect:sha256:compatibility-b",
    );
    let mismatched_idempotence = runtime.classify_writeback_idempotence(
        &effect_b,
        &lowered_policy,
        "truth-state:sha256:compatibility-b",
        BridgeWritebackIdempotenceIdentity::new("idempotence:compatibility-b"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract_a, &effect_a, &mismatched_idempotence)
        .expect_err("strategy compatibility drift should fail before authority execution");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyDescriptorMismatch);
}

#[test]
fn writeback_diagnostics_explanations_are_artifact_derived_and_stable() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:diagnostics-artifact-derived",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:diagnostics-artifact-derived",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            "causality:diagnostics-artifact-derived",
            "trigger:sha256:commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-artifact-derived"),
        "effect:sha256:diagnostics-artifact-derived",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:diagnostics-artifact-derived",
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-artifact-derived"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let strategy_compatibility =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);
    let candidate = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should succeed");
    let (outcome, _) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("authority execution should succeed");
    let replay_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &outcome);

    let candidate_explanation = runtime.diagnostics().explain_writeback_candidate(&candidate);
    let loop_explanation = runtime
        .diagnostics()
        .explain_writeback_loop_prevention(&loop_prevention);
    let compatibility_explanation = runtime
        .diagnostics()
        .explain_writeback_strategy_compatibility(&strategy_compatibility);
    let outcome_explanation = runtime.diagnostics().explain_writeback_outcome(&outcome);
    let replay_explanation = runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&replay_bundle);
    let mapper_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_mapper_record()
        .expect("writeback mapper explanation should exist");
    let execution_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_execution_record()
        .expect("writeback execution record explanation should exist");

    assert_eq!(candidate_explanation.candidate_digest(), candidate.digest());
    assert_eq!(
        candidate_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        candidate_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        candidate_explanation.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(loop_explanation.loop_prevention_digest(), loop_prevention.digest());
    assert_eq!(
        loop_explanation.disposition(),
        BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    );
    assert_eq!(
        compatibility_explanation.compatibility_digest(),
        strategy_compatibility.digest()
    );
    assert_eq!(
        compatibility_explanation.disposition(),
        BridgeWritebackStrategyCompatibilityDisposition::Compatible
    );
    assert_eq!(
        mapper_record_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        mapper_record_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        mapper_record_explanation.causality_digest(),
        effect.causality_digest()
    );
    assert_eq!(
        mapper_record_explanation.proposed_effect_digest(),
        effect.effect_digest()
    );
    assert_eq!(outcome_explanation.outcome_digest(), outcome.digest());
    assert_eq!(
        outcome_explanation.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        execution_record_explanation.idempotence_digest(),
        idempotence.digest()
    );
    assert_eq!(
        execution_record_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        execution_record_explanation.loop_prevention_digest(),
        loop_prevention.digest()
    );
    assert_eq!(
        execution_record_explanation.strategy_compatibility_digest(),
        strategy_compatibility.digest()
    );
    assert_eq!(
        execution_record_explanation.mapper_record_digest(),
        Some(
            runtime
                .diagnostics()
                .last_writeback_mapper_record()
                .expect("writeback mapper record should exist")
                .digest()
        )
    );
    assert_eq!(replay_explanation.replay_bundle_digest(), replay_bundle.digest());
    assert_eq!(
        replay_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        replay_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(replay_explanation.causality_digest(), effect.causality_digest());
    assert_eq!(
        replay_explanation.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(
        replay_explanation.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
}

#[test]
fn writeback_diagnostics_tier_variation_preserves_replay_meaning() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());

    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:diagnostics-tier-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:diagnostics-tier",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:diagnostics-tier-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:diagnostics-tier",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:diagnostics-tier", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        "effect:sha256:diagnostics-tier",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:diagnostics-tier", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        "effect:sha256:diagnostics-tier",
    );

    let standard_idempotence = standard_runtime.classify_writeback_idempotence(
        &standard_effect,
        &standard_lowered_policy,
        "truth-state:sha256:diagnostics-tier",
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-tier"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let exhaustive_idempotence = exhaustive_runtime.classify_writeback_idempotence(
        &exhaustive_effect,
        &exhaustive_lowered_policy,
        "truth-state:sha256:diagnostics-tier",
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-tier"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let (standard_outcome, _) = standard_runtime
        .execute_writeback_authority(&standard_contract, &standard_effect, &standard_idempotence)
        .expect("standard authority execution should succeed");
    let (exhaustive_outcome, _) = exhaustive_runtime
        .execute_writeback_authority(
            &exhaustive_contract,
            &exhaustive_effect,
            &exhaustive_idempotence,
        )
        .expect("exhaustive authority execution should succeed");

    let standard_bundle = standard_runtime.replay_writeback_bundle(
        &standard_contract,
        &standard_effect,
        &standard_idempotence,
        &standard_outcome,
    );
    let exhaustive_bundle = exhaustive_runtime.replay_writeback_bundle(
        &exhaustive_contract,
        &exhaustive_effect,
        &exhaustive_idempotence,
        &exhaustive_outcome,
    );

    let standard_explanation = standard_runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&standard_bundle);
    let exhaustive_explanation = exhaustive_runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&exhaustive_bundle);

    assert_ne!(standard_bundle.digest(), exhaustive_bundle.digest());
    assert_eq!(standard_bundle.semantic_digest(), exhaustive_bundle.semantic_digest());
    assert_eq!(
        standard_explanation.semantic_digest(),
        exhaustive_explanation.semantic_digest()
    );
    assert_eq!(standard_explanation.strategy_class(), exhaustive_explanation.strategy_class());
    assert_eq!(
        standard_explanation.causality_digest(),
        exhaustive_explanation.causality_digest()
    );
    assert_eq!(
        standard_explanation.retry_disposition(),
        exhaustive_explanation.retry_disposition()
    );
    assert_eq!(
        standard_explanation.outcome_class(),
        exhaustive_explanation.outcome_class()
    );
    standard_runtime
        .validate_replayed_writeback_bundle(&standard_bundle, &exhaustive_bundle)
        .expect("diagnostics-tier variation should preserve writeback replay meaning");
}

#[test]
fn writeback_feedback_provenance_is_diagnostics_invariant_for_semantically_equal_effects() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());
    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:feedback-provenance-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:feedback-provenance",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:feedback-provenance-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:feedback-provenance",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:feedback-provenance", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        "effect:sha256:feedback-provenance",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:feedback-provenance", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        "effect:sha256:feedback-provenance",
    );

    let standard_provenance = standard_runtime.derive_writeback_feedback_provenance(&standard_effect);
    let exhaustive_provenance =
        exhaustive_runtime.derive_writeback_feedback_provenance(&exhaustive_effect);

    assert_ne!(standard_contract.digest(), exhaustive_contract.digest());
    assert_ne!(standard_effect.digest(), exhaustive_effect.digest());
    assert_eq!(standard_provenance.effect_digest(), exhaustive_provenance.effect_digest());
    assert_eq!(
        standard_provenance.causality_digest(),
        exhaustive_provenance.causality_digest()
    );
    assert_eq!(standard_provenance.digest(), exhaustive_provenance.digest());
}

#[test]
fn runtime_rejects_replayed_writeback_bundle_when_semantic_meaning_drifts() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:replay-mismatch",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:replay-mismatch",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let original_effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay-mismatch", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-mismatch:original"),
        "effect:sha256:replay-mismatch:original",
    );
    let drifted_effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay-mismatch", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-mismatch:drifted"),
        "effect:sha256:replay-mismatch:drifted",
    );
    let original_idempotence = runtime.classify_writeback_idempotence(
        &original_effect,
        &lowered_policy,
        "truth-state:sha256:replay-mismatch",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-mismatch:original"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted_idempotence = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        "truth-state:sha256:replay-mismatch",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-mismatch:drifted"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let original_bundle = runtime.replay_writeback_bundle(
        &contract,
        &original_effect,
        &original_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &original_idempotence,
            "authoritative-artifact:sha256:replay-mismatch",
        ),
    );
    let drifted_bundle = runtime.replay_writeback_bundle(
        &contract,
        &drifted_effect,
        &drifted_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &drifted_idempotence,
            "authoritative-artifact:sha256:replay-mismatch",
        ),
    );

    let error = runtime
        .validate_replayed_writeback_bundle(&original_bundle, &drifted_bundle)
        .expect_err("replayed writeback bundle should reject semantic drift");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::ReplayMismatch);
    assert!(error.to_string().contains("semantic mismatch"));
    assert_ne!(original_bundle.digest(), drifted_bundle.digest());
    assert_ne!(
        original_bundle.semantic_digest(),
        drifted_bundle.semantic_digest()
    );

    let replay_record = runtime
        .diagnostics()
        .last_writeback_replay_record()
        .expect("runtime should retain a native writeback replay record");
    assert_eq!(
        replay_record.failure_class(),
        Some(crate::facade::BridgeWritebackFailureClass::ReplayMismatch)
    );
    assert_eq!(replay_record.expected_replay_digest(), original_bundle.digest());
    assert_eq!(replay_record.replayed_replay_digest(), drifted_bundle.digest());
    assert_eq!(replay_record.counters().writeback_replay_request_count(), 1);
    assert_eq!(replay_record.counters().writeback_replay_mismatch_count(), 1);

    let replay_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_replay_record()
        .expect("writeback replay record explanation should exist");
    assert_eq!(
        replay_record_explanation.expected_causality_digest(),
        original_bundle.causality_digest()
    );
    assert_eq!(
        replay_record_explanation.replayed_causality_digest(),
        drifted_bundle.causality_digest()
    );
}

#[test]
fn runtime_accepts_replayed_writeback_bundle_when_only_diagnostics_detail_differs() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());
    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:replay-semantic-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:replay-semantic",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:replay-semantic-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:replay-semantic",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:replay-semantic", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-semantic"),
        "effect:sha256:replay-semantic",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:replay-semantic", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-semantic"),
        "effect:sha256:replay-semantic",
    );
    let standard_idempotence = standard_runtime.classify_writeback_idempotence(
        &standard_effect,
        &standard_lowered_policy,
        "truth-state:sha256:replay-semantic",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-semantic"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let exhaustive_idempotence = exhaustive_runtime.classify_writeback_idempotence(
        &exhaustive_effect,
        &exhaustive_lowered_policy,
        "truth-state:sha256:replay-semantic",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-semantic"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let standard_bundle = standard_runtime.replay_writeback_bundle(
        &standard_contract,
        &standard_effect,
        &standard_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &standard_idempotence,
            "authoritative-artifact:sha256:replay-semantic-standard",
        ),
    );
    let exhaustive_bundle = exhaustive_runtime.replay_writeback_bundle(
        &exhaustive_contract,
        &exhaustive_effect,
        &exhaustive_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &exhaustive_idempotence,
            "authoritative-artifact:sha256:replay-semantic-exhaustive",
        ),
    );

    standard_runtime
        .validate_replayed_writeback_bundle(&standard_bundle, &exhaustive_bundle)
        .expect("replay validation should accept diagnostics-only detail drift");
}
