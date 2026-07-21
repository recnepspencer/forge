use std::any::TypeId;
use std::marker::PhantomData;

use crate::basis_lifecycle::BasisOperationLane;

mod graph_contract;

use super::authority_shape::WorthQueryBoundAuthorityShapeProofs;
use graph_contract::admit_graph_contract;

use super::{
    WorthQueryBoundAuthoritySet, WorthQueryBoundCommitPosture, WorthQueryBoundDomainOperation,
    WorthQueryBoundGraphParticipation, WorthQueryBoundRequiredDomain,
    WorthQueryBoundRuntimeProviders, WorthQueryInstalledOperatingWorld,
    WorthQueryOperationBindingCounters, WorthQueryOperationBindingDenial,
    WorthQueryOperationBindingDenialKind,
};
use crate::domain_installation::{
    WorthQueryGraphCommitPosture, WorthQueryInstalledDomainHandle,
    WorthQueryOperationEffectContract, WorthQueryOperationReversalContract,
    WorthQueryOperationTouchContract,
};

pub struct WorthQueryOperationFamilyView<'view, 'runtime, F, L: BasisOperationLane> {
    world: &'view WorthQueryInstalledOperatingWorld<'runtime, L>,
    _family: PhantomData<fn() -> F>,
}

impl<'view, 'runtime, F, L: BasisOperationLane>
    WorthQueryOperationFamilyView<'view, 'runtime, F, L>
{
    pub(super) fn new(world: &'view WorthQueryInstalledOperatingWorld<'runtime, L>) -> Self {
        Self {
            world,
            _family: PhantomData,
        }
    }

    pub fn bind<D: 'static, O: 'static>(
        &self,
        domain: &WorthQueryInstalledDomainHandle<D>,
        _operation_marker: O,
    ) -> Result<WorthQueryBoundDomainOperation<D, O, F, L>, WorthQueryOperationBindingDenial>
    where
        F: 'static,
    {
        let operation = self
            .world
            .runtime
            .resolve_installed_operation::<D, O, F>(domain)
            .map_err(|denial| {
                WorthQueryOperationBindingDenial::new(
                    match denial.kind() {
                        crate::domain_installation::WorthQueryInstalledDomainOperationLookupDenialKind::DomainAuthority => WorthQueryOperationBindingDenialKind::DomainAuthority,
                        crate::domain_installation::WorthQueryInstalledDomainOperationLookupDenialKind::OperationNotInstalled => WorthQueryOperationBindingDenialKind::OperationNotInstalled,
                    },
                    "installed operation lookup failed",
                    WorthQueryOperationBindingCounters {
                        authority_checks: 1,
                        operation_lookups: denial.counters().indexed_operation_lookups,
                        ..WorthQueryOperationBindingCounters::default()
                    },
                )
            })?;
        let bindings = self
            .world
            .runtime
            .installed_domain_execution_index()
            .domain_operation_graph_bindings(
                TypeId::of::<D>(),
                TypeId::of::<O>(),
                TypeId::of::<F>(),
            );
        let mut counters = WorthQueryOperationBindingCounters {
            authority_checks: 1,
            operation_lookups: 1,
            ..WorthQueryOperationBindingCounters::default()
        };
        let semantics = operation.definition().semantics();
        counters.conditional_lowering_lookups += 1;
        let conditional_nodes = self.world.runtime.conditional_nodes::<D, O, F>();
        let expected_conditional_count = semantics.conditional_nodes.len()
            + match &semantics.workflow {
                worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
                    workflow,
                ) => workflow
                    .stages()
                    .iter()
                    .map(|stage| stage.semantics().conditional_nodes.len())
                    .sum::<usize>(),
                worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => 0,
            };
        if conditional_nodes.len() != expected_conditional_count {
            return Err(WorthQueryOperationBindingDenial::new(
                WorthQueryOperationBindingDenialKind::ConditionalLoweringNotInstalled,
                "installed conditional lowering count differs from the portable declaration",
                counters,
            ));
        }
        let exact_conditional_set = conditional_nodes.iter().all(|node| {
            node.operation_identity == operation.definition().canonical_identity()
                && node.runtime_authority
                    == operation.domain_authority().runtime_authority().as_u64()
                && node.installation_generation == operation.installation_generation().ordinal()
                && crate::domain_installation::declared_node(
                    operation.definition(),
                    node.lowering.location(),
                )
                .is_some()
        });
        if !exact_conditional_set {
            return Err(WorthQueryOperationBindingDenial::new(
                WorthQueryOperationBindingDenialKind::ConditionalLoweringDrift,
                "conditional lowering drifted from its operation, runtime, generation, or workflow stage",
                counters,
            ));
        }
        let requires_primary_read = semantics.graph_reads.roles().iter().any(|read| {
            read.participation
                == crate::domain_installation::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
        });
        if requires_primary_read
            && self.world.basis.normalized().family()
                != crate::basis_lifecycle::BasisFamily::CurrentHead
        {
            return Err(WorthQueryOperationBindingDenial::new(
                WorthQueryOperationBindingDenialKind::BasisExecutionUnsupported,
                "installed primary reads currently lower only through an exact current-head basis",
                counters,
            ));
        }
        let mutation_authority_required = matches!(
            semantics.effects,
            WorthQueryOperationEffectContract::Declared { .. }
        ) || matches!(
            semantics.touches,
            WorthQueryOperationTouchContract::Declared { .. }
        );
        if mutation_authority_required
            && L::lane_name()
                != <crate::basis_lifecycle::MutationPreparationLaneWitness as BasisOperationLane>::lane_name()
        {
            return Err(WorthQueryOperationBindingDenial::new(
                WorthQueryOperationBindingDenialKind::BasisLaneInsufficient,
                "touch/effect operations require an admitted mutation-preparation basis lane",
                counters,
            ));
        }
        let mut graphs = Vec::with_capacity(bindings.len());
        for binding in bindings {
            counters.graph_binding_lookups += 1;
            let record = self
                .world
                .runtime
                .installed_graph_participation(binding.graph_marker)
                .map_err(|_| {
                    WorthQueryOperationBindingDenial::new(
                        WorthQueryOperationBindingDenialKind::GraphParticipationNotInstalled,
                        &binding.role,
                        counters,
                    )
                })?;
            if record.definition.role != binding.role {
                return Err(WorthQueryOperationBindingDenial::new(
                    WorthQueryOperationBindingDenialKind::GraphRoleMismatch,
                    &binding.role,
                    counters,
                ));
            }
            admit_graph_contract(&operation, &binding.role, &record, counters)?;
            graphs.push(WorthQueryBoundGraphParticipation {
                role: binding.role.clone(),
                record,
            });
        }
        let required_domain_bindings = self
            .world
            .runtime
            .installed_domain_execution_index()
            .domain_operation_required_domains(
                TypeId::of::<D>(),
                TypeId::of::<O>(),
                TypeId::of::<F>(),
            );
        let mut required_domains = Vec::with_capacity(required_domain_bindings.len());
        for binding in required_domain_bindings {
            counters.required_domain_lookups += 1;
            let authority = self
                .world
                .runtime
                .installed_domain_authority_by_marker(binding.domain_marker)
                .ok_or_else(|| {
                    WorthQueryOperationBindingDenial::new(
                        WorthQueryOperationBindingDenialKind::RequiredDomainNotInstalled,
                        &binding.role,
                        counters,
                    )
                })?;
            required_domains.push(WorthQueryBoundRequiredDomain {
                role: binding.role.clone(),
                authority,
            });
        }
        let shape_proofs =
            WorthQueryBoundAuthorityShapeProofs::admit(&mut graphs, &mut required_domains)
                .map_err(|_| {
                    WorthQueryOperationBindingDenial::new(
                        WorthQueryOperationBindingDenialKind::IncoherentAuthoritySet,
                        "bound graph and required-domain roles must be canonical and unique",
                        counters,
                    )
                })?;
        let commit_posture = admit_commit_posture(&operation, &graphs, counters)?;
        let executor = self.world.runtime.domain_operation_executor::<D, O, F>();
        let workflow_executor = self.world.runtime.workflow_stage_executor::<D, O, F>();
        let workflow_parallel_admission_provider = self
            .world
            .runtime
            .workflow_parallel_admission_provider::<D, O, F>();
        Ok(WorthQueryBoundDomainOperation::mint(
            operation,
            self.world.basis.clone(),
            WorthQueryBoundAuthoritySet {
                graph_participations: graphs,
                required_domains,
                commit_posture,
                shape_proofs,
            },
            self.world.runtime.consumer_support_profile().clone(),
            WorthQueryBoundRuntimeProviders {
                executor,
                workflow_executor,
                workflow_parallel_admission_provider,
                conditional_nodes,
            },
        ))
    }
}

