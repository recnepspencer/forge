use std::num::NonZeroU32;

use super::PhysicalWorkHostileTruthEvidenceDenial;
use crate::physical_runtime::record_serving::{
    PhysicalWorkArtifactBinding, PhysicalWorkEvidenceDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkHostileCurrentTruth {
    store: [u8; 16],
    generation: u64,
    records: u64,
    payload_bytes: u64,
    payload_digest: PhysicalWorkEvidenceDigest,
}

impl PhysicalWorkHostileCurrentTruth {
    pub fn new(
        store: [u8; 16],
        generation: u64,
        records: u64,
        payload_bytes: u64,
        payload_digest: PhysicalWorkEvidenceDigest,
    ) -> Result<Self, PhysicalWorkHostileTruthEvidenceDenial> {
        if store == [0; 16] {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::ZeroStoreIdentity);
        }
        Ok(Self {
            store,
            generation,
            records,
            payload_bytes,
            payload_digest,
        })
    }

    pub const fn store(self) -> [u8; 16] {
        self.store
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn records(self) -> u64 {
        self.records
    }

    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }

    pub const fn payload_digest(self) -> PhysicalWorkEvidenceDigest {
        self.payload_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkHostileArtifactEvidence {
    binding: PhysicalWorkArtifactBinding,
    prefix: Box<[u8]>,
    recovery_obligation: bool,
}

impl PhysicalWorkHostileArtifactEvidence {
    pub fn new(
        binding: PhysicalWorkArtifactBinding,
        prefix: impl Into<Box<[u8]>>,
        recovery_obligation: bool,
    ) -> Result<Self, PhysicalWorkHostileTruthEvidenceDenial> {
        let prefix = prefix.into();
        if prefix.len() as u64 > binding.byte_length() {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::PrefixExceedsArtifact);
        }
        let recovery_path = binding.path().starts_with("families/physical-work/")
            && binding.path().ends_with(".pending");
        if recovery_obligation != recovery_path {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::ArtifactRoleMismatch);
        }
        Ok(Self {
            binding,
            prefix,
            recovery_obligation,
        })
    }

    pub const fn binding(&self) -> &PhysicalWorkArtifactBinding {
        &self.binding
    }

    pub fn prefix(&self) -> &[u8] {
        &self.prefix
    }

    pub const fn is_recovery_obligation(&self) -> bool {
        self.recovery_obligation
    }

    pub fn is_mutation_coordination(&self) -> bool {
        self.binding.path() == "namespace/mutation.lock"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkFreshReopenIdentity {
    process: NonZeroU32,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
    records: u64,
}

impl PhysicalWorkFreshReopenIdentity {
    pub fn new(
        process: NonZeroU32,
        store: [u8; 16],
        runtime: u64,
        generation: u64,
        records: u64,
    ) -> Result<Self, PhysicalWorkHostileTruthEvidenceDenial> {
        if store == [0; 16] {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::ZeroStoreIdentity);
        }
        if runtime == 0 {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::ZeroRuntimeIdentity);
        }
        Ok(Self {
            process,
            store,
            runtime,
            generation,
            records,
        })
    }

    pub const fn process(self) -> NonZeroU32 {
        self.process
    }

    pub const fn store(self) -> [u8; 16] {
        self.store
    }

    pub const fn runtime(self) -> u64 {
        self.runtime
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn records(self) -> u64 {
        self.records
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkFreshReopenPosture {
    residue: bool,
    recovery_evidence_damaged: bool,
    recovery_obligations: u64,
    inspection_required: bool,
}

impl PhysicalWorkFreshReopenPosture {
    pub const fn new(
        residue: bool,
        recovery_evidence_damaged: bool,
        recovery_obligations: u64,
        inspection_required: bool,
    ) -> Self {
        Self {
            residue,
            recovery_evidence_damaged,
            recovery_obligations,
            inspection_required,
        }
    }

    pub const fn residue(self) -> bool {
        self.residue
    }

    pub const fn recovery_evidence_damaged(self) -> bool {
        self.recovery_evidence_damaged
    }

    pub const fn recovery_obligations(self) -> u64 {
        self.recovery_obligations
    }

    pub const fn inspection_required(self) -> bool {
        self.inspection_required
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkFreshReopenEvidence {
    identity: PhysicalWorkFreshReopenIdentity,
    posture: PhysicalWorkFreshReopenPosture,
}

impl PhysicalWorkFreshReopenEvidence {
    pub fn new(
        identity: PhysicalWorkFreshReopenIdentity,
        posture: PhysicalWorkFreshReopenPosture,
    ) -> Result<Self, PhysicalWorkHostileTruthEvidenceDenial> {
        let derived = posture.residue()
            || posture.recovery_evidence_damaged()
            || posture.recovery_obligations() != 0;
        if derived != posture.inspection_required()
            || (posture.inspection_required() && identity.records() != 0)
        {
            return Err(PhysicalWorkHostileTruthEvidenceDenial::InconsistentReopenPosture);
        }
        Ok(Self { identity, posture })
    }

    pub const fn identity(self) -> PhysicalWorkFreshReopenIdentity {
        self.identity
    }

    pub const fn posture(self) -> PhysicalWorkFreshReopenPosture {
        self.posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkHostileTruthComparison {
    baseline: PhysicalWorkHostileCurrentTruth,
    expected: PhysicalWorkHostileCurrentTruth,
    observed: PhysicalWorkHostileCurrentTruth,
}

impl PhysicalWorkHostileTruthComparison {
    pub const fn new(
        baseline: PhysicalWorkHostileCurrentTruth,
        expected: PhysicalWorkHostileCurrentTruth,
        observed: PhysicalWorkHostileCurrentTruth,
    ) -> Self {
        Self {
            baseline,
            expected,
            observed,
        }
    }

    pub const fn baseline(self) -> PhysicalWorkHostileCurrentTruth {
        self.baseline
    }

    pub const fn expected(self) -> PhysicalWorkHostileCurrentTruth {
        self.expected
    }

    pub const fn observed(self) -> PhysicalWorkHostileCurrentTruth {
        self.observed
    }
}
