use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredInputRole, ReplayUndoDeclaredSourceCatalog, ReplayUndoDeclaredSourceIdentity,
};

pub(crate) fn required_role_for_source(
    catalog: &ReplayUndoDeclaredSourceCatalog,
    identity: ReplayUndoDeclaredSourceIdentity,
    role: ReplayUndoDeclaredInputRole,
) -> bool {
    catalog
        .require_source(identity)
        .map(|source| {
            source.authority_roles().contains(role) || source.observability_roles().contains(role)
        })
        .unwrap_or(false)
}
