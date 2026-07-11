use super::invariant_suite::S8AdmittedStrategyInvariants;
use super::{
    capability::S8StrategyCapability,
    posture::{
        S8StrategyAmplificationProfile, S8StrategyCorruptionIsolationBehavior,
        S8StrategyLocalityProfile, S8StrategyMaterializationPosture,
        S8StrategyRebuildSourceRequirement,
    },
    S8LayoutStrategyFamily, S8StrategyDeclaration, S8StrategyDenial, S8StrategyInvariantSuite,
};
use crate::access_shape::{S8AccessShapeDetail, S8PrefixBasis, S8RangeBasis};
use crate::artifact_family::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleAdmission,
    DurableArtifactMigrationPosture, DurableArtifactProjectionClass, DurableArtifactRebuildPosture,
};
use crate::budget::S8PlannedCounterEnvelope;
use crate::execution::S8AccessPathCounterSnapshot;
use crate::key_domain::{
    declare_comparator_law, require_canonical_key_encoding, require_prefix_law,
    require_range_bound_law, CanonicalKeyEncoding, ComparatorLaw, PhysicalKeyDomain,
    PhysicalKeyDomainWitness, PrefixLawWitness, RangeBoundLawWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AdmittedLayoutStrategy {
    declaration: S8StrategyDeclaration,
    invariants: S8AdmittedStrategyInvariants,
}

impl S8AdmittedLayoutStrategy {
    pub(crate) const fn new(
        declaration: S8StrategyDeclaration,
        invariants: S8AdmittedStrategyInvariants,
    ) -> Self {
        Self {
            declaration,
            invariants,
        }
    }

    pub const fn family(&self) -> S8LayoutStrategyFamily {
        self.declaration.family()
    }

    pub const fn invariant_suite(&self) -> S8StrategyInvariantSuite {
        self.invariants.suite()
    }

    pub const fn invariant_production_transition(
        &self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        self.invariants.production_transition()
    }

    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.declaration.key_domain()
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.declaration.lifecycle()
    }

    pub const fn supports_point_access(&self) -> bool {
        self.declaration.capability().supports_point()
    }

    pub const fn supports_range_access(&self) -> bool {
        self.declaration.capability().supports_range()
    }

    pub const fn supports_prefix_access(&self) -> bool {
        self.declaration.capability().supports_prefix()
    }

    pub const fn supports_scan_access(&self) -> bool {
        self.declaration.capability().supports_scan()
    }

    pub const fn supports_streaming_access(&self) -> bool {
        self.declaration.capability().supports_streaming()
    }

    pub const fn allows_access_lane(&self, lane: ArtifactFamilyAccessLane) -> bool {
        self.declaration.capability().allows_lane(lane)
    }

    pub const fn declared_access_lane(&self) -> ArtifactFamilyAccessLane {
        self.declaration.access_lane()
    }

    pub const fn authority_class(&self) -> ArtifactFamilyAuthorityClass {
        self.declaration.authority_class()
    }

    pub const fn locality_profile(&self) -> S8StrategyLocalityProfile {
        self.declaration.locality()
    }

    pub const fn amplification_profile(&self) -> S8StrategyAmplificationProfile {
        self.declaration.amplification()
    }

    pub const fn materialization_posture(&self) -> S8StrategyMaterializationPosture {
        self.declaration.materialization()
    }

    pub const fn rebuild_source_requirement(&self) -> S8StrategyRebuildSourceRequirement {
        self.declaration.rebuild_source()
    }

    pub const fn corruption_isolation_behavior(&self) -> S8StrategyCorruptionIsolationBehavior {
        self.declaration.corruption_isolation()
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.declaration.rebuild_posture()
    }

    pub const fn migration_posture(&self) -> DurableArtifactMigrationPosture {
        self.declaration.migration_posture()
    }

    pub fn supports_projection_class(&self, class: DurableArtifactProjectionClass) -> bool {
        self.declaration.projection_classes().contains(&class)
    }

    pub const fn supports_materialization_state(
        &self,
        state: crate::materialization::S8MaterializationStateClass,
    ) -> bool {
        self.declaration.materialization().supports_state(state)
    }

    pub const fn declared_counter_profile(&self) -> super::S8StrategyCounterProfile {
        match self.planned_counter_envelope() {
            Some(envelope) => envelope.aggregate_profile(),
            None => self
                .invariants
                .suite()
                .counter_evidence()
                .aggregate_profile(),
        }
    }

    pub const fn planned_counter_envelope(&self) -> Option<S8PlannedCounterEnvelope> {
        if family_requires_shape_specific_lookup_envelope(self.declaration.family()) {
            None
        } else {
            self.declaration.planned_counter_envelope()
        }
    }

    pub const fn planned_counter_envelope_for(
        &self,
        detail: S8AccessShapeDetail,
    ) -> Option<S8PlannedCounterEnvelope> {
        planned_counter_envelope_for(self.declaration.family(), detail)
    }

    pub const fn canonical_key_encoding(&self) -> Option<CanonicalKeyEncoding> {
        self.declaration.canonical_key_encoding()
    }

    pub const fn comparator_law(&self) -> Option<ComparatorLaw> {
        self.declaration.comparator_law()
    }

    pub const fn prefix_law(&self) -> Option<PrefixLawWitness> {
        self.declaration.prefix_law()
    }

    pub const fn range_bound_law(&self) -> Option<RangeBoundLawWitness> {
        self.declaration.range_bound_law()
    }
}

