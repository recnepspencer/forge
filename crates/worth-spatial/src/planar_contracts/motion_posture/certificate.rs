use super::{
    planar_motion_posture_authority_entries, planar_motion_posture_digest,
    PlanarMotionPostureBasis, PlanarMotionPostureCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarMotionPostureReceipt {
    basis: PlanarMotionPostureBasis,
    declaration_digest: String,
    envelope_digest: String,
    retained_motion_digest: String,
    counters: PlanarMotionPostureCounters,
}

impl PlanarMotionPostureReceipt {
    pub(crate) fn new(
        basis: PlanarMotionPostureBasis,
        declaration_digest: String,
        envelope_digest: String,
        retained_motion_digest: String,
        counters: PlanarMotionPostureCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            retained_motion_digest,
            counters,
        }
    }

    pub(crate) fn retained_motion_digest_for(basis: &PlanarMotionPostureBasis) -> String {
        planar_motion_posture_digest(
            &planar_motion_posture_authority_entries(basis)
                .into_iter()
                .map(|entry| entry.digest_part())
                .collect::<Vec<_>>(),
        )
    }

    pub fn basis(&self) -> &PlanarMotionPostureBasis {
        &self.basis
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn retained_motion_digest(&self) -> &str {
        &self.retained_motion_digest
    }

    pub fn counters(&self) -> PlanarMotionPostureCounters {
        self.counters
    }
}
