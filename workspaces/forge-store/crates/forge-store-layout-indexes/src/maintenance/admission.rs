use crate::maintenance::{
    ExactPublicationAuthoritySource, IndexMaintenanceFailureOutcome, IndexMaintenanceMode,
    IndexPublicationProtocol, LayoutMutationAdmissionOutcome, LayoutMutationPlan,
    LiveMaintenanceRequest, MutationProofRequirement, PhysicalMutationShape,
    PublicationProofRequirement,
};
use crate::strategy::{admit_strategy_from_basis, LayoutStrategyFamily, StrategyAuthorityBasis};

use super::entrypoint::LayoutMaintenanceFacade;

impl LayoutMaintenanceFacade {
    pub fn admit_mutation(
        &self,
        request: LiveMaintenanceRequest,
    ) -> LayoutMutationAdmissionOutcome {
        let admitted_strategy = match admit_strategy_from_basis(
            StrategyAuthorityBasis::admitted(
                request.admitted_family(),
                request.admitted_key_domain(),
            ),
            request.family(),
        ) {
            Ok(admitted) => admitted,
            Err(denial) => {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::StrategyDenied { denial },
                );
            }
        };

        if !request
            .maintenance_mode()
            .supports_lane(request.requested_lane())
        {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::MaintenanceModeIncompatibleWithRequestedLane {
                    family: admitted_strategy.family(),
                    maintenance_mode: request.maintenance_mode(),
                    requested_lane: request.requested_lane(),
                },
            );
        }

        if let Some(required_posture) = request.required_migration_posture() {
            if admitted_strategy.migration_posture() != required_posture {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::MigrationPostureIncompatibleWithStrategy {
                        family: admitted_strategy.family(),
                        required_migration_posture: required_posture,
                        admitted_migration_posture: admitted_strategy.migration_posture(),
                    },
                );
            }
        }

        if !mutation_shape_is_compatible(admitted_strategy.family(), request.mutation_shape()) {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::MutationShapeIncompatibleWithStrategy {
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
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::PublicationProtocolIncompatibleWithStrategy {
                    family: admitted_strategy.family(),
                    publication_protocol: request.publication_protocol(),
                },
            );
        }

        if request.maintenance_mode().permits_exact_answers() {
            let Some(coverage) = request.exact_coverage() else {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::ExactCoverageRequired {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            };

            if coverage.family() != admitted_strategy.lifecycle().declaration().family() {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::CoverageFamilyDoesNotMatchStrategy {
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
                        return LayoutMutationAdmissionOutcome::denied(
                            IndexMaintenanceFailureOutcome::ExactPublicationAuthorityRequired {
                                family: admitted_strategy.family(),
                                publication_protocol: request.publication_protocol(),
                            },
                        );
                    }
                    if !authority.supports_exact_coverage(coverage) {
                        return LayoutMutationAdmissionOutcome::denied(
                            IndexMaintenanceFailureOutcome::PublicationAuthorityDoesNotMatchExactCoverage {
                                publication_protocol: request.publication_protocol(),
                                coverage: coverage.clone(),
                            },
                        );
                    }
                }
                ExactPublicationRequirement::RequiredButMissing => {
                    return LayoutMutationAdmissionOutcome::denied(
                        IndexMaintenanceFailureOutcome::ExactPublicationAuthorityRequired {
                            family: admitted_strategy.family(),
                            publication_protocol: request.publication_protocol(),
                        },
                    );
                }
                ExactPublicationRequirement::Unsupported(missing) => {
                    return LayoutMutationAdmissionOutcome::denied(
                        IndexMaintenanceFailureOutcome::LowerPublicationCapabilityRequired {
                            family: admitted_strategy.family(),
                            publication_protocol: request.publication_protocol(),
                            missing,
                        },
                    );
                }
            }

            if request.lag_witness().is_some() {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::LagWitnessUnexpected {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            }
        } else if request.maintenance_mode().requires_lag_witness() {
            let Some(witness) = request.lag_witness() else {
                return LayoutMutationAdmissionOutcome::denied(
                    IndexMaintenanceFailureOutcome::LagWitnessRequired {
                        family: admitted_strategy.family(),
                        maintenance_mode: request.maintenance_mode(),
                    },
                );
            };

            if let Some(coverage) = request.exact_coverage() {
                if witness.coverage() != coverage {
                    return LayoutMutationAdmissionOutcome::denied(
                        IndexMaintenanceFailureOutcome::LagCoverageDoesNotMatchRequest {
                            expected: coverage.clone(),
                            actual: witness.coverage().clone(),
                        },
                    );
                }
            }
        }

        if let Some(missing) = missing_lower_mutation_capability(request.mutation_shape()) {
            return LayoutMutationAdmissionOutcome::denied(
                IndexMaintenanceFailureOutcome::LowerMutationCapabilityRequired {
                    family: admitted_strategy.family(),
                    mutation_shape: request.mutation_shape(),
                    missing,
                },
            );
        }

        let maintenance_mode = request.maintenance_mode();
        let lag_witness = request.lag_witness().cloned();
        let plan = LayoutMutationPlan::new(admitted_strategy, request);
        match maintenance_mode {
            IndexMaintenanceMode::SynchronousExact => LayoutMutationAdmissionOutcome::exact(plan),
            IndexMaintenanceMode::AsynchronousLagged => {
                LayoutMutationAdmissionOutcome::lagged(plan, lag_witness.unwrap())
            }
            IndexMaintenanceMode::RebuildOnly => {
                LayoutMutationAdmissionOutcome::rebuild(plan, lag_witness.unwrap())
            }
            IndexMaintenanceMode::LazyMaterializedOnDemand => {
                LayoutMutationAdmissionOutcome::lazy(plan, lag_witness.unwrap())
            }
            IndexMaintenanceMode::AdvisoryOnly => {
                LayoutMutationAdmissionOutcome::advisory(plan, lag_witness.unwrap())
            }
            IndexMaintenanceMode::VerifierOnly => {
                LayoutMutationAdmissionOutcome::verifier(plan, lag_witness.unwrap())
            }
            IndexMaintenanceMode::MigrationOnly => {
                LayoutMutationAdmissionOutcome::migration(plan, lag_witness.unwrap())
            }
        }
    }
}

