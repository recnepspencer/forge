use std::sync::Arc;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryBoundInvariantExecutionView, WorthQueryInvariantExecutionDenialKind,
    WorthQueryInvariantExecutionFailure, WorthQueryInvariantExecutionProvider,
    WorthQueryInvariantProviderVerdict, WorthQueryInvariantStateLoadAdmission,
    WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantStateLoadRequestView,
    WorthQueryInvariantStateLocator, WorthQueryInvariantStructuralCounters,
    WorthQueryInvariantVerdictAdmission, WorthQueryInvariantVerdictEvidence,
    WorthQueryProviderSessionView,
};

pub(super) struct WorthQueryInvariantWorkMint {
    _private: (),
}

impl WorthQueryInvariantExecutionProvider for Arc<WorthQueryPrimaryGraphProvider> {
    fn load_invariant_state(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        request: WorthQueryInvariantStateLoadRequestView<'_>,
        admission: WorthQueryInvariantStateLoadAdmission,
    ) -> Result<WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantExecutionFailure> {
        self.application_attempt_work.observe_invariant_state_load();
        let attempts = self
            .attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let staged = attempts
            .staged_attempt(session)
            .ok_or_else(|| closure_failure("provider session has no staged application attempt"))?;
        let expected = expected_locators(staged.overlay_facts())?;
        if staged.expected_step_count() != staged.overlay_facts().len()
            || request.locators() != expected.as_slice()
        {
            return Err(closure_failure(
                "invariant state-load plan does not close over the proposed effects",
            ));
        }
        admission.admit(
            format!("primary-invariant-load:{}", staged.overlay_identity()),
            expected.clone(),
            WorthQueryInvariantStructuralCounters::new(
                expected.len(),
                expected.len() as u64,
                staged.overlay_facts().len() as u64,
            ),
        )
    }

    fn execute_invariant(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        execution: WorthQueryBoundInvariantExecutionView<'_>,
        admission: WorthQueryInvariantVerdictAdmission,
    ) -> Result<WorthQueryInvariantProviderVerdict, WorthQueryInvariantExecutionFailure> {
        self.application_attempt_work.observe_invariant_execution();
        let material = self.invariant_candidate_material(session)?;
        let load_evidence = execution.state_load_evidence();
        material.validate_load(&execution, load_evidence)?;
        if self.take_skipped_invariant_owner_execution() {
            let evidence = WorthQueryInvariantVerdictEvidence::new(
                execution.requirement().slot(),
                "mutation-probe-skipped-relational-owner",
                load_evidence.identity(),
                1,
            )?;
            return admission.passed(evidence);
        }
        let owner_work = u64::try_from(material.expected.len()).map_err(|_| owner_failure())?;
        let candidate = self.validate_relational_candidate(material.batch, &material.branch)?;
        let summary = candidate.invariant_evidence().summary();
        let work = super::mutation_work::WorthQueryPrimaryMutationWorkCounters::new(
            WorthQueryInvariantWorkMint { _private: () },
            material.decision_facts,
            material.expected.len(),
            material.expected.len(),
            owner_work,
            summary.execution_count,
            summary.result_count,
        );
        self.retain_validated_candidate(session, candidate, work)?;
        let evidence = WorthQueryInvariantVerdictEvidence::new(
            execution.requirement().slot(),
            "relational-installed-invariant-authority",
            load_evidence.identity(),
            owner_work,
        )?;
        admission.passed(evidence)
    }
}

struct ApplicationInvariantCandidateMaterial {
    expected: Vec<WorthQueryInvariantStateLocator>,
    load_evidence: String,
    batch: worth_relational::facade::transactions::WorkerIntentBatch,
    branch: worth_relational::facade::history::BranchId,
    decision_facts: usize,
}

impl ApplicationInvariantCandidateMaterial {
    fn validate_load(
        &self,
        execution: &WorthQueryBoundInvariantExecutionView<'_>,
        evidence: crate::domain_computation::WorthQueryInvariantStateLoadEvidenceView<'_>,
    ) -> Result<(), WorthQueryInvariantExecutionFailure> {
        (execution.state_load_plan().locators() == self.expected.as_slice()
            && evidence.loaded_fact_locators() == self.expected.as_slice()
            && evidence.physical_load_evidence() == self.load_evidence)
            .then_some(())
            .ok_or_else(|| {
                closure_failure(
                    "invariant execution evidence differs from the exact proposed state",
                )
            })
    }
}

