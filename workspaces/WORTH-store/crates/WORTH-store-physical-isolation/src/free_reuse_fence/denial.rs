#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeReuseFenceDenial {
    ReclaimStillBlocked,
    ReclaimRemovalDoesNotCoverReusedIdentity,
    IdentityReuseOwnerMismatch,
    IdentityReuseWithoutGenerationAdvance,
    GenerationAdvancementOrderingNotCrashStable,
    AllocatorPublicationOrderingNotCrashStable,
}
