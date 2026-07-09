use super::{
    S8LayoutAdmissionDeferred, S8LayoutAdmissionDenial, S8LayoutAdmissionRequest,
    S8LayoutRequestedCapability, S8LayoutStrategyCapability, S8LayoutStrategyRegistrySnapshot,
};
use crate::key_domain::{CompositeKeyOrderingLaw, HashCollisionLaw};
use crate::maintenance::{S8IndexMaintenanceMode, S8PhysicalMutationShape};
use crate::materialization::S8PhysicalAbsenceProof;
use crate::strategy::{admit_strategy, S8AdmittedLayoutStrategy, S8LayoutStrategyFamily};
use worth_proof::TransitionOutcome;

pub type S8LayoutAdmissionOutcome =
    TransitionOutcome<S8AdmittedLayoutStrategy, S8LayoutAdmissionDenial, S8LayoutAdmissionDeferred>;
pub type S8LayoutRegistrySnapshotOutcome = TransitionOutcome<
    S8LayoutStrategyRegistrySnapshot,
    S8LayoutAdmissionDenial,
    S8LayoutAdmissionDeferred,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutAdmissionRegistryFacade;

impl LayoutAdmissionRegistryFacade {
    pub fn admit(&self, request: S8LayoutAdmissionRequest) -> S8LayoutAdmissionOutcome {
        match self.admit_with(request) {
            TransitionOutcome::Success(snapshot) => self.try_admit_ready(snapshot),
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::Deferred(deferred),
            TransitionOutcome::Stale(stale) => match stale {},
            TransitionOutcome::RebindRequired(rebind) => match rebind {},
            TransitionOutcome::Failed(failed) => match failed {},
        }
    }

    pub fn admit_with(&self, request: S8LayoutAdmissionRequest) -> S8LayoutRegistrySnapshotOutcome {
        let admitted =
            match admit_strategy(request.lifecycle(), request.key_domain(), request.family()) {
                Ok(admitted) => admitted,
                Err(denial) => {
                    return TransitionOutcome::denied(
                        S8LayoutAdmissionDenial::StrategyVocabularyDenied(denial),
                    );
                }
            };

        if admitted.declared_access_lane() != request.requested_lane() {
            return TransitionOutcome::denied(
                S8LayoutAdmissionDenial::RequestedLaneDoesNotMatchFamilyLane {
                    family: admitted.family(),
                    requested_lane: request.requested_lane(),
                    declared_lane: admitted.declared_access_lane(),
                },
            );
        }

        if request.required_scope_partition() != request.key_domain().scope() {
            return TransitionOutcome::denied(
                S8LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain {
                    requested_scope: request.required_scope_partition(),
                    key_domain_scope: request.key_domain().scope(),
                },
            );
        }

        if !maintenance_mode_supports_lane(request.maintenance_mode(), request.requested_lane()) {
            return TransitionOutcome::denied(
                S8LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane {
                    family: admitted.family(),
                    maintenance_mode: request.maintenance_mode(),
                    requested_lane: request.requested_lane(),
                },
            );
        }

        if !mutation_shape_is_compatible(admitted.family(), request.mutation_shape()) {
            return TransitionOutcome::denied(
                S8LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy {
                    family: admitted.family(),
                    mutation_shape: request.mutation_shape(),
                },
            );
        }

        if let Some(required_posture) = request.required_migration_posture() {
            if admitted.migration_posture() != required_posture {
                return TransitionOutcome::denied(
                    S8LayoutAdmissionDenial::MigrationPostureIncompatibleWithStrategy {
                        family: admitted.family(),
                        required_migration_posture: required_posture,
                        admitted_migration_posture: admitted.migration_posture(),
                    },
                );
            }
        }

        if let Err(denial) = require_key_law_compatibility(admitted, request.requested_capability())
        {
            return TransitionOutcome::denied(denial);
        }

        let (hash_equality_law, composite_ordering_law) =
            match require_requested_key_law_compatibility(admitted, request) {
                Ok(laws) => laws,
                Err(denial) => return TransitionOutcome::denied(denial),
            };

        let granted_capability =
            S8LayoutStrategyCapability::from_requested(request.requested_capability());
        TransitionOutcome::success(S8LayoutStrategyRegistrySnapshot::new(
            admitted,
            request,
            granted_capability,
            hash_equality_law,
            composite_ordering_law,
        ))
    }

