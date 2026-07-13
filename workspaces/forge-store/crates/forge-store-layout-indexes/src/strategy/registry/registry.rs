use super::{
    LayoutAdmissionDenial, LayoutAdmissionRequest, LayoutRequestedCapability,
    LayoutStrategyCapability, LayoutStrategyRegistrySnapshot,
};
use crate::keyspace::{CompositeKeyOrderingLaw, HashCollisionLaw};
use crate::maintenance::{IndexMaintenanceMode, PhysicalMutationShape};
use crate::strategy::{admit_strategy_from_basis, AdmittedLayoutStrategy, LayoutStrategyFamily};

#[derive(Debug, PartialEq, Eq)]
enum LayoutAdmissionPayload {
    Success(LayoutStrategyRegistrySnapshot),
    Denied(LayoutAdmissionDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutAdmissionOutcome {
    case: LayoutAdmissionPayload,
}

impl LayoutAdmissionOutcome {
    pub(crate) fn success(value: LayoutStrategyRegistrySnapshot) -> Self {
        Self::from_owner_payload(LayoutAdmissionPayload::Success(value))
    }

    pub(crate) fn denied(value: LayoutAdmissionDenial) -> Self {
        Self::from_owner_payload(LayoutAdmissionPayload::Denied(value))
    }

    fn from_owner_payload(case: LayoutAdmissionPayload) -> Self {
        Self { case }
    }

    fn into_owner_payload(self) -> LayoutAdmissionPayload {
        self.case
    }
}

impl LayoutAdmissionOutcome {
    pub fn into_result(self) -> Result<LayoutStrategyRegistrySnapshot, LayoutAdmissionDenial> {
        match self.into_owner_payload() {
            LayoutAdmissionPayload::Success(value) => Ok(value),
            LayoutAdmissionPayload::Denied(denial) => Err(denial),
        }
    }
    #[cfg(test)]
    pub fn unwrap(self) -> LayoutStrategyRegistrySnapshot {
        self.into_result().unwrap()
    }
    #[cfg(test)]
    pub fn unwrap_err(self) -> LayoutAdmissionDenial {
        self.into_result().unwrap_err()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutAdmissionRegistryFacade;

impl LayoutAdmissionRegistryFacade {
    pub fn admit(&self, request: LayoutAdmissionRequest) -> LayoutAdmissionOutcome {
        match derive_registry_snapshot(request) {
            Ok(snapshot) => LayoutAdmissionOutcome::success(snapshot),
            Err(denial) => LayoutAdmissionOutcome::denied(denial),
        }
    }
}

fn derive_registry_snapshot(
    request: LayoutAdmissionRequest,
) -> Result<LayoutStrategyRegistrySnapshot, LayoutAdmissionDenial> {
    let admitted = match admit_strategy_from_basis(request.authority_basis(), request.family()) {
        Ok(admitted) => admitted,
        Err(denial) => {
            return Err(LayoutAdmissionDenial::StrategyVocabularyDenied(denial));
        }
    };

    if !family_lane_supports_operation(admitted.declared_access_lane(), request.requested_lane()) {
        return Err(LayoutAdmissionDenial::RequestedLaneDoesNotMatchFamilyLane {
            family: admitted.family(),
            requested_lane: request.requested_lane(),
            declared_lane: admitted.declared_access_lane(),
        });
    }

    if request.required_scope_partition() != request.key_domain().scope() {
        return Err(LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain {
            requested_scope: request.required_scope_partition(),
            key_domain_scope: request.key_domain().scope(),
        });
    }

    if !maintenance_mode_supports_lane(request.maintenance_mode(), request.requested_lane()) {
        return Err(
            LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane {
                family: admitted.family(),
                maintenance_mode: request.maintenance_mode(),
                requested_lane: request.requested_lane(),
            },
        );
    }

    if !mutation_shape_is_compatible(admitted.family(), request.mutation_shape()) {
        return Err(
            LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy {
                family: admitted.family(),
                mutation_shape: request.mutation_shape(),
            },
        );
    }

    if let Some(required_posture) = request.required_migration_posture() {
        if admitted.migration_posture() != required_posture {
            return Err(
                LayoutAdmissionDenial::MigrationPostureIncompatibleWithStrategy {
                    family: admitted.family(),
                    required_migration_posture: required_posture,
                    admitted_migration_posture: admitted.migration_posture(),
                },
            );
        }
    }

    require_materialization_compatibility(admitted, &request)?;

    if let Err(denial) = require_key_law_compatibility(admitted, &request) {
        return Err(denial);
    }

    let (hash_equality_law, composite_ordering_law) =
        match require_requested_key_law_compatibility(admitted, &request) {
            Ok(laws) => laws,
            Err(denial) => return Err(denial),
        };

    let granted_capability =
        LayoutStrategyCapability::from_requested(request.requested_capability());
    Ok(LayoutStrategyRegistrySnapshot::new(
        admitted,
        request,
        granted_capability,
        hash_equality_law,
        composite_ordering_law,
    ))
}

fn require_materialization_compatibility(
    admitted: AdmittedLayoutStrategy,
    request: &LayoutAdmissionRequest,
) -> Result<(), LayoutAdmissionDenial> {
    let strategy_family = admitted.admitted_family().declaration().family();
    let maintenance_witness = request.exact_maintenance_witness();

    if let Some(witness) = maintenance_witness {
        if witness.family() != strategy_family {
            return Err(
                LayoutAdmissionDenial::LiveExactMaintenanceWitnessDoesNotMatchStrategy {
                    witness_family: witness.family(),
                    strategy_family,
                },
            );
        }
        if let Some(requested_coverage) = request.exact_coverage() {
            if witness.exact_coverage() != requested_coverage {
                return Err(
                    LayoutAdmissionDenial::LiveExactMaintenanceCoverageDoesNotMatchRequest {
                        witness_coverage: witness.exact_coverage().clone(),
                        requested_coverage: requested_coverage.clone(),
                    },
                );
            }
        }
    }

    if !request.requires_exact_materialization() && maintenance_witness.is_none() {
        return Ok(());
    }

    let coverage = request
        .exact_coverage()
        .or_else(|| maintenance_witness.map(|witness| witness.exact_coverage()))
        .ok_or(LayoutAdmissionDenial::ExactMaterializationRequired)?;
    if coverage.family() != strategy_family {
        return Err(LayoutAdmissionDenial::CoverageFamilyDoesNotMatchStrategy {
            coverage_family: coverage.family(),
            strategy_family,
        });
    }
    coverage
        .require_exact()
        .map_err(LayoutAdmissionDenial::ExactCoverageDenied)?;
    Ok(())
}

pub(crate) const fn family_lane_supports_operation(
    family_lane: crate::catalog::ArtifactFamilyAccessLane,
    operation_lane: crate::catalog::ArtifactFamilyAccessLane,
) -> bool {
    use crate::catalog::ArtifactFamilyAccessLane;

    match family_lane {
        ArtifactFamilyAccessLane::HotPath => matches!(
            operation_lane,
            ArtifactFamilyAccessLane::HotPath
                | ArtifactFamilyAccessLane::MaintenancePath
                | ArtifactFamilyAccessLane::TerminalPath
        ),
        ArtifactFamilyAccessLane::MaintenancePath => {
            matches!(operation_lane, ArtifactFamilyAccessLane::MaintenancePath)
        }
        ArtifactFamilyAccessLane::VerifierPath => {
            matches!(operation_lane, ArtifactFamilyAccessLane::VerifierPath)
        }
        ArtifactFamilyAccessLane::TerminalPath => {
            matches!(operation_lane, ArtifactFamilyAccessLane::TerminalPath)
        }
    }
}

pub const fn layout_admission_registry() -> LayoutAdmissionRegistryFacade {
    LayoutAdmissionRegistryFacade
}

fn require_key_law_compatibility(
    admitted: AdmittedLayoutStrategy,
    request: &LayoutAdmissionRequest,
) -> Result<(), LayoutAdmissionDenial> {
    let capability = request.requested_capability();
    if requires_comparator_law(capability) && admitted.comparator_law().is_none() {
        return Err(LayoutAdmissionDenial::ComparatorLawRequired {
            family: admitted.family(),
            capability,
        });
    }

    match capability {
        LayoutRequestedCapability::PointLookup if admitted.supports_point_access() => Ok(()),
        LayoutRequestedCapability::OrderedRange
            if admitted.supports_range_access()
                || admits_owned_maintenance_traversal(admitted, request) =>
        {
            if admitted.range_bound_law().is_none() {
                return Err(LayoutAdmissionDenial::RangeBoundLawRequired {
                    family: admitted.family(),
                });
            }
            Ok(())
        }
        LayoutRequestedCapability::PrefixTraversal if admitted.supports_prefix_access() => {
            if admitted.prefix_law().is_none() {
                return Err(LayoutAdmissionDenial::PrefixLawRequired {
                    family: admitted.family(),
                });
            }
            Ok(())
        }
        LayoutRequestedCapability::ExactScan
            if admitted.supports_scan_access()
                || admits_owned_maintenance_traversal(admitted, request) =>
        {
            Ok(())
        }
        LayoutRequestedCapability::BlobStreaming if admitted.supports_streaming_access() => Ok(()),
        _ => Err(
            LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability {
                family: admitted.family(),
                capability,
            },
        ),
    }
}

const fn admits_owned_maintenance_traversal(
    admitted: AdmittedLayoutStrategy,
    request: &LayoutAdmissionRequest,
) -> bool {
    use crate::catalog::ArtifactFamilyAccessLane;

    matches!(
        request.requested_lane(),
        ArtifactFamilyAccessLane::MaintenancePath
    ) && match admitted.family() {
        LayoutStrategyFamily::BaselineBTreeRange => matches!(
            request.maintenance_mode(),
            IndexMaintenanceMode::RebuildOnly
        ),
        LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            matches!(
                request.mutation_shape(),
                PhysicalMutationShape::CompactionRewrite
            ) || matches!(
                request.maintenance_mode(),
                IndexMaintenanceMode::RebuildOnly
            )
        }
        _ => false,
    }
}

const fn requires_comparator_law(capability: LayoutRequestedCapability) -> bool {
    !matches!(
        capability,
        LayoutRequestedCapability::ExactScan | LayoutRequestedCapability::BlobStreaming
    )
}

fn require_requested_key_law_compatibility(
    admitted: AdmittedLayoutStrategy,
    request: &LayoutAdmissionRequest,
) -> Result<(Option<HashCollisionLaw>, Option<CompositeKeyOrderingLaw>), LayoutAdmissionDenial> {
    let hash_equality_law = request.required_key_laws().hash_equality_law();
    if let Some(law) = hash_equality_law {
        if law.domain() != admitted.key_domain() {
            return Err(
                LayoutAdmissionDenial::HashEqualityLawDoesNotMatchKeyDomain {
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
                LayoutAdmissionDenial::CompositeOrderingLawDoesNotMatchKeyDomain {
                    requested_domain: law.domain(),
                    strategy_domain: admitted.key_domain(),
                },
            );
        }
    }

    Ok((hash_equality_law, composite_ordering_law))
}

const fn maintenance_mode_supports_lane(
    mode: IndexMaintenanceMode,
    lane: crate::catalog::ArtifactFamilyAccessLane,
) -> bool {
    mode.supports_lane(lane)
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
