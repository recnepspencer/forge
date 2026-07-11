use crate::maintenance::{
    layout_rebuild, S8ExactPublicationAuthoritySource, S8IndexLagOutcome,
    S8IndexMaintenanceFailureOutcome, S8IndexMaintenanceMode, S8IndexMaintenanceTransitionOutcome,
    S8LayoutMutationAdmissionOutcome, S8LayoutMutationPlan, S8LayoutRebuildFacade,
    S8LiveExactMaintenanceWitness, S8LiveMaintenanceRequest, S8LoweredMaintenanceProtocol,
    S8MutationProofRequirement, S8PhysicalMutationShape, S8PublicationProofRequirement,
};
use crate::strategy::{admit_strategy, S8LayoutStrategyFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMaintenanceFacade;

impl LayoutMaintenanceFacade {
    pub fn admit_mutation(
        &self,
        request: S8LiveMaintenanceRequest,
    ) -> S8LayoutMutationAdmissionOutcome {
        let admitted_strategy =
            match admit_strategy(request.lifecycle(), request.key_domain(), request.family()) {
                Ok(admitted) => admitted,
                Err(denial) => {
                    return S8LayoutMutationAdmissionOutcome::denied(
                        S8IndexMaintenanceFailureOutcome::StrategyDenied { denial },
                    );
                }
            };

        if !request
            .maintenance_mode()
            .supports_lane(request.requested_lane())
        {
            return S8LayoutMutationAdmissionOutcome::denied(
                S8IndexMaintenanceFailureOutcome::MaintenanceModeIncompatibleWithRequestedLane {
                    family: admitted_strategy.family(),
                    maintenance_mode: request.maintenance_mode(),
                    requested_lane: request.requested_lane(),
                },
            );
        }

        if let Some(required_posture) = request.required_migration_posture() {
            if admitted_strategy.migration_posture() != required_posture {
                return S8LayoutMutationAdmissionOutcome::denied(
                    S8IndexMaintenanceFailureOutcome::MigrationPostureIncompatibleWithStrategy {
                        family: admitted_strategy.family(),
                        required_migration_posture: required_posture,
                        admitted_migration_posture: admitted_strategy.migration_posture(),
                    },
                );
            }
        }

        if !mutation_shape_is_compatible(admitted_strategy.family(), request.mutation_shape()) {
            return S8LayoutMutationAdmissionOutcome::denied(
                S8IndexMaintenanceFailureOutcome::MutationShapeIncompatibleWithStrategy {
                    family: admitted_strategy.family(),
                    mutation_shape: request.mutation_shape(),
                },
            );
        }

        if !request
            .publication_protocol()
            .supports_mode(request.maintenance_mode())
            || !request
                .publication_protocol()
                .matches_invariant(admitted_strategy.invariant_suite().publication_invariant())
        {
            return S8LayoutMutationAdmissionOutcome::denied(
                S8IndexMaintenanceFailureOutcome::PublicationProtocolIncompatibleWithStrategy {
                    family: admitted_strategy.family(),
                    publication_protocol: request.publication_protocol(),
                },
            );
        }

        if request.maintenance_mode().permits_exact_answers() {
            let Some(coverage) = request.exact_coverage() else {
                return S8LayoutMutationAdmissionOutcome::denied(
                    S8IndexMaintenanceFailureOutcome::ExactCoverageRequired {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            };

            if coverage.family() != admitted_strategy.lifecycle().declaration().family() {
                return S8LayoutMutationAdmissionOutcome::denied(
                    S8IndexMaintenanceFailureOutcome::CoverageFamilyDoesNotMatchStrategy {
                        coverage_family: coverage.family(),
                        strategy_family: admitted_strategy.lifecycle().declaration().family(),
                    },
                );
            }

            match exact_publication_requirement(
                request.publication_protocol(),
                request.exact_publication_authority(),
            ) {
                ExactPublicationRequirement::Admitted(authority) => {
                    if authority.publication_protocol() != request.publication_protocol() {
                        return S8LayoutMutationAdmissionOutcome::denied(
                            S8IndexMaintenanceFailureOutcome::ExactPublicationAuthorityRequired {
                                family: admitted_strategy.family(),
                                publication_protocol: request.publication_protocol(),
                            },
                        );
                    }
                    if !authority.supports_exact_coverage(coverage) {
                        return S8LayoutMutationAdmissionOutcome::denied(
                            S8IndexMaintenanceFailureOutcome::PublicationAuthorityDoesNotMatchExactCoverage {
                                publication_protocol: request.publication_protocol(),
                                coverage,
                            },
                        );
                    }
                }
                ExactPublicationRequirement::RequiredButMissing => {
                    return S8LayoutMutationAdmissionOutcome::denied(
                        S8IndexMaintenanceFailureOutcome::ExactPublicationAuthorityRequired {
                            family: admitted_strategy.family(),
                            publication_protocol: request.publication_protocol(),
                        },
                    );
                }
                ExactPublicationRequirement::Unsupported(missing) => {
                    return S8LayoutMutationAdmissionOutcome::denied(
                        S8IndexMaintenanceFailureOutcome::LowerPublicationCapabilityRequired {
                            family: admitted_strategy.family(),
                            publication_protocol: request.publication_protocol(),
                            missing,
                        },
                    );
                }
            }

            if request.lag_witness().is_some() {
                return S8LayoutMutationAdmissionOutcome::denied(
                    S8IndexMaintenanceFailureOutcome::LagWitnessUnexpected {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            }
        } else if request.maintenance_mode().requires_lag_witness() {
            let Some(witness) = request.lag_witness() else {
                return S8LayoutMutationAdmissionOutcome::denied(
                    S8IndexMaintenanceFailureOutcome::LagWitnessRequired {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            };

            if let Some(coverage) = request.exact_coverage() {
                if witness.coverage() != coverage {
                    return S8LayoutMutationAdmissionOutcome::denied(
                        S8IndexMaintenanceFailureOutcome::LagCoverageDoesNotMatchRequest {
                            expected: coverage,
                            actual: witness.coverage(),
                        },
                    );
                }
            }
        }

        if let Some(missing) = missing_lower_mutation_capability(request.mutation_shape()) {
            return S8LayoutMutationAdmissionOutcome::denied(
                S8IndexMaintenanceFailureOutcome::LowerMutationCapabilityRequired {
                    family: admitted_strategy.family(),
                    mutation_shape: request.mutation_shape(),
                    missing,
                },
            );
        }

        let plan = S8LayoutMutationPlan::new(admitted_strategy, request);
        match request.maintenance_mode() {
            S8IndexMaintenanceMode::SynchronousExact => {
                S8LayoutMutationAdmissionOutcome::ready(plan)
            }
            S8IndexMaintenanceMode::AsynchronousLagged => {
                S8LayoutMutationAdmissionOutcome::lagged((plan, request.lag_witness().unwrap()))
            }
            S8IndexMaintenanceMode::RebuildOnly
            | S8IndexMaintenanceMode::LazyMaterializedOnDemand
            | S8IndexMaintenanceMode::AdvisoryOnly
            | S8IndexMaintenanceMode::VerifierOnly
            | S8IndexMaintenanceMode::MigrationOnly => {
                S8LayoutMutationAdmissionOutcome::deferred((plan, request.lag_witness().unwrap()))
            }
        }
    }

    pub fn lower_protocol(
        &self,
        plan: S8LayoutMutationPlan,
    ) -> S8IndexMaintenanceTransitionOutcome {
        let lowered = S8LoweredMaintenanceProtocol::new(plan);
        match plan.maintenance_mode() {
            S8IndexMaintenanceMode::SynchronousExact => {
                S8IndexMaintenanceTransitionOutcome::ready_exact(lowered)
            }
            S8IndexMaintenanceMode::AsynchronousLagged => {
                S8IndexMaintenanceTransitionOutcome::lagged(lowered)
            }
            S8IndexMaintenanceMode::RebuildOnly => {
                S8IndexMaintenanceTransitionOutcome::rebuild_only(lowered)
            }
            S8IndexMaintenanceMode::LazyMaterializedOnDemand => {
                S8IndexMaintenanceTransitionOutcome::deferred(lowered)
            }
            S8IndexMaintenanceMode::AdvisoryOnly => {
                S8IndexMaintenanceTransitionOutcome::advisory_only(lowered)
            }
            S8IndexMaintenanceMode::VerifierOnly => {
                S8IndexMaintenanceTransitionOutcome::verifier_only(lowered)
            }
            S8IndexMaintenanceMode::MigrationOnly => {
                S8IndexMaintenanceTransitionOutcome::migration_only(lowered)
            }
        }
    }

    pub fn inspect_lag(&self, lowered: &S8LoweredMaintenanceProtocol) -> S8IndexLagOutcome {
        match lowered.plan().lag_witness() {
            Some(witness) => S8IndexLagOutcome::Lagged(witness),
            None if lowered.plan().maintenance_mode().permits_exact_answers() => {
                S8IndexLagOutcome::Exact
            }
            None => S8IndexLagOutcome::NonExact(lowered.plan().maintenance_mode()),
        }
    }

    pub fn certify_live_exact(
        &self,
        lowered: &S8LoweredMaintenanceProtocol,
    ) -> Option<S8LiveExactMaintenanceWitness> {
        let plan = lowered.plan();
        let coverage = plan.exact_coverage()?;
        let publication_authority = plan.exact_publication_authority()?;
        if !plan.maintenance_mode().permits_exact_answers()
            || plan.lag_witness().is_some()
            || !publication_authority.supports_exact_coverage(coverage)
        {
            return None;
        }

        Some(S8LiveExactMaintenanceWitness::new(
            plan.admitted_strategy().lifecycle().declaration().family(),
            coverage,
            plan.maintenance_mode(),
            publication_authority,
        ))
    }

    pub const fn rebuild(&self) -> S8LayoutRebuildFacade {
        layout_rebuild()
    }
}

pub const fn layout_maintenance() -> LayoutMaintenanceFacade {
    LayoutMaintenanceFacade
}

const fn mutation_shape_is_compatible(
    family: S8LayoutStrategyFamily,
    mutation_shape: S8PhysicalMutationShape,
) -> bool {
    match (family, mutation_shape) {
        (_, S8PhysicalMutationShape::ObservationOnly) => true,
        (S8LayoutStrategyFamily::BaselineBTreeRange, S8PhysicalMutationShape::PointRewrite) => true,
        (
            S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
            S8PhysicalMutationShape::LogStructuredAppend
            | S8PhysicalMutationShape::CompactionRewrite,
        ) => true,
        _ => false,
    }
}

const fn missing_lower_mutation_capability(
    mutation_shape: S8PhysicalMutationShape,
) -> Option<S8MutationProofRequirement> {
    if mutation_shape.requires_write_ordering_proof() {
        return Some(S8MutationProofRequirement::WalBeforeData);
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactPublicationRequirement {
    Admitted(S8ExactPublicationAuthoritySource),
    RequiredButMissing,
    Unsupported(S8PublicationProofRequirement),
}

const fn exact_publication_requirement(
    protocol: crate::S8IndexPublicationProtocol,
    authority: Option<S8ExactPublicationAuthoritySource>,
) -> ExactPublicationRequirement {
    match protocol {
        crate::S8IndexPublicationProtocol::StableRootSwap => {
            let _ = authority;
            ExactPublicationRequirement::Unsupported(
                S8PublicationProofRequirement::RootEpochPublicationBinding,
            )
        }
        crate::S8IndexPublicationProtocol::StableManifestInstall => {
            ExactPublicationRequirement::Unsupported(
                S8PublicationProofRequirement::ManifestPublicationValidation,
            )
        }
        crate::S8IndexPublicationProtocol::DeferredCatchUp
        | crate::S8IndexPublicationProtocol::CompactionCutover
        | crate::S8IndexPublicationProtocol::MigrationCutover
        | crate::S8IndexPublicationProtocol::VerifierObservationOnly => {
            ExactPublicationRequirement::Unsupported(
                S8PublicationProofRequirement::RootPublicationValidation,
            )
        }
    }
}
