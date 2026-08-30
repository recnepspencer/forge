use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::world::supply_chain::{
    compile_supply_chain_baseline_with_custom_invariant, snapshot_for_supply_chain_identity,
    CompiledSupplyChainProgram, SupplyChainScale, SupplyChainWorldDefinition,
};
use worth_foundational::facade::{
    AspectKey, AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
    FieldKey, InternedString,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::identity::EntityId;
use worth_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion, CustomInvariantVerdict,
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, InvariantGroupSet,
    InvariantReportedRule,
};
use worth_relational::facade::transactions::{
    planned_single_field_locator, AspectFieldPatch, EntityMutationIntent, MutationIntent,
    TransactionCommitError, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

const RULE_ID: &str = "phase5.custom-proposed-aspect";

#[derive(Clone, Default)]
struct ProbeEvidence {
    target: Arc<Mutex<Option<EntityId>>>,
    prepared: Arc<Mutex<usize>>,
    evaluated: Arc<Mutex<usize>>,
}

impl ProbeEvidence {
    fn set_target(&self, target: EntityId) {
        *self.target.lock().expect("target lock") = Some(target);
    }

    fn target(&self) -> Option<EntityId> {
        *self.target.lock().expect("target lock")
    }

    fn prepared(&self) -> usize {
        *self.prepared.lock().expect("prepared lock")
    }

    fn evaluated(&self) -> usize {
        *self.evaluated.lock().expect("evaluated lock")
    }
}

#[test]
fn custom_commit_boundary_reads_the_same_transaction_proposed_aspect_state() {
    let definition = SupplyChainWorldDefinition::operating(SupplyChainScale::court())
        .expect("Court Supply Chain definition is valid");
    let program = CompiledSupplyChainProgram::compile(definition)
        .expect("Court Supply Chain program compiles");
    let evidence = ProbeEvidence::default();
    let registration = CustomInvariantRegistration::new(ProposedAspectStateProbe {
        evidence: evidence.clone(),
    })
    .expect("proposed-state probe registers");
    let world = compile_supply_chain_baseline_with_custom_invariant(program, registration)
        .expect("baseline commits with the inactive proposed-state probe");

    let target = world.handles.aurora_voyage().id;
    assert_snapshot_status(
        &world.runtime,
        &BranchId("main".to_owned()),
        target,
        "Planned",
    );
    evidence.set_target(target);
    let commit = commit_status_update(&world.runtime, BranchId("main".to_owned()), target);
    let commit = commit.expect("custom rule sees the proposed status in both phases");
    assert_snapshot_status(&world.runtime, &BranchId("main".to_owned()), target, "Held");

    assert_eq!(evidence.prepared(), 1);
    assert_eq!(evidence.evaluated(), 1);
    assert!(commit.invariant_executions().iter().any(|execution| {
        execution.results().iter().any(|result| {
            matches!(
                &result.rule,
                InvariantReportedRule::Custom(identity) if identity.rule_id.as_str() == RULE_ID
            )
        })
    }));
}

#[derive(Clone)]
struct ProposedAspectStateProbe {
    evidence: ProbeEvidence,
}

impl CustomInvariantRule for ProposedAspectStateProbe {
    type Scope = ();

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: worth_relational::facade::runtime::CustomInvariantRuleId::new(RULE_ID),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from("Phase 5 proposed aspect state probe"),
            operational: CustomInvariantOperationalMetadata {
                execution_point: InvariantExecutionPoint::CommitBoundary,
                groups: InvariantGroupSet::all(),
                cost_class: InvariantCostClass::Touched,
                failure_effect: InvariantFailureEffect::BlockCommit,
            },
        }
    }

    fn prepare_scope(
        &self,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Result<Self::Scope, CustomInvariantPreparationError> {
        if let Some(target) = self.evidence.target() {
            assert_status(
                planner
                    .committed_aspect_states()
                    .entity_aspect_state(target),
                "Planned",
            );
            assert_proposed_status(planner.aspect_states().entity_aspect_state(target));
            *self.evidence.prepared.lock().expect("prepared lock") += 1;
        }
        Ok(())
    }

    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
        _scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
        if let Some(target) = self.evidence.target() {
            assert_status(
                context
                    .committed_aspect_states()
                    .entity_aspect_state(target),
                "Planned",
            );
            assert_proposed_status(context.aspect_states().entity_aspect_state(target));
            let identity = context
                .provenance()
                .proposal_identity
                .expect("custom execution carries the owner-issued proposal identity");
            assert!(
                identity.proposed_version_id().0 > context.version_id().0,
                "proposal identity must name the new commit version"
            );
            *self.evidence.evaluated.lock().expect("evaluated lock") += 1;
        }
        Ok(CustomInvariantVerdict::Pass)
    }
}

fn assert_proposed_status(state: Option<&AuthoritativeRecordAspectState>) {
    assert_status(state, "Held");
}

fn assert_status(state: Option<&AuthoritativeRecordAspectState>, expected: &str) {
    let state = state.expect("custom view resolves the existing target");
    let aspect = AspectKey::new("status").expect("status aspect");
    let value = state.get(&aspect).expect("status aspect exists").view();
    let ContractValidatedAspectValueView::Scalar(AspectValue::String(InternedString::Raw(
        observed,
    ))) = value
    else {
        panic!("status aspect must be a scalar string");
    };
    assert_eq!(observed, expected);
}

fn assert_snapshot_status(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    branch: &BranchId,
    entity_id: EntityId,
    expected: &str,
) {
    let identity = runtime
        .branch_identity(branch)
        .expect("branch identity is owner-issued");
    let snapshot = snapshot_for_supply_chain_identity(runtime, &identity);
    let view = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("branch snapshot is readable");
    let record = view
        .entities()
        .iter()
        .find(|record| record.entity_id == entity_id)
        .expect("target remains in the branch snapshot");
    assert_status(record.authoritative_aspect_state.as_ref(), expected);
}

fn commit_status_update(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    branch: BranchId,
    entity_id: EntityId,
) -> Result<worth_relational::facade::transactions::CommitResult, TransactionCommitError> {
    let identity = runtime
        .branch_identity(&branch)
        .expect("branch identity is owner-issued");
    let options = runtime
        .admit_branch_basis(&identity)
        .expect("transaction authority is owner-issued");
    let locator = planned_single_field_locator(
        AspectKey::new("status").expect("status aspect"),
        FieldKey::new("status").expect("status field"),
    );
    let fields = AspectFieldPatch::new(BTreeMap::from([(
        locator,
        AspectValue::String(InternedString::Raw("Held".to_owned())),
    )]));
    let mut transaction = runtime
        .begin_branch_transaction(
            &options,
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("owner-admitted transaction context");
    transaction
        .push_batch(
            WorkerIntentBatch::new("phase5-proposed-status").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent { entity_id, fields }),
            )),
        )
        .unwrap();
    transaction.commit(runtime)
}
