use forge_query::facade::ForgeQueryBatchWriteReceipt;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_domain::TopologyQueryDomain;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::application::{
    ensure_declared_touched_basis_covers_sequence_before_write,
    finalize_graph_or_batch_receipt_closeout, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
    TopologyRetainedApplicationHandoff,
};
use crate::topology_operators::local_rewrites::boundary_wiring::ResolvedLoopSuccessorRewire;
use crate::topology_operators::{
    TopologyDeclaredMutationActionRef, TopologyDeclaredMutationSequence,
    TopologyMutationApplicationMode, TopologyMutationFamily,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn execute_composed_loop_successor_program<I>(
        &mut self,
        semantic_family_key: &'static str,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        mode: TopologyMutationApplicationMode,
        sequence: TopologyDeclaredMutationSequence,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
    where
        I: forge_query::facade::ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        ensure_declared_touched_basis_covers_sequence_before_write(
            &retained_handoff,
            &sequence,
            mode.clone(),
        )?;
        let rewires = sequence
            .members()
            .map(|contract| match contract.action_ref() {
                TopologyDeclaredMutationActionRef::RewireLoopSuccessor {
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                } => self.resolve_loop_successor_rewire(
                    bindings,
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                ),
                _ => Err(TopologyMutationApplicationError::UnsupportedFamilies(vec![
                    TopologyMutationFamily::RewireLoopSuccessor,
                ])),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let receipt: ForgeQueryBatchWriteReceipt = self.workspace.compose_graph(|graph| {
            for rewire in &rewires {
                add_verified_successor_retarget(graph, rewire)?;
            }
            Ok(())
        })?;
        finalize_graph_or_batch_receipt_closeout(
            self,
            retained_handoff,
            semantic_family_key,
            mode,
            &sequence,
            receipt,
            crate::projection::runtime_boundary::query_runtime::TopologyQueryMutationLaneExecutionShape::GraphComposition,
        )
    }
}

fn add_verified_successor_retarget(
    graph: &mut forge_query::facade::ForgeQueryGraphCompositionBuilder,
    rewire: &ResolvedLoopSuccessorRewire,
) -> Result<(), forge_query::facade::ForgeQueryRuntimeError> {
    let relation_kind = rewire.relation_kind.kind_name().to_string();
    let relation_kind_verify = relation_kind.clone();
    let relation_kind_update = relation_kind.clone();
    let authoritative_identity = rewire.authoritative_identity.clone();
    let successor_authoritative_identity = rewire.successor_authoritative_identity.clone();
    let verify_source = rewire.source_query_identity.clone();
    let verify_target = rewire.current_target_query_identity.clone();
    let update_source = rewire.source_query_identity.clone();
    let update_target = rewire.updated_target_query_identity.clone();
    let dependency_path = rewire.dependency_path.clone();
    graph.retarget_existing_verified(
        rewire.binding.clone(),
        |verify| {
            let verify = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                    TopologyNativeQueryRowField::TopologyKind
                        .set_on(verify, relation_kind_verify.clone()),
                    verify_source,
                ),
                verify_target,
            );
            if let Some(field) =
                dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
            {
                field.set_on(verify, relation_kind_verify.clone())
            } else {
                verify
            }
        },
        |update| {
            let update = update.continuity_rebind_existing_target(
                authoritative_identity,
                successor_authoritative_identity,
            );
            let update = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                    TopologyNativeQueryRowField::TopologyKind
                        .set_on(update, relation_kind_update.clone()),
                    update_source,
                ),
                update_target,
            );
            if let Some(field) =
                dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
            {
                field.set_on(update, relation_kind_update)
            } else {
                update
            }
        },
    )
}
