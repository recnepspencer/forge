use std::process::Command;

use worth_store_certification::courtroom::operational_recovery::{
    required_s10_crash_reopen_yieldpoints, S10OperationalScenarioKind,
};
use worth_store_operations::certification_scenario::reopen_owner_backed_control_store_at;
use worth_store_physical_certification::{
    admit_current_process_probe, write_reopen_observation_from_environment,
    OperationalRecoveryCrashCutEvidence,
    OperationalRecoveryDriverTrace, OperationalRecoveryFreshProcessRunner,
    OperationalRecoveryProcessCrashConfig, OperationalRecoveryYieldpoint, ProcessRole,
    PROCESS_CRASH_ROLE_ENV,
};

const ROOT_ENV: &str = "WORTH_STORE_S10_SCENARIO_CRASH_ROOT";
const IDENTITY_ENV: &str = "WORTH_STORE_S10_SCENARIO_CRASH_IDENTITY";
const KIND_ENV: &str = "WORTH_STORE_S10_SCENARIO_CRASH_KIND";

pub fn scenario_crash_coverage(
    kind: S10OperationalScenarioKind,
    identity: &str,
    trace: &OperationalRecoveryDriverTrace,
) -> Vec<OperationalRecoveryCrashCutEvidence> {
    required_s10_crash_reopen_yieldpoints(kind)
        .into_iter()
        .map(|point| certify_cut(kind, identity, trace, point))
        .collect()
}

fn certify_cut(
    kind: S10OperationalScenarioKind,
    identity: &str,
    trace: &OperationalRecoveryDriverTrace,
    point: OperationalRecoveryYieldpoint,
) -> OperationalRecoveryCrashCutEvidence {
    let directory = tempfile::tempdir().unwrap();
    let media_root = directory.path().join("media");
    let executable = std::env::current_exe().unwrap();
    let exact = "s10_operational_world::crash_coverage::scenario_process_crash_probe";
    let command = || {
        let mut command = Command::new(&executable);
        command
            .arg("--exact")
            .arg(exact)
            .arg("--nocapture")
            .env(ROOT_ENV, &media_root)
            .env(IDENTITY_ENV, identity)
            .env(KIND_ENV, kind.token());
        command
    };
    OperationalRecoveryFreshProcessRunner::new(directory.path().join("evidence"))
        .certify_control_cut(
            &media_root,
            identity,
            &mut command(),
            &mut command(),
            point,
            trace,
        )
        .unwrap()
}

#[test]
fn scenario_process_crash_probe() {
    let Some(root) = std::env::var_os(ROOT_ENV).map(std::path::PathBuf::from) else {
        return;
    };
    let identity = std::env::var(IDENTITY_ENV).unwrap();
    let kind = scenario_kind_from_token(&std::env::var(KIND_ENV).unwrap());
    if std::env::var(PROCESS_CRASH_ROLE_ENV).ok().as_deref() == Some("reopen") {
        let admission = admit_current_process_probe(ProcessRole::RecoveredRuntime).unwrap();
        let control = reopen_owner_backed_control_store_at(&root);
        assert!(write_reopen_observation_from_environment(&admission, &control).unwrap());
        return;
    }
    let config = OperationalRecoveryProcessCrashConfig::from_environment()
        .unwrap()
        .expect("cut child configuration");
    super::owner_world::execute_scenario_crash_probe(kind, &identity, &root, config);
    panic!("configured crash cut must terminate the child");
}

fn scenario_kind_from_token(token: &str) -> S10OperationalScenarioKind {
    [
        S10OperationalScenarioKind::BurningPrimary,
        S10OperationalScenarioKind::SplitBrainPromotion,
        S10OperationalScenarioKind::AuthorityRepairRollback,
    ]
    .into_iter()
    .find(|kind| kind.token() == token)
    .expect("known S10 scenario kind")
}
