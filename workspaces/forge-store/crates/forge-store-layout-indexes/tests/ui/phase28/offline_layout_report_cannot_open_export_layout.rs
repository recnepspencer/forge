use forge_store_offline_verifier::OfflineLayoutReport;
use forge_store_operations::ExportLayoutEvidenceReport;

fn main() {
    let report = OfflineLayoutReport::new(Vec::new());
    let _ = ExportLayoutEvidenceReport::from_blob_export_bundle(&report);
}
