use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{Phase28LayoutAuthorityPosture, S8AccessShape};
use forge_store_operations::RestoreLayoutEvidenceReport;
fn main() { let _ = RestoreLayoutEvidenceReport { family_id: DurableArtifactFamilyId::ImportBundle, access_shape: S8AccessShape::PointLookup, posture: Phase28LayoutAuthorityPosture::ReadmissionRequired, trigger: todo!(), admission: todo!() }; }
