use super::{
    S8LayoutAdmissionDenial, S8LayoutAdmissionRequest, S8LayoutRequestedCapability,
    S8LayoutStrategyCapability, S8LayoutStrategyRegistrySnapshot,
};
use crate::key_domain::{CompositeKeyOrderingLaw, HashCollisionLaw};
use crate::maintenance::{S8IndexMaintenanceMode, S8PhysicalMutationShape};
use crate::production_transition::define_owner_outcome;
use crate::strategy::{admit_strategy, S8AdmittedLayoutStrategy, S8LayoutStrategyFamily};

define_owner_outcome!(
    pub S8LayoutAdmissionOutcome,
    pub S8LayoutAdmissionView,
    S8LayoutAdmissionPayload,
    LayoutAdmission,
    AdmitLayoutStrategy,
    [
        success => Success(S8LayoutStrategyRegistrySnapshot): Declared => Admit => Admitted,
        denied => Denied(S8LayoutAdmissionDenial): Declared => Deny => Denied,
    ]
);

impl S8LayoutAdmissionOutcome {
    pub fn into_result(self) -> Result<S8LayoutStrategyRegistrySnapshot, S8LayoutAdmissionDenial> {
        match self.into_owner_payload() {
            S8LayoutAdmissionPayload::Success(value) => Ok(value),
            S8LayoutAdmissionPayload::Denied(denial) => Err(denial),
        }
    }
    pub fn unwrap(self) -> S8LayoutStrategyRegistrySnapshot {
        self.into_result().unwrap()
    }
    pub fn unwrap_err(self) -> S8LayoutAdmissionDenial {
        self.into_result().unwrap_err()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutAdmissionRegistryFacade;

impl LayoutAdmissionRegistryFacade {
    pub fn admit(&self, request: S8LayoutAdmissionRequest) -> S8LayoutAdmissionOutcome {
        match derive_registry_snapshot(request) {
            Ok(snapshot) => S8LayoutAdmissionOutcome::success(snapshot),
            Err(denial) => S8LayoutAdmissionOutcome::denied(denial),
        }
    }
}

fn derive_registry_snapshot(
    request: S8LayoutAdmissionRequest,
) -> Result<S8LayoutStrategyRegistrySnapshot, S8LayoutAdmissionDenial> {
    let admitted = match admit_strategy(request.lifecycle(), request.key_domain(), request.family())
    {
        Ok(admitted) => admitted,
        Err(denial) => {
            return Err(S8LayoutAdmissionDenial::StrategyVocabularyDenied(denial));
        }
    };

    if admitted.declared_access_lane() != request.requested_lane() {
        return Err(
            S8LayoutAdmissionDenial::RequestedLaneDoesNotMatchFamilyLane {
                family: admitted.family(),
                requested_lane: request.requested_lane(),
                declared_lane: admitted.declared_access_lane(),
            },
        );
    }

    if request.required_scope_partition() != request.key_domain().scope() {
        return Err(
            S8LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain {
                requested_scope: request.required_scope_partition(),
                key_domain_scope: request.key_domain().scope(),
            },
        );
    }

    if !maintenance_mode_supports_lane(request.maintenance_mode(), request.requested_lane()) {
        return Err(
            S8LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane {
                family: admitted.family(),
                maintenance_mode: request.maintenance_mode(),
                requested_lane: request.requested_lane(),
            },
        );
    }

    if !mutation_shape_is_compatible(admitted.family(), request.mutation_shape()) {
        return Err(
            S8LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy {
                family: admitted.family(),
                mutation_shape: request.mutation_shape(),
            },
        );
    }

    if let Some(required_posture) = request.required_migration_posture() {
        if admitted.migration_posture() != required_posture {
            return Err(
                S8LayoutAdmissionDenial::MigrationPostureIncompatibleWithStrategy {
                    family: admitted.family(),
                    required_migration_posture: required_posture,
                    admitted_migration_posture: admitted.migration_posture(),
                },
            );
        }
    }

    if let Err(denial) = require_key_law_compatibility(admitted, request.requested_capability()) {
        return Err(denial);
    }

    let (hash_equality_law, composite_ordering_law) =
        match require_requested_key_law_compatibility(admitted, request) {
            Ok(laws) => laws,
            Err(denial) => return Err(denial),
        };

    let granted_capability =
        S8LayoutStrategyCapability::from_requested(request.requested_capability());
    Ok(S8LayoutStrategyRegistrySnapshot::new(
        admitted,
        request,
        granted_capability,
        hash_equality_law,
        composite_ordering_law,
    ))
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
