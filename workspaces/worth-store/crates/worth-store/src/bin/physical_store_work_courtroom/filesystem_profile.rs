use worth_store::physical_runtime::{
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkFilesystemSupportEvidence,
};

pub(super) fn emit(profile: &PhysicalWorkFilesystemProfileEvidence) {
    let support = profile
        .capabilities()
        .iter()
        .map(|observation| match observation.support() {
            PhysicalWorkFilesystemSupportEvidence::Supported => 'S',
            PhysicalWorkFilesystemSupportEvidence::Unsupported => 'U',
            PhysicalWorkFilesystemSupportEvidence::Indeterminate => 'I',
        })
        .collect::<String>();
    println!(
        "C5_1_FILESYSTEM_PROFILE {} {} {} {} {} {} {} {}",
        hex(&profile.root_identity()),
        hex(&profile.volume_identity()),
        hex(profile.filesystem_type().as_bytes()),
        profile.allocation_granularity(),
        profile.location().label(),
        profile.is_removable(),
        profile.is_read_only(),
        support,
    );
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