pub(crate) fn declare_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8StrategyDeclaration, S8StrategyDenial> {
    if lifecycle.family_id() != key_domain.family_id() {
        return Err(S8StrategyDenial::FamilyDoesNotMatchKeyDomain);
    }

    let domain = key_domain.domain();
    let supported = match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => matches!(
            domain,
            PhysicalKeyDomain::PageAddressKey
                | PhysicalKeyDomain::SegmentAddressKey
                | PhysicalKeyDomain::ExtentAddressKey
                | PhysicalKeyDomain::PhysicalReferenceKey
        ),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => matches!(
            domain,
            PhysicalKeyDomain::WalRecordKey | PhysicalKeyDomain::BlobIdentityKey
        ),
        _ => false,
    };

    if !supported {
        return Err(match family {
            S8LayoutStrategyFamily::BaselineBTreeRange => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineBTree
            }
            S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineLsm
            }
            _ => S8StrategyDenial::UnsupportedFamily,
        });
    }

    let family_declaration = lifecycle.declaration();
    let access_lane = family_declaration.access_lane();
    let authority_class = family_declaration.authority();
    let rebuild_posture = family_declaration.rebuild_posture();
    let migration_posture = family_declaration.migration_posture();
    let projection_classes = family_declaration.non_authority_projection_classes();
    let key_law = declare_key_law_posture(family, key_domain)?;
    let planned_counter_envelope = declare_planned_counter_envelope(family);

    let declaration = match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => S8StrategyDeclaration::new(
            lifecycle,
            key_domain,
            family,
            S8StrategyCapability::baseline_btree_range(),
            S8StrategyLocalityProfile::OrderedPageLocality,
            S8StrategyAmplificationProfile::SplitMergeBounded,
            S8StrategyMaterializationPosture::PublishedTreeLifecycle,
            S8StrategyRebuildSourceRequirement::PhysicalSnapshotReplay,
            S8StrategyCorruptionIsolationBehavior::PageScoped,
            access_lane,
            authority_class,
            rebuild_posture,
            migration_posture,
            projection_classes,
            Some(key_law.encoding),
            Some(key_law.comparator),
            key_law.prefix,
            key_law.range,
            planned_counter_envelope,
        ),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => S8StrategyDeclaration::new(
            lifecycle,
            key_domain,
            family,
            S8StrategyCapability::baseline_lsm_write_optimized(),
            S8StrategyLocalityProfile::BufferedRunLocality,
            S8StrategyAmplificationProfile::CompactionWriteAmplified,
            S8StrategyMaterializationPosture::WalBufferedRunLifecycle,
            S8StrategyRebuildSourceRequirement::WalReplay,
            S8StrategyCorruptionIsolationBehavior::RunScoped,
            access_lane,
            authority_class,
            rebuild_posture,
            migration_posture,
            projection_classes,
            Some(key_law.encoding),
            Some(key_law.comparator),
            None,
            None,
            planned_counter_envelope,
        ),
        _ => return Err(S8StrategyDenial::UnsupportedFamily),
    };

    if !declaration.capability().allows_lane(access_lane) {
        return Err(S8StrategyDenial::StrategyDoesNotSupportDeclaredAccessLane);
    }
    if !family_requires_shape_specific_lookup_envelope(declaration.family())
        && declaration.planned_counter_envelope().is_none()
    {
        return Err(S8StrategyDenial::StrategyDoesNotDeclarePlannedCounterEnvelope);
    }

    Ok(declaration)
}

