use sha2::{Digest, Sha256};
use worth_proof::{CanonicalVec, NonEmpty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegrityRepairRegionClass {
    DerivedRebuildable,
    AuthorityTrustedSourceRequired,
    ContentTrustedSourceRequired,
    QuarantineRequired,
    Indeterminate,
    Unrecoverable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegrityRepairArtifactFamily {
    Manifest,
    Page,
    Extent,
    Wal,
    LayoutIndex,
    BlobChunk,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegrityRepairOwnerBinding {
    family: IntegrityRepairArtifactFamily,
    observed_generation: Option<u64>,
    physical_owner_identity: Option<[u8; 32]>,
    security_scope_identity: Option<[u8; 32]>,
}

impl IntegrityRepairOwnerBinding {
    pub const fn observed(
        family: IntegrityRepairArtifactFamily,
        observed_generation: Option<u64>,
        physical_owner_identity: Option<[u8; 32]>,
        security_scope_identity: Option<[u8; 32]>,
    ) -> Self {
        Self {
            family,
            observed_generation,
            physical_owner_identity,
            security_scope_identity,
        }
    }
    pub const fn family(self) -> IntegrityRepairArtifactFamily {
        self.family
    }
    pub const fn observed_generation(self) -> Option<u64> {
        self.observed_generation
    }
    pub const fn physical_owner_identity(self) -> Option<[u8; 32]> {
        self.physical_owner_identity
    }
    pub const fn security_scope_identity(self) -> Option<[u8; 32]> {
        self.security_scope_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegrityRepairRegion {
    start: u64,
    end_exclusive: u64,
    identity: [u8; 32],
    class: IntegrityRepairRegionClass,
    evidence_digest: [u8; 32],
    target_identity: [u8; 32],
    owner_binding: IntegrityRepairOwnerBinding,
}

impl IntegrityRepairRegion {
    pub fn bounded(
        identity: [u8; 32],
        start: u64,
        end_exclusive: u64,
        class: IntegrityRepairRegionClass,
        evidence_digest: [u8; 32],
        target_identity: [u8; 32],
        owner_binding: IntegrityRepairOwnerBinding,
    ) -> Option<Self> {
        if identity == [0; 32]
            || evidence_digest == [0; 32]
            || target_identity == [0; 32]
            || start >= end_exclusive
        {
            None
        } else {
            Some(Self {
                start,
                end_exclusive,
                identity,
                class,
                evidence_digest,
                target_identity,
                owner_binding,
            })
        }
    }
    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }
    pub const fn class(self) -> IntegrityRepairRegionClass {
        self.class
    }
    pub const fn evidence_digest(self) -> [u8; 32] {
        self.evidence_digest
    }
    pub const fn target_identity(self) -> [u8; 32] {
        self.target_identity
    }
    pub const fn owner_binding(self) -> IntegrityRepairOwnerBinding {
        self.owner_binding
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityRepairClassificationDenial {
    EmptyRegions,
    DuplicateRegion,
    AmbiguousOverlap,
    AllocationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityRepairClassificationPlan {
    fingerprint: [u8; 32],
    regions: CanonicalVec<IntegrityRepairRegion>,
    non_empty: NonEmpty<[u8; 32]>,
}

impl IntegrityRepairClassificationPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    pub fn regions(&self) -> &[IntegrityRepairRegion] {
        self.regions.as_slice()
    }
    pub fn region_identities(&self) -> &[[u8; 32]] {
        self.non_empty.as_slice()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityRepairClassificationReceipt {
    plan_fingerprint: [u8; 32],
    classified_regions: u64,
    quarantined_regions: u64,
}

impl IntegrityRepairClassificationReceipt {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn classified_regions(self) -> u64 {
        self.classified_regions
    }
    pub const fn quarantined_regions(self) -> u64 {
        self.quarantined_regions
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntegrityOperationalRepairOwner;

impl IntegrityOperationalRepairOwner {
    pub fn lower(
        mut regions: Vec<IntegrityRepairRegion>,
    ) -> Result<IntegrityRepairClassificationPlan, IntegrityRepairClassificationDenial> {
        if regions.is_empty() {
            return Err(IntegrityRepairClassificationDenial::EmptyRegions);
        }
        regions.sort();
        if regions
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(IntegrityRepairClassificationDenial::DuplicateRegion);
        }
        let identities = regions
            .iter()
            .map(|region| region.identity)
            .collect::<Vec<_>>();
        let non_empty = NonEmpty::try_from_vec(identities)
            .map_err(|_| IntegrityRepairClassificationDenial::EmptyRegions)?;
        let mut digest = Sha256::new();
        digest.update(b"worth-store-integrity-repair-classification-plan-v1");
        for region in &regions {
            digest.update(region.identity);
            digest.update(region.start.to_be_bytes());
            digest.update(region.end_exclusive.to_be_bytes());
            digest.update([class_tag(region.class)]);
            digest.update(region.evidence_digest);
            digest.update(region.target_identity);
            digest.update([family_tag(region.owner_binding.family)]);
            digest.update(
                region
                    .owner_binding
                    .observed_generation
                    .unwrap_or(0)
                    .to_be_bytes(),
            );
            digest.update(
                region
                    .owner_binding
                    .physical_owner_identity
                    .unwrap_or([0; 32]),
            );
            digest.update(
                region
                    .owner_binding
                    .security_scope_identity
                    .unwrap_or([0; 32]),
            );
        }
        Ok(IntegrityRepairClassificationPlan {
            fingerprint: digest.finalize().into(),
            regions: CanonicalVec::try_from_sorted(regions)
                .map_err(|_| IntegrityRepairClassificationDenial::AllocationFailed)?,
            non_empty,
        })
    }

    pub fn execute(
        plan: &IntegrityRepairClassificationPlan,
    ) -> IntegrityRepairClassificationReceipt {
        IntegrityRepairClassificationReceipt {
            plan_fingerprint: plan.fingerprint,
            classified_regions: plan.regions.as_slice().len() as u64,
            quarantined_regions: plan
                .regions
                .as_slice()
                .iter()
                .filter(|region| region.class == IntegrityRepairRegionClass::QuarantineRequired)
                .count() as u64,
        }
    }
}

const fn family_tag(family: IntegrityRepairArtifactFamily) -> u8 {
    match family {
        IntegrityRepairArtifactFamily::Manifest => 1,
        IntegrityRepairArtifactFamily::Page => 2,
        IntegrityRepairArtifactFamily::Extent => 3,
        IntegrityRepairArtifactFamily::Wal => 4,
        IntegrityRepairArtifactFamily::LayoutIndex => 5,
        IntegrityRepairArtifactFamily::BlobChunk => 6,
        IntegrityRepairArtifactFamily::Unknown => 7,
    }
}

const fn class_tag(class: IntegrityRepairRegionClass) -> u8 {
    match class {
        IntegrityRepairRegionClass::DerivedRebuildable => 1,
        IntegrityRepairRegionClass::AuthorityTrustedSourceRequired => 2,
        IntegrityRepairRegionClass::ContentTrustedSourceRequired => 3,
        IntegrityRepairRegionClass::QuarantineRequired => 4,
        IntegrityRepairRegionClass::Indeterminate => 5,
        IntegrityRepairRegionClass::Unrecoverable => 6,
    }
}
