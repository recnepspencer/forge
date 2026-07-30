use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceDigest, PhysicalWorkFeatureGraphEvidence, PhysicalWorkSourceBinding,
};

use super::process_execution;

mod source_inventory;

const BUILD_TIMEOUT: Duration = Duration::from_secs(300);

pub(super) struct BoundBinary {
    path: PathBuf,
    binding: PhysicalWorkSourceBinding,
}

#[derive(Clone, Copy)]
pub(super) struct ExecutableBindingTimings {
    source_inventory: Duration,
    prebuild_source_binding: Duration,
    postbuild_binary_binding: Duration,
    postbuild_source_binding: Duration,
}

impl BoundBinary {
    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) const fn binding(&self) -> &PhysicalWorkSourceBinding {
        &self.binding
    }

    fn verify_unchanged(&self) -> Result<(), String> {
        if hash_file(&self.path)? != self.binding.digest() {
            return Err(format!(
                "courtroom executable changed during the campaign: {}",
                self.path.display()
            ));
        }
        Ok(())
    }
}

pub(super) struct BuiltCourtroomExecutables {
    source_inventory: source_inventory::LocalSourceInventory,
    source: PhysicalWorkSourceBinding,
    runner: BoundBinary,
    writer: BoundBinary,
    observer: BoundBinary,
    cargo_build_elapsed: Duration,
    binding_timings: ExecutableBindingTimings,
}

impl BuiltCourtroomExecutables {
    pub(super) fn build(workspace: &Path) -> Result<Self, String> {
        let inventory_started = Instant::now();
        let source_inventory = source_inventory::LocalSourceInventory::discover(workspace)?;
        let source_inventory_elapsed = inventory_started.elapsed();
        let binding_started = Instant::now();
        let source_before_build = source_inventory.bind()?;
        let prebuild_source_binding_elapsed = binding_started.elapsed();
        let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
        command.current_dir(workspace).args([
            "build",
            "--quiet",
            "--release",
            "--package",
            "worth-store",
            "--features",
            "worth-store/certification-test-authority",
            "--bin",
            "physical_store_work_courtroom",
            "--package",
            "worth-store-offline-verifier",
            "--bin",
            "physical_store_offline_observer",
        ]);
        let build_started = Instant::now();
        process_execution::run_success(&mut command, BUILD_TIMEOUT, "courtroom binary build")?;
        let cargo_build_elapsed = build_started.elapsed();
        let binding_started = Instant::now();
        let target = target_directory(workspace).join("release");
        let runner = bind_current_binary()?;
        let writer = bind_binary(&target, "physical_store_work_courtroom")?;
        let observer = bind_binary(&target, "physical_store_offline_observer")?;
        let postbuild_binary_binding_elapsed = binding_started.elapsed();
        let binding_started = Instant::now();
        let source_after_build = source_inventory.bind()?;
        let source = require_stable_source(source_before_build, source_after_build)?;
        let postbuild_source_binding_elapsed = binding_started.elapsed();
        Ok(Self {
            source_inventory,
            source,
            runner,
            writer,
            observer,
            cargo_build_elapsed,
            binding_timings: ExecutableBindingTimings {
                source_inventory: source_inventory_elapsed,
                prebuild_source_binding: prebuild_source_binding_elapsed,
                postbuild_binary_binding: postbuild_binary_binding_elapsed,
                postbuild_source_binding: postbuild_source_binding_elapsed,
            },
        })
    }

    pub(super) const fn source(&self) -> &PhysicalWorkSourceBinding {
        &self.source
    }

    pub(super) const fn feature_graph(&self) -> &PhysicalWorkFeatureGraphEvidence {
        self.source_inventory.feature_graph()
    }

    pub(super) const fn writer(&self) -> &BoundBinary {
        &self.writer
    }

    pub(super) const fn runner(&self) -> &BoundBinary {
        &self.runner
    }

    pub(super) const fn observer(&self) -> &BoundBinary {
        &self.observer
    }

    pub(super) fn verify_source_unchanged(&self) -> Result<(), String> {
        require_campaign_source(&self.source, self.source_inventory.bind()?)
    }

    pub(super) fn verify_executables_unchanged(&self) -> Result<(), String> {
        self.runner.verify_unchanged()?;
        self.writer.verify_unchanged()?;
        self.observer.verify_unchanged()
    }

    pub(super) const fn cargo_build_elapsed(&self) -> Duration {
        self.cargo_build_elapsed
    }

    pub(super) const fn binding_timings(&self) -> ExecutableBindingTimings {
        self.binding_timings
    }
}

