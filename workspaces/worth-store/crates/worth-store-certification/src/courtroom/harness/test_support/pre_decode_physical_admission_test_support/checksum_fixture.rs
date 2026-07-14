use worth_store_physical_format::ChecksumCoverageMap;
use worth_store_physical_integrity::{
    ChecksumAlgorithmDeclaration, ChecksumAlgorithmId, ChecksumScopeDeclaration,
};

pub(crate) fn checksum_declaration() -> ChecksumAlgorithmDeclaration {
    ChecksumAlgorithmId::crc32c()
        .declare_for_scope(checksum_scope())
        .unwrap()
}

pub(crate) fn checksum_scope() -> ChecksumScopeDeclaration {
    let format =
        worth_store_physical_format::PhysicalFormatDeclaration::physical_format_canonical()
            .unwrap();
    ChecksumScopeDeclaration::for_physical_format(
        format.identity(),
        ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap(),
    )
    .unwrap()
}

pub(crate) fn crc32c(bytes: &[u8]) -> u64 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    u64::from(!crc)
}
