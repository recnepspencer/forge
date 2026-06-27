use crate::ChecksumAlgorithmId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedPhysicalChecksum {
    algorithm: ChecksumAlgorithmId,
    value: u64,
}

impl ExecutedPhysicalChecksum {
    pub(crate) const fn new(algorithm: ChecksumAlgorithmId, value: u64) -> Self {
        Self { algorithm, value }
    }

    pub const fn algorithm(self) -> ChecksumAlgorithmId {
        self.algorithm
    }

    pub const fn value(self) -> u64 {
        self.value
    }
}

pub(crate) fn execute_declared_checksum(
    algorithm: ChecksumAlgorithmId,
    bytes: &[u8],
) -> ExecutedPhysicalChecksum {
    let value = match algorithm.as_str() {
        "crc32c" => u64::from(crc32c(bytes)),
        "crc64-nvme" => crc64_nvme_compatible(bytes),
        _ => unreachable!("ChecksumAlgorithmId admits only declared algorithms"),
    };
    ExecutedPhysicalChecksum::new(algorithm, value)
}

fn crc32c(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    !crc
}

fn crc64_nvme_compatible(bytes: &[u8]) -> u64 {
    let mut crc = !0u64;
    for byte in bytes {
        crc ^= u64::from(*byte);
        for _ in 0..8 {
            let mask = 0u64.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x9a6c_9329_ac4b_c9b5 & mask);
        }
    }
    !crc
}
