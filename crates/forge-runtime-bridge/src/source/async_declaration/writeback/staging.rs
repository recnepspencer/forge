use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::facade::{
    AdmittedBridgePolicyContract, AdmittedBridgeWritebackContract,
    BridgeAsyncRequestTruthViewBasisKind, BridgeDerivedWritebackEffect,
    BridgeValidatedWritebackCandidate, BridgeWritebackAuthoritativeStateBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackIdempotenceBasis,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackLoopDisposition,
    BridgeWritebackLoopPreventionReport, BridgeWritebackNativeCausalityInputs,
    BridgeWritebackStrategyCoherenceReport, LoweredBridgeExecutionPolicy, RuntimeBridge,
    TruthCommitIdentity,
};
use crate::identity::{AsyncWritebackStagedEffectIdentityTag, BridgeIdentity};
use crate::routing::BridgeRouteIdentity;
use crate::snapshot::TruthSnapshotIdentity;

use super::{
    AdmittedBridgeAsyncWriteback, BridgeAsyncWritebackCounters, BridgeAsyncWritebackMapperOutput,
    BridgeAsyncWritebackRejection, BridgeAsyncWritebackRejectionKind,
};

pub type BridgeAsyncWritebackStagedEffectIdentity =
    BridgeIdentity<AsyncWritebackStagedEffectIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedBridgeAsyncWritebackEffect {
    staged_effect_identity: BridgeAsyncWritebackStagedEffectIdentity,
    admitted: AdmittedBridgeAsyncWriteback,
    mapper_output: BridgeAsyncWritebackMapperOutput,
    policy_contract: AdmittedBridgePolicyContract,
    lowered_policy: LoweredBridgeExecutionPolicy,
    writeback_contract: AdmittedBridgeWritebackContract,
    causality: BridgeWritebackNativeCausalityInputs,
    effect: BridgeDerivedWritebackEffect,
    authoritative_state_basis: BridgeWritebackAuthoritativeStateBasis,
    idempotence: BridgeWritebackIdempotenceBasis,
    loop_prevention: BridgeWritebackLoopPreventionReport,
    strategy_coherence: BridgeWritebackStrategyCoherenceReport,
    candidate: Option<BridgeValidatedWritebackCandidate>,
    counters: BridgeAsyncWritebackCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl StagedBridgeAsyncWritebackEffect {
    pub(crate) fn stage(
        runtime: &RuntimeBridge,
        admitted: &AdmittedBridgeAsyncWriteback,
    ) -> Result<Self, BridgeAsyncWritebackRejection> {
        let mapper_output = BridgeAsyncWritebackMapperOutput::map(admitted)?;
        let policy_contract = runtime
            .admit_policy_declaration(admitted.policy_declaration().clone())
            .map_err(|error| {
                BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::PolicyAdmissionRejected,
                    format!("{:?}: {}", error.kind(), error.detail()),
                )
            })?;
        let lowered_policy = runtime.lower_admitted_policy(&policy_contract);
        let writeback_contract = runtime
            .admit_writeback_declaration(admitted.writeback_declaration().clone(), &lowered_policy)
            .map_err(|error| {
                BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::WritebackContractRejected,
                    error.to_string(),
                )
            })?;
        let truth_basis = admitted
            .request_identity()
            .basis_binding()
            .truth_view_basis();
        debug_assert_eq!(
            truth_basis.kind(),
            BridgeAsyncRequestTruthViewBasisKind::Authoritative
        );
        let truth_commit = truth_basis
            .truth_commit_identity()
            .cloned()
            .unwrap_or_else(|| TruthCommitIdentity::new("bridge-async-writeback-missing-truth"));
        let truth_snapshot = truth_basis
            .truth_snapshot_identity()
            .cloned()
            .unwrap_or_else(|| {
                TruthSnapshotIdentity::new("bridge-async-writeback-missing-snapshot")
            });
        let causality = BridgeWritebackNativeCausalityInputs::new(
            BridgeWritebackCausalityIdentity::new(format!(
                "bridge-async-writeback-causality:{}",
                admitted.admission_identity().as_str()
            )),
            truth_commit,
            BridgeRouteIdentity::new(format!(
                "bridge-async-writeback-route:{}",
                admitted.completion().completion_identity()
            )),
            truth_snapshot.clone(),
            truth_snapshot,
        );
        let effect = runtime.lower_writeback_effect(
            &writeback_contract,
            &causality,
            crate::facade::BridgeWritebackEffectIdentity::new(format!(
                "bridge-async-writeback-effect:{}",
                admitted.admission_identity().as_str()
            )),
            mapper_output.effect_intent().clone(),
        );
        let authoritative_state_basis =
            BridgeWritebackAuthoritativeStateBasis::from_effect(&effect);
        let idempotence = runtime.classify_writeback_idempotence(
            &effect,
            &lowered_policy,
            &authoritative_state_basis,
            BridgeWritebackIdempotenceIdentity::new(format!(
                "bridge-async-writeback-idempotence:{}",
                admitted.admission_identity().as_str()
            )),
            admitted.writeback_declaration().idempotence_class(),
        );
        let loop_prevention =
            runtime.classify_writeback_loop_prevention(&effect, &idempotence, None);
        let strategy_coherence = runtime.classify_writeback_strategy_coherence(
            &writeback_contract,
            &effect,
            &idempotence,
        );
        let candidate = if loop_prevention.disposition()
            == BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
        {
            Some(
                runtime
                    .validate_writeback_candidate(
                        &writeback_contract,
                        &effect,
                        &idempotence,
                        &loop_prevention,
                        &strategy_coherence,
                    )
                    .map_err(|error| {
                        BridgeAsyncWritebackRejection::new(
                            BridgeAsyncWritebackRejectionKind::CandidateValidationRejected,
                            error.to_string(),
                        )
                    })?,
            )
        } else {
            None
        };
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-writeback-staged-effect|admission={}|mapper-output={}|policy={}|writeback-contract={}|causality={}|effect={}|idempotence={}|loop-prevention={}|strategy-coherence={}|candidate={}",
            admitted.digest(),
            mapper_output.digest(),
            policy_contract.digest(),
            writeback_contract.digest(),
            causality.digest(),
            effect.digest(),
            idempotence.digest(),
            loop_prevention.digest(),
            strategy_coherence.digest(),
            candidate
                .as_ref()
                .map(|value| value.digest())
                .unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            staged_effect_identity: BridgeAsyncWritebackStagedEffectIdentity::new(format!(
                "bridge-async-writeback-staged-effect-id:sha256:{digest:x}"
            )),
            admitted: admitted.clone(),
            mapper_output,
            policy_contract,
            lowered_policy,
            writeback_contract,
            causality,
            effect,
            authoritative_state_basis,
            idempotence,
            loop_prevention,
            strategy_coherence,
            candidate,
            counters: BridgeAsyncWritebackCounters::staged(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-writeback-staged-effect:sha256:{digest:x}"
            )),
        })
    }

    pub fn staged_effect_identity(&self) -> &BridgeAsyncWritebackStagedEffectIdentity {
        &self.staged_effect_identity
    }

    pub fn admitted(&self) -> &AdmittedBridgeAsyncWriteback {
        &self.admitted
    }

    pub fn mapper_output(&self) -> &BridgeAsyncWritebackMapperOutput {
        &self.mapper_output
    }

    pub fn policy_contract(&self) -> &AdmittedBridgePolicyContract {
        &self.policy_contract
    }

    pub fn lowered_policy(&self) -> &LoweredBridgeExecutionPolicy {
        &self.lowered_policy
    }

    pub fn writeback_contract(&self) -> &AdmittedBridgeWritebackContract {
        &self.writeback_contract
    }

    pub fn causality(&self) -> &BridgeWritebackNativeCausalityInputs {
        &self.causality
    }

    pub fn effect(&self) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub fn authoritative_state_basis(&self) -> &BridgeWritebackAuthoritativeStateBasis {
        &self.authoritative_state_basis
    }

    pub fn idempotence(&self) -> &BridgeWritebackIdempotenceBasis {
        &self.idempotence
    }

    pub fn loop_prevention(&self) -> &BridgeWritebackLoopPreventionReport {
        &self.loop_prevention
    }

    pub fn strategy_coherence(&self) -> &BridgeWritebackStrategyCoherenceReport {
        &self.strategy_coherence
    }

    pub fn candidate(&self) -> Option<&BridgeValidatedWritebackCandidate> {
        self.candidate.as_ref()
    }

    pub fn counters(&self) -> &BridgeAsyncWritebackCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
