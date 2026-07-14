pub(crate) fn stable_fixture_digest(material: &[u8]) -> [u8; 32] {
    let mut lanes = [
        0xcbf2_9ce4_8422_2325_u64,
        0x9e37_79b9_7f4a_7c15_u64,
        0x94d0_49bb_1331_11eb_u64,
        0xd6e8_feb8_6659_fd93_u64,
    ];
    for (index, byte) in material.iter().enumerate() {
        let lane = index % lanes.len();
        lanes[lane] ^= u64::from(*byte) + ((index as u64) << 8);
        lanes[lane] = lanes[lane].wrapping_mul(0x1000_0000_01b3);
        lanes[lane] = lanes[lane].rotate_left(13);
    }

    let mut bytes = [0_u8; 32];
    for (index, lane) in lanes.iter().enumerate() {
        bytes[index * 8..(index + 1) * 8].copy_from_slice(&lane.to_be_bytes());
    }
    bytes
}
