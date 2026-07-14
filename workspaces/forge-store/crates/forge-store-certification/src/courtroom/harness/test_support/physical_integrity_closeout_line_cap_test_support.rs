use crate::{
    courtroom::source_tree::{certification_source, store_crate_source},
    IntegrityCloseoutModuleKind, IntegrityCompositionEvidence, IntegrityModuleCompositionEvidence,
    IntegrityOwnedCloseoutFileEvidence,
};
use std::{fs, path::PathBuf};

pub(crate) fn line_cap_composition_evidence() -> IntegrityCompositionEvidence {
    IntegrityCompositionEvidence::from_checked_modules_and_owned_files(
        line_cap_module_evidence(),
        physical_integrity_owned_closeout_file_evidence(),
    )
    .unwrap()
}

pub(crate) fn line_cap_module_evidence() -> Vec<IntegrityModuleCompositionEvidence> {
    let cap = 400;
    vec![
        checked_module(
            IntegrityCloseoutModuleKind::Checksum,
            physical_integrity("checksums/checksum_algorithm.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::Scrub,
            physical_integrity("scrub/scrub_execution.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::Quarantine,
            physical_integrity("quarantine/quarantine_authority.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::Evidence,
            certification_courtroom("physical_integrity_closeout_proof.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::Handoff,
            recovery_physics("integrity_handoff/mod.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::CloseoutSuite,
            certification_courtroom("physical_integrity_closeout_suite.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::CloseoutReport,
            certification_courtroom("physical_integrity_closeout_report.rs"),
            cap,
        ),
        checked_module(
            IntegrityCloseoutModuleKind::CloseoutTest,
            certification_courtroom("physical_integrity_closeout_tests.rs"),
            cap,
        ),
    ]
}

pub(crate) fn physical_integrity_owned_closeout_file_evidence(
) -> Vec<IntegrityOwnedCloseoutFileEvidence> {
    let cap = 400;
    [
        certification_courtroom_dir().join("suite"),
        certification_courtroom_dir().join("composition"),
        certification_scenario_dir(),
    ]
    .into_iter()
    .flat_map(|directory| fs::read_dir(directory).unwrap())
    .map(|entry| entry.unwrap().path())
    .filter(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| {
                name.starts_with("physical_integrity_closeout") && name.ends_with(".rs")
            })
    })
    .map(|path| {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .to_string();
        IntegrityOwnedCloseoutFileEvidence::checked(name, line_count(path), cap).unwrap()
    })
    .collect()
}

fn checked_module(
    module: IntegrityCloseoutModuleKind,
    path: PathBuf,
    cap: u32,
) -> IntegrityModuleCompositionEvidence {
    IntegrityModuleCompositionEvidence::checked(module, line_count(path), cap).unwrap()
}

fn line_count(path: PathBuf) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .count()
        .try_into()
        .unwrap()
}

fn certification_courtroom(file: &str) -> PathBuf {
    certification_courtroom_dir().join("suite").join(file)
}

fn certification_courtroom_dir() -> PathBuf {
    certification_source("courtroom/physical_integrity/closeout")
}

fn certification_scenario_dir() -> PathBuf {
    certification_source("scenario/physical_integrity")
}

fn physical_integrity(file: &str) -> PathBuf {
    store_crate_source("forge-store-physical-integrity").join(file)
}

fn recovery_physics(file: &str) -> PathBuf {
    store_crate_source("forge-store-recovery-physics").join(file)
}