impl WorthQueryPrimaryGraphProvider {
    fn invariant_candidate_material(
        &self,
        session: WorthQueryProviderSessionView<'_>,
    ) -> Result<ApplicationInvariantCandidateMaterial, WorthQueryInvariantExecutionFailure> {
        let attempts = self
            .attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let staged = attempts
            .staged_attempt(session)
            .ok_or_else(|| closure_failure("invariant execution lost its staged attempt"))?;
        if staged.expected_step_count() != staged.overlay_facts().len() {
            return Err(closure_failure(
                "invariant execution lost proposed-effect closure",
            ));
        }
        Ok(ApplicationInvariantCandidateMaterial {
            expected: expected_locators(staged.overlay_facts())?,
            load_evidence: format!("primary-invariant-load:{}", staged.overlay_identity()),
            batch: staged.batch().clone(),
            branch: staged.branch().clone(),
            decision_facts: staged.decision_fact_count(),
        })
    }

    fn validate_relational_candidate(
        &self,
        batch: worth_relational::facade::transactions::WorkerIntentBatch,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Result<
        worth_relational::facade::transactions::ValidatedRelationalMutation,
        WorthQueryInvariantExecutionFailure,
    > {
        let batch = if self.take_relational_invariant_violation() {
            batch.push(invariant_violation_probe())
        } else {
            batch
        };
        let candidate = self.graph.with_runtime_mut(|runtime| {
            let identity = runtime
                .branch_identity(branch)
                .map_err(|_| owner_failure())?;
            let options = runtime
                .transaction_options_for(&identity)
                .map_err(|_| owner_failure())?;
            let mut transaction = runtime.begin_transaction(options);
            transaction.push_batch(batch);
            Ok::<_, WorthQueryInvariantExecutionFailure>(transaction.validate())
        });
        let candidate = candidate
            .map_err(|error| error)?
            .map_err(|_| owner_failure())?;
        validate_owner_evidence(candidate.invariant_evidence(), branch)?;
        Ok(candidate)
    }

    fn retain_validated_candidate(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
        work: super::mutation_work::WorthQueryPrimaryMutationWorkCounters,
    ) -> Result<(), WorthQueryInvariantExecutionFailure> {
        self.attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .retain_invariant_approved(session, candidate, work)
            .map_err(closure_failure)
    }
}

fn expected_locators(
    facts: &[crate::domain_computation::WorthQueryProposedFact],
) -> Result<Vec<WorthQueryInvariantStateLocator>, WorthQueryInvariantExecutionFailure> {
    let mut expected = facts
        .iter()
        .map(|fact| {
            WorthQueryInvariantStateLocator::new("application-proposed-state", fact.identity())
        })
        .collect::<Result<Vec<_>, _>>()?;
    expected.sort();
    let original_len = expected.len();
    expected.dedup();
    if expected.len() != original_len {
        return Err(closure_failure(
            "proposed effects contain duplicate invariant fact identities",
        ));
    }
    Ok(expected)
}

fn validate_owner_evidence(
    evidence: &worth_relational::facade::transactions::RelationalMutationInvariantEvidence,
    branch: &worth_relational::facade::history::BranchId,
) -> Result<(), WorthQueryInvariantExecutionFailure> {
    if evidence.branch() != branch {
        return Err(closure_failure(
            "Relational invariant evidence belongs to a different branch",
        ));
    }
    let summary = evidence.summary();
    (summary.execution_count == 3
        && summary.commit_boundary_seen
        && summary.mutation_sensitive_seen
        && summary.snapshot_publication_seen)
        .then_some(())
        .ok_or_else(owner_failure)
}

fn invariant_violation_probe() -> worth_relational::facade::transactions::MutationIntent {
    use worth_relational::facade::{identity, transactions};

    transactions::MutationIntent::Create(transactions::CreateIntent::Relation(
        transactions::RelationSpec {
            partition_id: identity::PartitionId::main(),
            kind_id: identity::KindId::new(u32::MAX),
            client_key: worth_relational::facade::symbols::ClientKey::raw(
                "invariant-mutation-probe",
            ),
            source: transactions::EntityReference::Existing(identity::EntityId::new(
                identity::PartitionId::main(),
                u64::MAX - 1,
                1,
            )),
            target: transactions::EntityReference::Existing(identity::EntityId::new(
                identity::PartitionId::main(),
                u64::MAX,
                1,
            )),
            fields: transactions::AspectFieldPatch::default(),
        },
    ))
}

fn closure_failure(detail: &'static str) -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::new(
        WorthQueryInvariantExecutionDenialKind::StateLoadClosureMismatch,
        detail,
    )
}

fn owner_failure() -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::new(
        WorthQueryInvariantExecutionDenialKind::ProviderRejected,
        "Relational rejected the installed proposed-state invariant",
    )
}
