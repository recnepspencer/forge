use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::RelationKind;
use crate::logic::transaction::SpecDraft;

use super::wire_loop_cycle::{create_loop_cycle, WiredLoopCycle};

pub fn create_face_cycle(
    draft: &mut SpecDraft,
    face: SpecNodeId,
    vertices: &[SpecNodeId],
    role_prefix: &str,
) -> Result<WiredLoopCycle, SpecError> {
    create_loop_cycle(
        draft,
        face,
        vertices,
        RelationKind::FaceOuterLoop,
        0,
        &format!("{role_prefix}-face-outer"),
    )
}
