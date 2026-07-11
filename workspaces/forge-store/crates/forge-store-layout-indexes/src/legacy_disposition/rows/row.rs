use super::super::{
    bypass::LegacyAccessPathBypass,
    disposition::LegacySurfaceDisposition,
    surface_row::{LegacySurfaceInventoryRow, LegacySurfaceOwner, LegacySurfaceStage},
};

pub(super) const fn superseded_legacy_root(
    surface: &'static str,
    stage: LegacySurfaceStage,
    bypass: LegacyAccessPathBypass,
) -> LegacySurfaceInventoryRow {
    legacy_root(
        surface,
        stage,
        LegacySurfaceDisposition::SupersededAndForbidden,
        bypass,
    )
}

pub(super) const fn forbidden_legacy_root(
    surface: &'static str,
    stage: LegacySurfaceStage,
    bypass: LegacyAccessPathBypass,
) -> LegacySurfaceInventoryRow {
    legacy_root(
        surface,
        stage,
        LegacySurfaceDisposition::ForbiddenAsAuthority,
        bypass,
    )
}

pub(super) const fn terminal_legacy_root(surface: &'static str) -> LegacySurfaceInventoryRow {
    legacy_root(
        surface,
        LegacySurfaceStage::ExecutionArtifact,
        LegacySurfaceDisposition::TerminalOnly,
        LegacyAccessPathBypass::Execution,
    )
}

pub(super) const fn owner_input(
    surface: &'static str,
    bypass: LegacyAccessPathBypass,
) -> LegacySurfaceInventoryRow {
    legacy_root(
        surface,
        LegacySurfaceStage::InputOnlyArtifact,
        LegacySurfaceDisposition::ConsumedAsInputOnly,
        bypass,
    )
}

pub(super) const fn certification_report(surface: &'static str) -> LegacySurfaceInventoryRow {
    LegacySurfaceInventoryRow::new(
        surface,
        LegacySurfaceOwner::CertificationLane,
        LegacySurfaceStage::CertificationArtifact,
        LegacySurfaceDisposition::CertificationOnly,
        LegacyAccessPathBypass::CertificationShortcut,
    )
}

const fn legacy_root(
    surface: &'static str,
    stage: LegacySurfaceStage,
    disposition: LegacySurfaceDisposition,
    bypass: LegacyAccessPathBypass,
) -> LegacySurfaceInventoryRow {
    LegacySurfaceInventoryRow::new(
        surface,
        LegacySurfaceOwner::LegacyRootCrate,
        stage,
        disposition,
        bypass,
    )
}
