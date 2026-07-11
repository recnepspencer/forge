use forge_store_operations::CapsuleOperationLayoutReport;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{Phase28LayoutAuthorityPosture, S8AccessShape};
fn main() { let _ = CapsuleOperationLayoutReport { family_id: DurableArtifactFamilyId::CapsuleArtifact, access_shape: S8AccessShape::ManifestGraphWalk, posture: Phase28LayoutAuthorityPosture::TerminalOnly, declared_bytes: 4, admission: todo!() }; }
