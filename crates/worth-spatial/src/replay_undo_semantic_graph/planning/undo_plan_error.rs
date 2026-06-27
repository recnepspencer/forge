use crate::undo_family_catalog::{SpatialUndoFamilyIdentity, SpatialUndoFamilyScopeProductPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialUndoPlanError {
    UnsupportedScopeProductPosture {
        family_identity: SpatialUndoFamilyIdentity,
        scope_product_posture: SpatialUndoFamilyScopeProductPosture,
    },
}