fn admit_commit_posture<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    graphs: &[WorthQueryBoundGraphParticipation],
    counters: WorthQueryOperationBindingCounters,
) -> Result<WorthQueryBoundCommitPosture, WorthQueryOperationBindingDenial> {
    let semantics = operation.definition().semantics();
    let touched_roles = match &semantics.touches {
        WorthQueryOperationTouchContract::Declared { graph_roles, .. } => graph_roles.as_slice(),
        WorthQueryOperationTouchContract::NotRequired => &[],
    };
    let primary_graph_mutation = ((touched_roles.is_empty()
        || matches!(
            semantics.workflow,
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(_)
        ))
        && matches!(
            semantics.effects,
            WorthQueryOperationEffectContract::Declared { .. }
        ))
        || matches!(
            &semantics.touches,
            WorthQueryOperationTouchContract::Declared { graph_roles, .. }
                if graph_roles.iter().any(|role| {
                    semantics.graph_reads.roles().iter().any(|read| {
                        read.role == *role
                            && read.participation
                                == crate::domain_installation::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
                    })
                })
        );
    let mutating_graphs = graphs
        .iter()
        .filter(|graph| touched_roles.contains(&graph.role))
        .collect::<Vec<_>>();
    if mutating_graphs.is_empty() {
        return Ok(if primary_graph_mutation {
            WorthQueryBoundCommitPosture::Atomic
        } else {
            WorthQueryBoundCommitPosture::ReadOnly
        });
    }
    if primary_graph_mutation {
        return require_compensation(operation, counters);
    }
    let compensation_required = mutating_graphs.iter().any(|graph| {
        graph.record.definition.contract.commit
            == WorthQueryGraphCommitPosture::CompensationRequired
    });
    let atomic_count = mutating_graphs
        .iter()
        .filter(|graph| {
            graph.record.definition.contract.commit
                == WorthQueryGraphCommitPosture::AtomicAuthorityRequired
        })
        .count();
    let mut commit_authorities = mutating_graphs
        .iter()
        .filter_map(|graph| graph.record.commit_authority.as_ref())
        .map(std::sync::Arc::as_ref);
    let first = commit_authorities.next();
    let mismatch =
        first.is_some_and(|first| commit_authorities.any(|next| !std::ptr::eq(first, next)));
    let every_graph_shares_atomic_authority =
        atomic_count == mutating_graphs.len() && first.is_some() && !mismatch;
    if compensation_required || !every_graph_shares_atomic_authority {
        return require_compensation(operation, counters);
    }
    Ok(WorthQueryBoundCommitPosture::Atomic)
}

fn require_compensation<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    counters: WorthQueryOperationBindingCounters,
) -> Result<WorthQueryBoundCommitPosture, WorthQueryOperationBindingDenial> {
    match operation.definition().semantics().reversal {
        WorthQueryOperationReversalContract::Compensation { .. }
        | WorthQueryOperationReversalContract::CompensationWithPostcondition { .. } => {
            Ok(WorthQueryBoundCommitPosture::Compensated)
        }
        _ => Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::CompensationUndeclared,
            "primary and separate mutations or separate commit authorities require compensation",
            counters,
        )),
    }
}
