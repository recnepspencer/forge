use crate::{
    S3CloseoutModuleKind, S3LineCapCompositionEvidence, S3LineCapModuleEvidence,
    S3OwnedCloseoutFileEvidence,
};
use std::{fs, path::PathBuf};

pub(crate) fn line_cap_composition_evidence() -> S3LineCapCompositionEvidence {
    S3LineCapCompositionEvidence::from_checked_modules_and_owned_files(
        line_cap_module_evidence(),
        s3_owned_closeout_file_evidence(),
    )
    .unwrap()
}

pub(crate) fn line_cap_module_evidence() -> Vec<S3LineCapModuleEvidence> {
    let cap = 400;
    vec![
        checked_module(
            S3CloseoutModuleKind::Checksum,
            physical_integrity("checksum_algorithm.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::Scrub,
            physical_integrity("scrub_execution.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::Quarantine,
            physical_integrity("quarantine_authority.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::Evidence,
            certification("physical_integrity_closeout_proof.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::Handoff,
            recovery_physics("s4_integrity_handoff_payload.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::CloseoutSuite,
            certification("physical_integrity_closeout_suite.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::CloseoutReport,
            certification("physical_integrity_closeout_report.rs"),
            cap,
        ),
        checked_module(
            S3CloseoutModuleKind::CloseoutTest,
            certification("physical_integrity_closeout_tests.rs"),
            cap,
        ),
    ]
}

pub(crate) fn s3_owned_closeout_file_evidence() -> Vec<S3OwnedCloseoutFileEvidence> {
    let cap = 400;
    fs::read_dir(certification_src())
        .unwrap()
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
            S3OwnedCloseoutFileEvidence::checked(name, line_count(path), cap).unwrap()
        })
        .collect()
}

fn checked_module(
    module: S3CloseoutModuleKind,
    path: PathBuf,
    cap: u32,
) -> S3LineCapModuleEvidence {
    S3LineCapModuleEvidence::checked(module, line_count(path), cap).unwrap()
}

fn line_count(path: PathBuf) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .count()
        .try_into()
        .unwrap()
}

fn certification(file: &str) -> PathBuf {
    certification_src().join(file)
}

fn certification_src() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn physical_integrity(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("forge-store-physical-integrity")
        .join("src")
        .join(file)
}

fn recovery_physics(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("forge-store-recovery-physics")
        .join("src")
        .join(file)
}
