use worth_spatial::facade::replay_family_catalog::admit_spatial_replay_family_identity;
use worth_spatial::facade::undo_family_catalog::SpatialUndoFamilyIdentityAuthority;

fn main() {
    let authority = SpatialUndoFamilyIdentityAuthority::boolean_event_ledger();
    let _ = admit_spatial_replay_family_identity(authority);
}