    pub fn try_admit_ready(
        &self,
        snapshot: S8LayoutStrategyRegistrySnapshot,
    ) -> S8LayoutAdmissionOutcome {
        let request = snapshot.request();
        let admitted = snapshot.admitted_strategy();

        if request.requires_exact_materialization() && request.exact_coverage().is_none() {
            return TransitionOutcome::deferred(
                S8LayoutAdmissionDeferred::ExactCoverageEvidenceRequired {
                    family: admitted.family(),
                    capability: snapshot.granted_capability(),
                },
            );
        }

        if let Some(coverage) = request.exact_coverage() {
            let Some(maintenance_witness) = request.exact_maintenance_witness() else {
                return TransitionOutcome::deferred(
                    S8LayoutAdmissionDeferred::LiveExactMaintenanceWitnessRequired {
                        family: admitted.family(),
                        capability: snapshot.granted_capability(),
                    },
                );
            };

            if coverage.family() != admitted.lifecycle().declaration().family() {
                return TransitionOutcome::denied(
                    S8LayoutAdmissionDenial::CoverageFamilyDoesNotMatchStrategy {
                        coverage_family: coverage.family(),
                        strategy_family: admitted.lifecycle().declaration().family(),
                    },
                );
            }

            if maintenance_witness.family() != admitted.lifecycle().declaration().family() {
                return TransitionOutcome::denied(
                    S8LayoutAdmissionDenial::LiveExactMaintenanceWitnessDoesNotMatchStrategy {
                        witness_family: maintenance_witness.family(),
                        strategy_family: admitted.lifecycle().declaration().family(),
                    },
                );
            }

            if maintenance_witness.exact_coverage() != coverage {
                return TransitionOutcome::denied(
                    S8LayoutAdmissionDenial::LiveExactMaintenanceCoverageDoesNotMatchRequest {
                        witness_coverage: maintenance_witness.exact_coverage(),
                        requested_coverage: coverage,
                    },
                );
            }

            if !maintenance_witness
                .maintenance_mode()
                .permits_exact_answers()
            {
                return TransitionOutcome::deferred(
                    S8LayoutAdmissionDeferred::LiveExactMaintenanceWitnessRequired {
                        family: admitted.family(),
                        capability: snapshot.granted_capability(),
                    },
                );
            }

            if let Err(denial) = coverage.require_exact() {
                return TransitionOutcome::denied(S8LayoutAdmissionDenial::ExactCoverageDenied(
                    denial,
                ));
            }

            if request.requires_exact_absence_proof() {
                if let Err(denial) = S8PhysicalAbsenceProof::exact_index(coverage) {
                    return TransitionOutcome::denied(
                        S8LayoutAdmissionDenial::ExactAbsenceProofDenied(denial),
                    );
                }
            }
        }

        TransitionOutcome::success(admitted)
    }
}

pub const fn layout_admission_registry() -> LayoutAdmissionRegistryFacade {
    LayoutAdmissionRegistryFacade
}

fn require_key_law_compatibility(
    admitted: S8AdmittedLayoutStrategy,
    capability: S8LayoutRequestedCapability,
) -> Result<(), S8LayoutAdmissionDenial> {
    if requires_comparator_law(capability) && admitted.comparator_law().is_none() {
        return Err(S8LayoutAdmissionDenial::ComparatorLawRequired {
            family: admitted.family(),
            capability,
        });
    }

    match capability {
        S8LayoutRequestedCapability::PointLookup if admitted.supports_point_access() => Ok(()),
        S8LayoutRequestedCapability::OrderedRange if admitted.supports_range_access() => {
            if admitted.range_bound_law().is_none() {
                return Err(S8LayoutAdmissionDenial::RangeBoundLawRequired {
                    family: admitted.family(),
                });
            }
            Ok(())
        }
        S8LayoutRequestedCapability::PrefixTraversal if admitted.supports_prefix_access() => {
            if admitted.prefix_law().is_none() {
                return Err(S8LayoutAdmissionDenial::PrefixLawRequired {
                    family: admitted.family(),
                });
            }
            Ok(())
        }
        S8LayoutRequestedCapability::ExactScan if admitted.supports_scan_access() => Ok(()),
        S8LayoutRequestedCapability::BlobStreaming if admitted.supports_streaming_access() => {
            Ok(())
        }
        _ => Err(
            S8LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability {
                family: admitted.family(),
                capability,
            },
        ),
    }
}

const fn requires_comparator_law(capability: S8LayoutRequestedCapability) -> bool {
    !matches!(
        capability,
        S8LayoutRequestedCapability::ExactScan | S8LayoutRequestedCapability::BlobStreaming
    )
}

fn require_requested_key_law_compatibility(
    admitted: S8AdmittedLayoutStrategy,
    request: S8LayoutAdmissionRequest,
) -> Result<(Option<HashCollisionLaw>, Option<CompositeKeyOrderingLaw>), S8LayoutAdmissionDenial> {
    let hash_equality_law = request.required_key_laws().hash_equality_law();
    if let Some(law) = hash_equality_law {
        if law.domain() != admitted.key_domain() {
            return Err(
                S8LayoutAdmissionDenial::HashEqualityLawDoesNotMatchKeyDomain {
                    requested_domain: law.domain(),
                    strategy_domain: admitted.key_domain(),
                },
            );
        }
    }

    let composite_ordering_law = request.required_key_laws().composite_ordering_law();
    if let Some(law) = composite_ordering_law {
        if law.domain() != admitted.key_domain() {
            return Err(
                S8LayoutAdmissionDenial::CompositeOrderingLawDoesNotMatchKeyDomain {
                    requested_domain: law.domain(),
                    strategy_domain: admitted.key_domain(),
                },
            );
        }
    }

    Ok((hash_equality_law, composite_ordering_law))
}

const fn maintenance_mode_supports_lane(
    mode: S8IndexMaintenanceMode,
    lane: crate::artifact_family::ArtifactFamilyAccessLane,
) -> bool {
    mode.supports_lane(lane)
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