impl ExecutableBindingTimings {
    pub(super) const fn source_inventory(self) -> Duration {
        self.source_inventory
    }

    pub(super) const fn prebuild_source_binding(self) -> Duration {
        self.prebuild_source_binding
    }

    pub(super) const fn postbuild_binary_binding(self) -> Duration {
        self.postbuild_binary_binding
    }

    pub(super) const fn postbuild_source_binding(self) -> Duration {
        self.postbuild_source_binding
    }
}

fn require_campaign_source(
    expected: &PhysicalWorkSourceBinding,
    current: PhysicalWorkSourceBinding,
) -> Result<(), String> {
    if expected != &current {
        return Err("courtroom source changed during the campaign".to_owned());
    }
    Ok(())
}

fn require_stable_source(
    before: PhysicalWorkSourceBinding,
    after: PhysicalWorkSourceBinding,
) -> Result<PhysicalWorkSourceBinding, String> {
    if before != after {
        return Err(
            "courtroom source changed while its evidence binaries were being built".to_owned(),
        );
    }
    Ok(after)
}

fn bind_binary(target: &Path, name: &str) -> Result<BoundBinary, String> {
    let path = target.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let path = path
        .canonicalize()
        .map_err(|error| format!("cannot locate built binary {}: {error}", path.display()))?;
    let binding = PhysicalWorkSourceBinding::new(path.display().to_string(), hash_file(&path)?)
        .map_err(|denial| format!("binary evidence binding denied: {denial:?}"))?;
    Ok(BoundBinary { path, binding })
}

fn bind_current_binary() -> Result<BoundBinary, String> {
    let path = std::env::current_exe()
        .map_err(|error| format!("cannot locate current courtroom runner: {error}"))?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize current courtroom runner: {error}"))?;
    let binding = PhysicalWorkSourceBinding::new(path.display().to_string(), hash_file(&path)?)
        .map_err(|denial| format!("runner evidence binding denied: {denial:?}"))?;
    Ok(BoundBinary { path, binding })
}

fn target_directory(workspace: &Path) -> PathBuf {
    match std::env::var_os("CARGO_TARGET_DIR") {
        Some(target) if Path::new(&target).is_absolute() => PathBuf::from(target),
        Some(target) => workspace.join(target),
        None => workspace.join("target"),
    }
}

fn hash_file(path: &Path) -> Result<PhysicalWorkEvidenceDigest, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("cannot hash {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot hash {}: {error}", path.display()))?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    evidence_digest(digest.finalize().into(), &path.display().to_string())
}

fn evidence_digest(bytes: [u8; 32], subject: &str) -> Result<PhysicalWorkEvidenceDigest, String> {
    PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| format!("{subject} has an all-zero digest"))
}

#[cfg(test)]
mod tests {
    use super::{bind_binary, require_campaign_source, require_stable_source};
    use worth_store::physical_runtime::{PhysicalWorkEvidenceDigest, PhysicalWorkSourceBinding};

    #[test]
    fn stable_source_manifest_is_accepted_across_the_build_interval() {
        let before = source_binding(7);
        let after = source_binding(7);

        assert_eq!(
            require_stable_source(before.clone(), after).unwrap(),
            before
        );
    }

    #[test]
    fn source_drift_during_the_build_interval_is_rejected() {
        let before = source_binding(7);
        let after = source_binding(8);

        let error = require_stable_source(before, after).unwrap_err();
        assert!(error.contains("source changed"), "{error}");
    }

    #[test]
    fn source_drift_after_the_build_interval_is_rejected() {
        let bound = source_binding(7);
        let current = source_binding(8);

        let error = require_campaign_source(&bound, current).unwrap_err();
        assert!(error.contains("during the campaign"), "{error}");
    }

    #[test]
    fn bound_runner_replacement_during_the_campaign_is_rejected() {
        let target = tempfile::tempdir().unwrap();
        let executable = target
            .path()
            .join(format!("store-test-runner{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&executable, b"original executable").unwrap();
        let bound = bind_binary(target.path(), "store-test-runner").unwrap();
        std::fs::write(&executable, b"replacement executable").unwrap();

        let error = bound.verify_unchanged().unwrap_err();
        assert!(error.contains("executable changed"), "{error}");
    }

    fn source_binding(byte: u8) -> PhysicalWorkSourceBinding {
        PhysicalWorkSourceBinding::new(
            "test-source",
            PhysicalWorkEvidenceDigest::new([byte; 32]).unwrap(),
        )
        .unwrap()
    }
}
