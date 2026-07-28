use std::collections::BTreeSet;

use worth_ui_host_contract::UiMountedFrameManifest;

use super::assembly::UiMountedFramePreparationDenial;

pub(crate) fn validate_manifest(
    manifest: &UiMountedFrameManifest,
) -> Result<(), UiMountedFramePreparationDenial> {
    let mut surfaces = BTreeSet::new();
    let mut bindings = BTreeSet::new();
    for requirement in manifest.surfaces() {
        if !surfaces.insert(requirement.semantic_surface())
            || !bindings.insert(requirement.binding())
        {
            return Err(UiMountedFramePreparationDenial::IncompleteManifest);
        }
    }
    let expected = surfaces
        .iter()
        .flat_map(|surface| {
            [
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Ordinary,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Virtualized,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::CanvasSpatial,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Realtime,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Preview,
                ),
            ]
        })
        .collect::<BTreeSet<_>>();
    let actual = manifest
        .lane_contributions()
        .iter()
        .map(|cell| (cell.surface(), cell.lane()))
        .collect::<BTreeSet<_>>();
    if actual != expected || actual.len() != manifest.lane_contributions().len() {
        return Err(UiMountedFramePreparationDenial::IncompleteManifest);
    }
    Ok(())
}
