use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    PhysicalPageSizeClass, PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration,
    PhysicalRecordOpen, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordBootstrapDenial,
};

pub(super) struct InvalidScaleWorlds {
    pub(super) missing_catalog_refused: bool,
    pub(super) checksum_damage_refused: bool,
    pub(super) stale_manifest_refused: bool,
    pub(super) format_drift_refused: bool,
    pub(super) residue_excluded: bool,
}

impl InvalidScaleWorlds {
    pub(super) fn count(&self) -> u8 {
        [
            self.missing_catalog_refused,
            self.checksum_damage_refused,
            self.stale_manifest_refused,
            self.format_drift_refused,
            self.residue_excluded,
        ]
        .into_iter()
        .filter(|passed| *passed)
        .count() as u8
    }
}

pub(super) fn exercise(
    source: &Path,
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
    access: AdmittedRecordAccessPolicy,
) -> InvalidScaleWorlds {
    InvalidScaleWorlds {
        missing_catalog_refused: missing_catalog(source, format, access),
        checksum_damage_refused: checksum_damage(source, format, access),
        stale_manifest_refused: stale_manifest(source, format, access),
        format_drift_refused: format_drift(source),
        residue_excluded: unpublished_residue(source, format, placement, access),
    }
}

fn missing_catalog(
    source: &Path,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
) -> bool {
    let world = clone_world(source, "missing-catalog");
    std::fs::remove_file(world.join("families/records/bootstrap.catalog")).unwrap();
    let TransitionOutcome::Denied(denial) = super::media(&world)
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw()
    else {
        return false;
    };
    let matched = denial.reason() == RecordBootstrapDenial::AmbiguousRecordFamilyResidue
        && worth_store_offline_verifier::walk_current_durable_record_manifest(
            &world,
            format.declaration(),
        )
        .is_err();
    denial.into_runtime().close();
    matched
}

fn checksum_damage(
    source: &Path,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
) -> bool {
    let world = clone_world(source, "checksum-damage");
    let root = current_root_path(&world, format);
    let mut bytes = std::fs::read(&root).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(root, bytes).unwrap();
    let TransitionOutcome::Denied(denial) = super::media(&world)
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw()
    else {
        return false;
    };
    let matched = denial.reason() == RecordBootstrapDenial::CurrentRootDamaged
        && worth_store_offline_verifier::walk_current_durable_record_manifest(
            &world,
            format.declaration(),
        )
        .is_err();
    denial.into_runtime().close();
    matched
}

fn stale_manifest(
    source: &Path,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
) -> bool {
    let world = clone_world(source, "stale-manifest");
    let current = current_root_path(&world, format);
    let stale = world.join("families/records/roots/root-0000000000000001.manifest");
    std::fs::copy(stale, current).unwrap();
    match super::media(&world)
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw()
    {
        TransitionOutcome::Stale(stale) => {
            stale.into_runtime().close();
            worth_store_offline_verifier::walk_current_durable_record_manifest(
                &world,
                format.declaration(),
            )
            .is_err()
        }
        TransitionOutcome::Success(runtime) => {
            runtime.close();
            false
        }
        TransitionOutcome::Denied(denial) => {
            denial.into_runtime().close();
            false
        }
        _ => false,
    }
}

fn format_drift(source: &Path) -> bool {
    let world = clone_world(source, "format-drift");
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder()
            .page_size(PhysicalPageSizeClass::KiB64)
            .admit()
            .unwrap(),
    );
    let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
    let TransitionOutcome::Denied(denial) = super::media(&world)
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw()
    else {
        return false;
    };
    let matched = matches!(
        denial.reason(),
        RecordBootstrapDenial::PhysicalRecordFormatMismatch(_)
    ) && worth_store_offline_verifier::walk_current_durable_record_manifest(
        &world,
        format.declaration(),
    )
    .is_err();
    denial.into_runtime().close();
    matched
}

fn unpublished_residue(
    source: &Path,
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
    access: AdmittedRecordAccessPolicy,
) -> bool {
    let world = clone_world(source, "unpublished-residue");
    let catalog = world.join("families/records/bootstrap.catalog");
    let candidate = world.join("staging/records/bootstrap-deadbeefdeadbeef.candidate");
    let candidate_bytes = std::fs::copy(&catalog, &candidate).unwrap();
    assert_eq!(candidate_bytes, std::fs::metadata(catalog).unwrap().len());
    let reopened = super::success(
        super::media(&world).open_record_store(PhysicalRecordOpen::new(format, access)),
    );
    let excluded = reopened.observed_staging_residue()
        && reopened.publication_residue().staging_catalog_candidate()
        && reopened.observed_non_authoritative_residue()
        && matches!(
            reopened.record_submission().append_batch(
                RecordAppendBatch::try_from_iter([b"blocked".as_slice()]).unwrap(),
                placement,
            ),
            Err(RecordAppendError::Denied(
                RecordAppendDenial::ServingRequiresInspection
            ))
        );
    reopened.close();
    let offline_prior = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &world,
        format.declaration(),
    )
    .is_ok_and(|walk| !walk.placements().is_empty());
    excluded && offline_prior
}

fn current_root_path(root: &Path, format: AdmittedPhysicalRecordFormat) -> std::path::PathBuf {
    let generation = worth_store_offline_verifier::walk_current_durable_record_manifest(
        root,
        format.declaration(),
    )
    .unwrap()
    .root_generation();
    root.join(format!(
        "families/records/roots/root-{generation:016x}.manifest"
    ))
}

fn clone_world(source: &Path, name: &str) -> std::path::PathBuf {
    let source_name = source.file_name().unwrap().to_string_lossy();
    let destination = source
        .parent()
        .unwrap()
        .join(format!("invalid-{source_name}-{name}"));
    assert!(
        !destination.exists(),
        "an invalid-world clone must begin from one exact valid source"
    );
    copy_directory(source, &destination);
    destination
}

fn copy_directory(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_directory(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}
