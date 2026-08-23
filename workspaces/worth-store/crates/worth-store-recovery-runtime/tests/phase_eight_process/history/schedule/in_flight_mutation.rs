use sha2::{Digest, Sha256};

const CHECKPOINT_PAGE_BYTES: usize = 16 * 1024;
const EXTENT_OVERHEAD_BYTES: usize = 112;
const ORDINARY_PAYLOAD_BYTES: usize = 8 * 1024;
const CHECKPOINT_PAYLOAD_BYTES: usize = 2 * (CHECKPOINT_PAGE_BYTES - EXTENT_OVERHEAD_BYTES) + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectedInFlightMutation {
    perturbation_seed: u64,
    material: [u8; 32],
    payload: Vec<u8>,
    fate: u8,
}

impl ExpectedInFlightMutation {
    pub(crate) fn checkpoint_exact_prefix(seed: u64) -> Self {
        Self::new(seed, CHECKPOINT_PAYLOAD_BYTES, 4)
    }

    pub(crate) fn durable_before_ack(seed: u64) -> Self {
        Self::new(seed, ORDINARY_PAYLOAD_BYTES, 2)
    }

    pub(crate) const fn material(&self) -> [u8; 32] {
        self.material
    }

    pub(crate) const fn perturbation_seed(&self) -> u64 {
        self.perturbation_seed
    }

    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub(crate) const fn fate(&self) -> u8 {
        self.fate
    }

    fn new(seed: u64, payload_bytes: usize, fate: u8) -> Self {
        let material = dirty_material(seed);
        Self {
            perturbation_seed: seed,
            material,
            payload: payload(material, payload_bytes),
            fate,
        }
    }
}

fn dirty_material(seed: u64) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.c8.dirty-mutation.v1");
    digest.update(seed.to_le_bytes());
    digest.finalize().into()
}

fn payload(material: [u8; 32], length: usize) -> Vec<u8> {
    let mut payload = vec![0; length];
    for (index, byte) in payload.iter_mut().enumerate() {
        *byte = material[index % material.len()]
            .wrapping_add(index as u8)
            .wrapping_mul(31);
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::ExpectedInFlightMutation;

    #[test]
    fn semantic_profiles_bind_payload_geometry_and_fate() {
        let checkpoint = ExpectedInFlightMutation::checkpoint_exact_prefix(7);
        assert_eq!(checkpoint.payload().len(), 32_545);
        assert_eq!(checkpoint.fate(), 4);

        let durable = ExpectedInFlightMutation::durable_before_ack(7);
        assert_eq!(durable.payload().len(), 8 * 1024);
        assert_eq!(durable.fate(), 2);
        assert_eq!(checkpoint.material(), durable.material());
    }
}
