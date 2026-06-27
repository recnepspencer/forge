use worth_spatial::facade::replay_family_catalog::SpatialReplayFamilyIdentityAuthority;
use worth_spatial::facade::undo_family_catalog::admit_spatial_undo_family_identity;

fn main() {
    let authority = SpatialReplayFamilyIdentityAuthority::boolean_event_ledger();
    let _ = admit_spatial_undo_family_identity(authority);
}