const fn mutation_shape_is_compatible(
    family: LayoutStrategyFamily,
    mutation_shape: PhysicalMutationShape,
) -> bool {
    match (family, mutation_shape) {
        (_, PhysicalMutationShape::ObservationOnly) => true,
        (LayoutStrategyFamily::BaselineBTreeRange, PhysicalMutationShape::PointRewrite) => true,
        (
            LayoutStrategyFamily::BaselineLsmWriteOptimized,
            PhysicalMutationShape::LogStructuredAppend | PhysicalMutationShape::CompactionRewrite,
        ) => true,
        _ => false,
    }
}

const fn missing_lower_mutation_capability(
    mutation_shape: PhysicalMutationShape,
) -> Option<MutationProofRequirement> {
    if mutation_shape.requires_write_ordering_proof() {
        return Some(MutationProofRequirement::WalBeforeData);
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactPublicationRequirement {
    Admitted(ExactPublicationAuthoritySource),
    RequiredButMissing,
    Unsupported(PublicationProofRequirement),
}

fn exact_publication_requirement(
    protocol: IndexPublicationProtocol,
    authority: Option<ExactPublicationAuthoritySource>,
) -> ExactPublicationRequirement {
    match protocol {
        IndexPublicationProtocol::StableRootSwap => match authority {
            Some(authority)
                if authority.publication_protocol() == IndexPublicationProtocol::StableRootSwap =>
            {
                ExactPublicationRequirement::Admitted(authority)
            }
            None => ExactPublicationRequirement::RequiredButMissing,
            Some(_) => ExactPublicationRequirement::Unsupported(
                PublicationProofRequirement::RootEpochPublicationBinding,
            ),
        },
        IndexPublicationProtocol::StableManifestInstall => match authority {
            Some(authority)
                if authority.publication_protocol()
                    == IndexPublicationProtocol::StableManifestInstall =>
            {
                ExactPublicationRequirement::Admitted(authority)
            }
            None => ExactPublicationRequirement::RequiredButMissing,
            Some(_) => ExactPublicationRequirement::Unsupported(
                PublicationProofRequirement::ManifestPublicationValidation,
            ),
        },
        IndexPublicationProtocol::DeferredCatchUp
        | IndexPublicationProtocol::CompactionCutover
        | IndexPublicationProtocol::MigrationCutover
        | IndexPublicationProtocol::VerifierObservationOnly => {
            ExactPublicationRequirement::Unsupported(
                PublicationProofRequirement::RootPublicationValidation,
            )
        }
    }
}