pub(crate) fn admit_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8AdmittedLayoutStrategy, S8StrategyDenial> {
    let declaration = declare_strategy(lifecycle, key_domain, family)?;
    let invariants = S8StrategyInvariantSuite::declare(declaration).into_admitted()?;
    Ok(S8AdmittedLayoutStrategy::new(declaration, invariants))
}

pub(crate) const fn planned_counter_envelope_for(
    family: S8LayoutStrategyFamily,
    detail: S8AccessShapeDetail,
) -> Option<S8PlannedCounterEnvelope> {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => match detail {
            S8AccessShapeDetail::PointLookup => Some(baseline_btree_point_counter_envelope()),
            S8AccessShapeDetail::RangeLookup(S8RangeBasis::CanonicalRangeBounds) => {
                Some(baseline_btree_range_counter_envelope())
            }
            S8AccessShapeDetail::PrefixLookup(S8PrefixBasis::CanonicalPrefixBounds) => {
                Some(baseline_btree_prefix_counter_envelope())
            }
            _ => None,
        },
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => match detail {
            S8AccessShapeDetail::PointLookup => declare_planned_counter_envelope(family),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S8DeclaredKeyLawPosture {
    encoding: CanonicalKeyEncoding,
    comparator: ComparatorLaw,
    prefix: Option<PrefixLawWitness>,
    range: Option<RangeBoundLawWitness>,
}

fn declare_key_law_posture(
    family: S8LayoutStrategyFamily,
    key_domain: PhysicalKeyDomainWitness,
) -> Result<S8DeclaredKeyLawPosture, S8StrategyDenial> {
    let encoding = require_canonical_key_encoding(key_domain);
    let comparator = declare_comparator_law(encoding);
    let prefix = require_prefix_law(encoding).ok();
    let range = require_range_bound_law(comparator).ok();

    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => {
            let prefix = prefix.ok_or(S8StrategyDenial::RangeOrPrefixLawRequired)?;
            let range = range.ok_or(S8StrategyDenial::RangeOrPrefixLawRequired)?;
            Ok(S8DeclaredKeyLawPosture {
                encoding,
                comparator,
                prefix: Some(prefix),
                range: Some(range),
            })
        }
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => Ok(S8DeclaredKeyLawPosture {
            encoding,
            comparator,
            prefix: None,
            range: None,
        }),
        _ => Ok(S8DeclaredKeyLawPosture {
            encoding,
            comparator,
            prefix,
            range,
        }),
    }
}

const fn declare_planned_counter_envelope(
    family: S8LayoutStrategyFamily,
) -> Option<S8PlannedCounterEnvelope> {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => None,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => Some(S8PlannedCounterEnvelope::new(
            S8AccessPathCounterSnapshot::exact(
                1, 1, 0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 8_192, 0, 0, 2, 0,
            ),
            S8AccessPathCounterSnapshot::exact(
                0, 0, 0, 2, 2, 4, 0, 0, 0, 0, 0, 4, 16_384, 8_192, 2, 4, 2,
            ),
            S8AccessPathCounterSnapshot::exact(
                0, 0, 1, 0, 1, 2, 0, 0, 0, 0, 0, 1, 8_192, 0, 0, 2, 0,
            ),
        )),
        _ => None,
    }
}

pub(super) const fn planned_publication_counter_snapshot_for(
    family: S8LayoutStrategyFamily,
) -> S8AccessPathCounterSnapshot {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => baseline_btree_publication_snapshot(),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            declare_planned_counter_envelope(family)
                .expect("LSM strategy declares planned counter envelope")
                .publication()
        }
        _ => S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    }
}

pub(super) const fn planned_recovery_counter_snapshot_for(
    family: S8LayoutStrategyFamily,
) -> S8AccessPathCounterSnapshot {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => baseline_btree_recovery_snapshot(),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            declare_planned_counter_envelope(family)
                .expect("LSM strategy declares planned counter envelope")
                .recovery()
        }
        _ => S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    }
}

const fn family_requires_shape_specific_lookup_envelope(family: S8LayoutStrategyFamily) -> bool {
    matches!(family, S8LayoutStrategyFamily::BaselineBTreeRange)
}

const fn baseline_btree_point_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(1, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

const fn baseline_btree_range_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

const fn baseline_btree_prefix_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 2, 2, 2, 0, 1, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

pub(super) const fn baseline_btree_publication_snapshot() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 4_096, 4_096, 1, 1, 1)
}

pub(super) const fn baseline_btree_recovery_snapshot() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 4_096, 0, 0, 1, 0)
}
