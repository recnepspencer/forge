use sha2::{Digest, Sha256};

/// Exact cross-crate description of one proposed recovery cleanup effect.
///
/// This value is deliberately not authority. The physical effect owner must
/// match every field against the freshly read selector, verified checkpoint,
/// verified WAL artifact, and admitted Store work before it may mint an
/// effect admission identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryCleanupEffectBinding {
    store: [u8; 16],
    session: [u8; 16],
    plan: [u8; 32],
    published_generation: u64,
    checkpoint_store: [u8; 16],
    checkpoint_sequence: u64,
    artifact_segment: u64,
    artifact_generation: u64,
    lsn_start: u64,
    lsn_end_exclusive: u64,
    byte_count: u64,
    artifact_digest: [u8; 32],
    work_runtime: u64,
    work_generation: u64,
    work_operation: u64,
}

impl RecoveryCleanupEffectBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        store: [u8; 16],
        session: [u8; 16],
        plan: [u8; 32],
        published_generation: u64,
        checkpoint_store: [u8; 16],
        checkpoint_sequence: u64,
        artifact_segment: u64,
        artifact_generation: u64,
        lsn_start: u64,
        lsn_end_exclusive: u64,
        byte_count: u64,
        artifact_digest: [u8; 32],
        work_runtime: u64,
        work_generation: u64,
        work_operation: u64,
    ) -> Option<Self> {
        (store != [0; 16]
            && session != [0; 16]
            && plan != [0; 32]
            && published_generation != 0
            && checkpoint_store == store
            && checkpoint_sequence != 0
            && artifact_segment != 0
            && artifact_generation != 0
            && lsn_start < lsn_end_exclusive
            && byte_count != 0
            && artifact_digest != [0; 32]
            && work_runtime != 0
            && work_generation != 0
            && work_operation != 0)
            .then_some(Self {
                store,
                session,
                plan,
                published_generation,
                checkpoint_store,
                checkpoint_sequence,
                artifact_segment,
                artifact_generation,
                lsn_start,
                lsn_end_exclusive,
                byte_count,
                artifact_digest,
                work_runtime,
                work_generation,
                work_operation,
            })
    }

    pub const fn store(self) -> [u8; 16] {
        self.store
    }
    pub const fn session(self) -> [u8; 16] {
        self.session
    }
    pub const fn plan(self) -> [u8; 32] {
        self.plan
    }
    pub const fn published_generation(self) -> u64 {
        self.published_generation
    }
    pub const fn checkpoint_store(self) -> [u8; 16] {
        self.checkpoint_store
    }
    pub const fn checkpoint_sequence(self) -> u64 {
        self.checkpoint_sequence
    }
    pub const fn artifact_segment(self) -> u64 {
        self.artifact_segment
    }
    pub const fn artifact_generation(self) -> u64 {
        self.artifact_generation
    }
    pub const fn lsn_start(self) -> u64 {
        self.lsn_start
    }
    pub const fn lsn_end_exclusive(self) -> u64 {
        self.lsn_end_exclusive
    }
    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }
    pub const fn artifact_digest(self) -> [u8; 32] {
        self.artifact_digest
    }
    pub const fn work_runtime(self) -> u64 {
        self.work_runtime
    }
    pub const fn work_generation(self) -> u64 {
        self.work_generation
    }
    pub const fn work_operation(self) -> u64 {
        self.work_operation
    }

    pub fn identity(&self) -> [u8; 32] {
        let binding = *self;
        let mut digest = Sha256::new();
        digest.update(b"worth.store.recovery.cleanup-effect-admission.v2");
        digest.update(binding.store);
        digest.update(binding.session);
        digest.update(binding.plan);
        digest.update(binding.published_generation.to_le_bytes());
        digest.update(binding.checkpoint_store);
        digest.update(binding.checkpoint_sequence.to_le_bytes());
        digest.update(binding.artifact_segment.to_le_bytes());
        digest.update(binding.artifact_generation.to_le_bytes());
        digest.update(binding.lsn_start.to_le_bytes());
        digest.update(binding.lsn_end_exclusive.to_le_bytes());
        digest.update(binding.byte_count.to_le_bytes());
        digest.update(binding.artifact_digest);
        digest.update(binding.work_runtime.to_le_bytes());
        digest.update(binding.work_generation.to_le_bytes());
        digest.update(binding.work_operation.to_le_bytes());
        digest.finalize().into()
    }
}
