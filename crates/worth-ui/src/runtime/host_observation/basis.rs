use super::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHostObservationBasis {
    mounted_product_view_digest: u64,
    frame_epoch: u64,
    basis_digest: u64,
}

impl WorthUiHostObservationBasis {
    pub fn new(mounted_product_view_digest: u64, frame_epoch: u64) -> Self {
        let basis_digest = digest_parts([
            "host_observation_basis",
            &mounted_product_view_digest.to_string(),
            &frame_epoch.to_string(),
        ]);
        Self {
            mounted_product_view_digest,
            frame_epoch,
            basis_digest,
        }
    }

    pub fn mounted_product_view_digest(&self) -> u64 {
        self.mounted_product_view_digest
    }

    pub fn frame_epoch(&self) -> u64 {
        self.frame_epoch
    }

    pub fn basis_digest(&self) -> u64 {
        self.basis_digest
    }
}
