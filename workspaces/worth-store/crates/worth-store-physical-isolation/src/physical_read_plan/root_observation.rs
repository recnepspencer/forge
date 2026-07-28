use crate::{
    CurrentPhysicalRoot, EpochComparisonScope, PhysicalEpochVector, PhysicalReadStabilityAuthority,
};

use super::{
    PhysicalReadPlanAdmissionDenial, PhysicalReadPlanRetryPosture, ProtectedPhysicalReference,
    ProtectedPhysicalReferenceSet, PublishedReaderHazard,
};

#[derive(Debug, Clone)]
pub struct ProtectedRootObservation {
    hazard: PublishedReaderHazard,
    observed: PostHazardRootObservation,
}

#[derive(Debug, Clone)]
pub struct ValidatedRootObservation {
    observation: ProtectedRootObservation,
    epoch_vector: PhysicalEpochVector,
    retry_posture: PhysicalReadPlanRetryPosture,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PostHazardRootObservation {
    observed: PhysicalEpochVector,
}

#[derive(Debug, Clone)]
pub struct PostProtectionPhysicalReadObservation {
    root: CurrentPhysicalRoot,
    protected_references: ProtectedPhysicalReferenceSet,
    hazard_barrier: super::PhysicalReadReachabilityBarrier,
}

impl ProtectedRootObservation {
    pub(crate) fn from_published_hazard(
        hazard: PublishedReaderHazard,
        observed: PostHazardRootObservation,
    ) -> Self {
        Self { hazard, observed }
    }

    pub fn validate(self) -> Result<ValidatedRootObservation, PhysicalReadPlanAdmissionDenial> {
        let root = self.hazard.intent().root();
        let mut builder =
            PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(root.scope()))
                .with_root(root.epoch())
                .with_manifest(root.manifest_epoch());
        for reference in self.hazard.intent().protected_references().references() {
            builder = add_reference_epoch(root, builder, *reference)?;
        }
        let expected = builder.seal()?;
        let comparison = expected.compare_against(self.observed.epoch_vector());
        match comparison.decision() {
            crate::EpochRetryDecision::Current => Ok(ValidatedRootObservation {
                epoch_vector: self.observed.epoch_vector(),
                retry_posture: PhysicalReadPlanRetryPosture::from(comparison.decision()),
                observation: self,
            }),
            crate::EpochRetryDecision::Retry | crate::EpochRetryDecision::RebindRequired => {
                Err(PhysicalReadPlanAdmissionDenial::StalePlan(
                    comparison
                        .into_stale_read_plan_denial()
                        .expect("non-current freshness has drift"),
                ))
            }
        }
    }
}

impl PostHazardRootObservation {
    pub(crate) fn from_published_hazard_authority(
        authority: &PhysicalReadStabilityAuthority,
        observed: PostProtectionPhysicalReadObservation,
    ) -> Result<Self, PhysicalReadPlanAdmissionDenial> {
        let root = observed.root();
        require_root_matches_authority(authority, root)?;
        let mut builder =
            PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(root.scope()))
                .with_root(root.epoch())
                .with_manifest(root.manifest_epoch());
        for reference in observed.protected_references().references() {
            builder = add_reference_epoch(root, builder, *reference)?;
        }
        Ok(Self {
            observed: builder.seal()?,
        })
    }

    pub const fn epoch_vector(self) -> PhysicalEpochVector {
        self.observed
    }
}

impl PostProtectionPhysicalReadObservation {
    pub fn from_authority_after_hazard_publication(
        authority: &PhysicalReadStabilityAuthority,
        published_hazard: &PublishedReaderHazard,
        root: CurrentPhysicalRoot,
        protected_references: ProtectedPhysicalReferenceSet,
    ) -> Result<Self, PhysicalReadPlanAdmissionDenial> {
        require_root_matches_authority(authority, root)?;
        Ok(Self {
            root,
            protected_references,
            hazard_barrier: published_hazard.reachability_barrier(),
        })
    }

    pub const fn root(&self) -> CurrentPhysicalRoot {
        self.root
    }

    pub const fn protected_references(&self) -> &ProtectedPhysicalReferenceSet {
        &self.protected_references
    }

    pub(crate) const fn hazard_barrier(&self) -> super::PhysicalReadReachabilityBarrier {
        self.hazard_barrier
    }
}

impl ValidatedRootObservation {
    pub(crate) fn hazard(&self) -> &PublishedReaderHazard {
        &self.observation.hazard
    }

    pub const fn epoch_vector(&self) -> PhysicalEpochVector {
        self.epoch_vector
    }

    pub const fn retry_posture(&self) -> PhysicalReadPlanRetryPosture {
        self.retry_posture
    }

    pub(crate) const fn reachability_barrier(&self) -> super::PhysicalReadReachabilityBarrier {
        self.observation.hazard.reachability_barrier()
    }
}

fn require_root_matches_authority(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
) -> Result<(), PhysicalReadPlanAdmissionDenial> {
    let expected = authority.root_epoch_basis();
    if expected.epoch().get() == root.epoch().get()
        && expected.manifest_epoch().get() == root.manifest_epoch().get()
    {
        Ok(())
    } else {
        Err(PhysicalReadPlanAdmissionDenial::AuthorityRootMismatch {
            expected_root: expected.epoch().get(),
            observed_root: root.epoch().get(),
            expected_manifest: expected.manifest_epoch().get(),
            observed_manifest: root.manifest_epoch().get(),
        })
    }
}

fn add_reference_epoch(
    root: CurrentPhysicalRoot,
    builder: crate::PhysicalEpochVectorBuilder,
    reference: ProtectedPhysicalReference,
) -> Result<crate::PhysicalEpochVectorBuilder, PhysicalReadPlanAdmissionDenial> {
    match super::footprint_ranges::latch_domain(reference) {
        worth_store_physical_format::PhysicalCellReuseDomain::Segment => Ok(builder.with_segment(
            root.admit_segment_publication_epoch(reference.current_generation())?
                .epoch(),
        )),
        worth_store_physical_format::PhysicalCellReuseDomain::ExtentAllocation
        | worth_store_physical_format::PhysicalCellReuseDomain::RecordExtentAllocation
        | worth_store_physical_format::PhysicalCellReuseDomain::FreeSpaceReuse => Ok(builder
            .with_extent(
                root.admit_extent_publication_epoch(reference.current_generation())?
                    .epoch(),
            )),
        worth_store_physical_format::PhysicalCellReuseDomain::Page
        | worth_store_physical_format::PhysicalCellReuseDomain::SlotAllocation => Ok(builder
            .with_page(
                root.admit_page_publication_epoch(reference.current_generation())?
                    .epoch(),
            )),
        worth_store_physical_format::PhysicalCellReuseDomain::RootPublication => Ok(builder),
    }
}
