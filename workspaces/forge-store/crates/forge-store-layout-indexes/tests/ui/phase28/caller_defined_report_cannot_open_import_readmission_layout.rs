use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{Phase28LayoutAuthorityPosture, S8AccessShape};
use forge_store_operations::ImportLayoutEvidenceReport;
fn main() { let _ = ImportLayoutEvidenceReport { family_id: DurableArtifactFamilyId::ImportBundle, access_shape: S8AccessShape::PointLookup, posture: Phase28LayoutAuthorityPosture::ReadmittedEvidence, declared_chunks: 1, local_chunks: 1, admission: todo!() }; }
