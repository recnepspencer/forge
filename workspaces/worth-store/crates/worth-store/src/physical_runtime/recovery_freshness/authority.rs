use worth_proof::AuthorityWitness;
use worth_store_physical_backend::PhysicalRecoveryMediaGeneration;

worth_proof::authority_marker!(pub PhysicalRecoveryFreshnessMarker);

/// Store-owned proof that recovery freshness was sampled for this new session.
///
/// The marker and every binding field are private. A caller can carry this
/// concrete authority, but cannot mint, clone, or rebind it.
#[derive(Debug)]
pub struct PhysicalRecoveryFreshnessAuthority {
    _witness: AuthorityWitness<PhysicalRecoveryFreshnessMarker>,
    _media_generation: PhysicalRecoveryMediaGeneration,
    _sample_identity: [u8; 16],
}

impl PhysicalRecoveryFreshnessAuthority {
    pub(super) fn issue(media_generation: PhysicalRecoveryMediaGeneration) -> Option<Self> {
        let mut sample_identity = [0; 16];
        getrandom::fill(&mut sample_identity).ok()?;
        (sample_identity != [0; 16]).then_some(Self {
            _witness: PhysicalRecoveryFreshnessMarker::witness(),
            _media_generation: media_generation,
            _sample_identity: sample_identity,
        })
    }

    pub(in crate::physical_runtime) fn matches_media_generation(
        &self,
        media_generation: PhysicalRecoveryMediaGeneration,
    ) -> bool {
        self._media_generation == media_generation
    }

    pub(super) const fn sample_identity(&self) -> [u8; 16] {
        self._sample_identity
    }
}
