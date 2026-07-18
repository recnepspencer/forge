use worth_store_certification::S51CertificationCloseoutEvidence;
use worth_store_operations::BackupExportCustodyReadiness;

fn requires_readiness(_: BackupExportCustodyReadiness) {}

fn main() {
    let evidence: S51CertificationCloseoutEvidence = todo!();
    requires_readiness(evidence);
}
