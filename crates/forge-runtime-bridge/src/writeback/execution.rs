use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::adapter::{TruthWritebackReceipt, TruthWritebackRequest};
use crate::identity::{BridgeIdentity, WritebackExecutionRecordIdentityTag};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeValidatedWritebackCandidate, BridgeWritebackAuthorityOutcome, BridgeWritebackCounters,
    BridgeWritebackFailureClass, BridgeWritebackIdempotenceBasis,
    BridgeWritebackLoopPreventionReport, BridgeWritebackMapperRecord, BridgeWritebackReplayBundle,
    BridgeWritebackStrategyCompatibilityReport,
};

pub type BridgeWritebackExecutionRecordIdentity =
    BridgeIdentity<WritebackExecutionRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackExecutionRecord {
    record_identity: BridgeWritebackExecutionRecordIdentity,
    contract_digest: Arc<str>,
    derived_effect_digest: Arc<str>,
    proposed_effect_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    causality_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    loop_prevention_digest: Arc<str>,
    strategy_compatibility_digest: Arc<str>,
    mapper_record_digest: Option<Arc<str>>,
    candidate_digest: Option<Arc<str>>,
    outcome_digest: Option<Arc<str>>,
    outcome_class: Option<crate::writeback::BridgeWritebackOutcomeClass>,
    replay_bundle_digest: Option<Arc<str>>,
    request_digest: Option<Arc<str>>,
    receipt_digest: Option<Arc<str>>,
    execution_receipt_digest: Option<Arc<str>>,
    failure_class: Option<BridgeWritebackFailureClass>,
    failure_digest: Option<Arc<str>>,
    counters: BridgeWritebackCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackExecutionRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: &BridgeWritebackLoopPreventionReport,
        strategy_compatibility: &BridgeWritebackStrategyCompatibilityReport,
        mapper_record: Option<&BridgeWritebackMapperRecord>,
        candidate: Option<&BridgeValidatedWritebackCandidate>,
        outcome: Option<&BridgeWritebackAuthorityOutcome>,
        replay_bundle: Option<&BridgeWritebackReplayBundle>,
        request: Option<&TruthWritebackRequest>,
        receipt: Option<&TruthWritebackReceipt>,
        execution_receipt_digest: Option<impl Into<Arc<str>>>,
        failure_class: Option<BridgeWritebackFailureClass>,
        failure_digest: Option<impl Into<Arc<str>>>,
        counters: BridgeWritebackCounters,
    ) -> Self {
        let execution_receipt_digest = execution_receipt_digest.map(Into::into);
        let failure_digest = failure_digest.map(Into::into);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-execution-record|contract={}|derived-effect={}|proposed-effect={}|family:{:?}|strategy-class:{:?}|causality={}|idempotence={}|loop-prevention={}|strategy-compatibility={}|mapper-record={}|candidate={}|outcome={}|outcome-class={}|replay-bundle={}|request={}|receipt={}|execution-receipt={}|failure-class={}|failure-digest={}|counter-digest={}",
            contract.digest(),
            effect.digest(),
            effect.effect_digest(),
            effect.family_kind(),
            effect.strategy_class(),
            effect.causality_digest(),
            idempotence.digest(),
            loop_prevention.digest(),
            strategy_compatibility.digest(),
            mapper_record.map_or("none", BridgeWritebackMapperRecord::digest),
            candidate.map_or("none", BridgeValidatedWritebackCandidate::digest),
            outcome.map_or("none", BridgeWritebackAuthorityOutcome::digest),
            outcome
                .map(|value| format!("{:?}", value.outcome_class()))
                .unwrap_or_else(|| "none".to_string()),
            replay_bundle.map_or("none", BridgeWritebackReplayBundle::digest),
            request.map_or("none", TruthWritebackRequest::digest),
            receipt.map_or("none", TruthWritebackReceipt::digest),
            execution_receipt_digest.as_deref().unwrap_or("none"),
            failure_class
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "none".to_string()),
            failure_digest.as_deref().unwrap_or("none"),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeWritebackExecutionRecordIdentity::new(format!(
                "bridge-writeback-execution-record:sha256:{digest:x}"
            )),
            contract_digest: Arc::from(contract.digest().to_owned()),
            derived_effect_digest: Arc::from(effect.digest().to_owned()),
            proposed_effect_digest: Arc::from(effect.effect_digest().to_owned()),
            family_kind: effect.family_kind(),
            strategy_class: effect.strategy_class(),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            idempotence_digest: Arc::from(idempotence.digest().to_owned()),
            loop_prevention_digest: Arc::from(loop_prevention.digest().to_owned()),
            strategy_compatibility_digest: Arc::from(strategy_compatibility.digest().to_owned()),
            mapper_record_digest: mapper_record.map(|value| Arc::from(value.digest().to_owned())),
            candidate_digest: candidate.map(|value| Arc::from(value.digest().to_owned())),
            outcome_digest: outcome.map(|value| Arc::from(value.digest().to_owned())),
            outcome_class: outcome.map(BridgeWritebackAuthorityOutcome::outcome_class),
            replay_bundle_digest: replay_bundle.map(|value| Arc::from(value.digest().to_owned())),
            request_digest: request.map(|value| Arc::from(value.digest().to_owned())),
            receipt_digest: receipt.map(|value| Arc::from(value.digest().to_owned())),
            execution_receipt_digest,
            failure_class,
            failure_digest,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-execution-record:sha256:{digest:x}"
            )),
        }
    }

    pub fn record_identity(&self) -> &BridgeWritebackExecutionRecordIdentity {
        &self.record_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn derived_effect_digest(&self) -> &str {
        self.derived_effect_digest.as_ref()
    }

    pub fn proposed_effect_digest(&self) -> &str {
        self.proposed_effect_digest.as_ref()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.loop_prevention_digest.as_ref()
    }

    pub fn strategy_compatibility_digest(&self) -> &str {
        self.strategy_compatibility_digest.as_ref()
    }

    pub fn candidate_digest(&self) -> Option<&str> {
        self.candidate_digest.as_deref()
    }

    pub fn mapper_record_digest(&self) -> Option<&str> {
        self.mapper_record_digest.as_deref()
    }

    pub fn outcome_digest(&self) -> Option<&str> {
        self.outcome_digest.as_deref()
    }

    pub fn outcome_class(&self) -> Option<crate::writeback::BridgeWritebackOutcomeClass> {
        self.outcome_class
    }

    pub fn replay_bundle_digest(&self) -> Option<&str> {
        self.replay_bundle_digest.as_deref()
    }

    pub fn request_digest(&self) -> Option<&str> {
        self.request_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }

    pub fn execution_receipt_digest(&self) -> Option<&str> {
        self.execution_receipt_digest.as_deref()
    }

    pub fn failure_class(&self) -> Option<BridgeWritebackFailureClass> {
        self.failure_class
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn counters(&self) -> &BridgeWritebackCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn with_execution_receipt_digest(
        &self,
        execution_receipt_digest: impl Into<Arc<str>>,
    ) -> Self {
        let execution_receipt_digest = execution_receipt_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-execution-record|contract={}|derived-effect={}|proposed-effect={}|family:{:?}|strategy-class:{:?}|causality={}|idempotence={}|loop-prevention={}|strategy-compatibility={}|mapper-record={}|candidate={}|outcome={}|outcome-class={}|replay-bundle={}|request={}|receipt={}|execution-receipt={}|failure-class={}|failure-digest={}|counter-digest={}",
            self.contract_digest.as_ref(),
            self.derived_effect_digest.as_ref(),
            self.proposed_effect_digest.as_ref(),
            self.family_kind,
            self.strategy_class,
            self.causality_digest.as_ref(),
            self.idempotence_digest.as_ref(),
            self.loop_prevention_digest.as_ref(),
            self.strategy_compatibility_digest.as_ref(),
            self.mapper_record_digest.as_deref().unwrap_or("none"),
            self.candidate_digest.as_deref().unwrap_or("none"),
            self.outcome_digest.as_deref().unwrap_or("none"),
            self.outcome_class
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.replay_bundle_digest.as_deref().unwrap_or("none"),
            self.request_digest.as_deref().unwrap_or("none"),
            self.receipt_digest.as_deref().unwrap_or("none"),
            execution_receipt_digest.as_ref(),
            self.failure_class
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.failure_digest.as_deref().unwrap_or("none"),
            self.counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeWritebackExecutionRecordIdentity::new(format!(
                "bridge-writeback-execution-record:sha256:{digest:x}"
            )),
            contract_digest: Arc::clone(&self.contract_digest),
            derived_effect_digest: Arc::clone(&self.derived_effect_digest),
            proposed_effect_digest: Arc::clone(&self.proposed_effect_digest),
            family_kind: self.family_kind,
            strategy_class: self.strategy_class,
            causality_digest: Arc::clone(&self.causality_digest),
            idempotence_digest: Arc::clone(&self.idempotence_digest),
            loop_prevention_digest: Arc::clone(&self.loop_prevention_digest),
            strategy_compatibility_digest: Arc::clone(&self.strategy_compatibility_digest),
            mapper_record_digest: self.mapper_record_digest.clone(),
            candidate_digest: self.candidate_digest.clone(),
            outcome_digest: self.outcome_digest.clone(),
            outcome_class: self.outcome_class,
            replay_bundle_digest: self.replay_bundle_digest.clone(),
            request_digest: self.request_digest.clone(),
            receipt_digest: self.receipt_digest.clone(),
            execution_receipt_digest: Some(execution_receipt_digest),
            failure_class: self.failure_class,
            failure_digest: self.failure_digest.clone(),
            counters: self.counters.clone(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-execution-record:sha256:{digest:x}"
            )),
        }
    }
}
