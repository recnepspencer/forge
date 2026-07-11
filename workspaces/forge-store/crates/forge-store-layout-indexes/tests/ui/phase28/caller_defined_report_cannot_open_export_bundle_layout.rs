use forge_store_operations::ExportLayoutEvidenceReport;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{Phase28LayoutAuthorityPosture, S8AccessShape};
fn main() { let _ = ExportLayoutEvidenceReport { family_id: DurableArtifactFamilyId::ExportBundle, access_shape: S8AccessShape::ManifestGraphWalk, posture: Phase28LayoutAuthorityPosture::TerminalOnly, declared_chunks: 1, admission: todo!() }; }
