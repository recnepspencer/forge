use crate::undo_family_catalog::{
    TopologyUndoFamilyIdentity, TopologyUndoFamilyScopeProductPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyUndoPlanError {
    UnsupportedScopeProductPosture {
        family_identity: TopologyUndoFamilyIdentity,
        scope_product_posture: TopologyUndoFamilyScopeProductPosture,
    },
}
