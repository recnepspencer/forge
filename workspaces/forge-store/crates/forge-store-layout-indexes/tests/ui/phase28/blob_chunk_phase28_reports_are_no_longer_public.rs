use forge_store_blob_chunks::{
    CapsuleManifestLayoutReport, ExportBundleLayoutReport, ImportedLayoutReadmissionReport,
};

fn main() {
    let _ = (
        core::mem::size_of::<CapsuleManifestLayoutReport>(),
        core::mem::size_of::<ExportBundleLayoutReport>(),
        core::mem::size_of::<ImportedLayoutReadmissionReport>(),
    );
}
