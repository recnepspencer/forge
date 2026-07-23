use std::any::TypeId;
use std::marker::PhantomData;

use crate::basis_lifecycle::BasisOperationLane;

mod commit_posture;
mod conditional_inventory;
mod graph_contract;

use super::authority_shape::WorthQueryBoundAuthorityShapeProofs;
use commit_posture::admit_commit_posture;
use conditional_inventory::{
    admit_conditional_inventory, ConditionalInventoryAdmission, ConditionalInventoryOwner,
};
use graph_contract::admit_graph_contract;

use super::{
    WorthQueryBoundAuthoritySet, WorthQueryBoundDomainOperation, WorthQueryBoundGraphParticipation,
    WorthQueryBoundRequiredDomain, WorthQueryBoundRuntimeProviders,
    WorthQueryInstalledOperatingWorld, WorthQueryOperationBindingCounters,
    WorthQueryOperationBindingDenial, WorthQueryOperationBindingDenialKind,
};
use crate::domain_installation::{
    WorthQueryInstalledDomainHandle, WorthQueryOperationEffectContract,
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
                        authority_checks: denial.counters().authority_checks,
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
            authority_checks: operation.lookup_counters().authority_checks,
            operation_lookups: operation.lookup_counters().indexed_operation_lookups,
            graph_binding_lookups: operation.lookup_counters().graph_binding_lookups,
            ..WorthQueryOperationBindingCounters::default()
        };
        let semantics = operation.definition().semantics();
        counters.conditional_lowering_lookups += 1;
        let conditional_nodes = self.world.runtime.conditional_nodes::<D, O, F>();
        counters.conditional_lowerings_retained = conditional_nodes.len();
        let conditional_inventory = admit_conditional_inventory(
            operation.definition(),
            &conditional_nodes,
            ConditionalInventoryOwner {
                runtime_authority: operation.domain_authority().runtime_authority().as_u64(),
                installation_generation: operation.installation_generation().ordinal(),
            },
            &mut counters,
        );
        if !matches!(
            conditional_inventory,
            ConditionalInventoryAdmission::Admitted
        ) {
            let kind = match conditional_inventory {
                ConditionalInventoryAdmission::Missing => {
                    WorthQueryOperationBindingDenialKind::ConditionalLoweringNotInstalled
                }
                ConditionalInventoryAdmission::Drifted => {
                    WorthQueryOperationBindingDenialKind::ConditionalLoweringDrift
                }
                ConditionalInventoryAdmission::Admitted => unreachable!(),
            };
            return Err(WorthQueryOperationBindingDenial::new(
                kind,
                "installed conditional lowerings differ from the portable declaration or owner",
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
            counters.graph_participation_lookups += 1;
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
            admit_graph_contract(&operation, &binding.role, &record, &mut counters)?;
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
        counters.authority_shape_admissions += 1;
        let shape_proofs =
            WorthQueryBoundAuthorityShapeProofs::admit(&mut graphs, &mut required_domains)
                .map_err(|_| {
                    WorthQueryOperationBindingDenial::new(
                        WorthQueryOperationBindingDenialKind::IncoherentAuthoritySet,
                        "bound graph and required-domain roles must be canonical and unique",
                        counters,
                    )
                })?;
        counters.commit_posture_classifications += 1;
        let commit_posture = admit_commit_posture(&operation, &graphs, &mut counters)?;
        counters.planning_steps += 1;
        counters.executor_route_lookups += 1;
        let executor = self.world.runtime.domain_operation_executor::<D, O, F>();
        counters.workflow_executor_route_lookups += 1;
        let workflow_executor = self.world.runtime.workflow_stage_executor::<D, O, F>();
        counters.parallel_admission_route_lookups += 1;
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
            counters,
        ))
    }
}
