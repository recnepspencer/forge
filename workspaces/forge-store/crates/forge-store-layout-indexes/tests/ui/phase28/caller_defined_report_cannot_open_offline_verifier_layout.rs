use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{OfflineVerifierAccessShape, OfflineVerifierAuthorityPosture, OfflineVerifierEvidenceKind, OfflineVerifierLayoutReport};
fn main() { let _ = OfflineVerifierLayoutReport { family_id: DurableArtifactFamilyId::OfflineVerificationRecord, access_shape: OfflineVerifierAccessShape::FullDeclaredScan, posture: OfflineVerifierAuthorityPosture::TerminalOnly, evidence_kind: OfflineVerifierEvidenceKind::LayoutReport, evidence_items: 0, admission: todo!() }; }
