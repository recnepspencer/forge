use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceDigest, PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence,
    PhysicalWorkSourceBinding,
};
use worth_store_process_bundle::{
    target_parent, FreshProcessCargoTarget, FreshRecoveryProcessBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SourceClosureWorkload {
    source_files: u64,
    source_bytes: u64,
}

impl SourceClosureWorkload {
    pub(super) const fn new(source_files: u64, source_bytes: u64) -> Self {
        Self {
            source_files,
            source_bytes,
        }
    }

    pub(super) const fn source_files(self) -> u64 {
        self.source_files
    }

    pub(super) const fn source_bytes(self) -> u64 {
        self.source_bytes
    }
}

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
    bundle: FreshRecoveryProcessBundle,
    _target: FreshProcessCargoTarget,
    source: PhysicalWorkSourceBinding,
    feature_graph: PhysicalWorkFeatureGraphEvidence,
    runner: BoundBinary,
    writer: BoundBinary,
    observer: BoundBinary,
    recovery: BoundBinary,
    cargo_build_elapsed: Duration,
    binding_timings: ExecutableBindingTimings,
}

impl BuiltCourtroomExecutables {
    pub(super) fn build(workspace: &Path) -> Result<Self, String> {
        let repository = workspace
            .join("../..")
            .canonicalize()
            .map_err(|error| format!("canonicalize courtroom repository: {error}"))?;
        let target = FreshProcessCargoTarget::allocate(&target_parent(workspace))?;
        let bundle =
            FreshRecoveryProcessBundle::build_bounded_residency(workspace, &repository, &target)?;
        let source = source_binding(bundle.source())?;
        let feature_graph = feature_graph(&bundle)?;
        let runner = bind_current_binary()?;
        let writer = bind_artifact(bundle.writer())?;
        let observer = bind_artifact(bundle.observer())?;
        let recovery = bind_artifact(bundle.recovery())?;
        let timings = bundle.timings();
        Ok(Self {
            bundle,
            _target: target,
            source,
            feature_graph,
            runner,
            writer,
            observer,
            recovery,
            cargo_build_elapsed: timings.cargo_build(),
            binding_timings: ExecutableBindingTimings {
                source_inventory: timings.source_discovery(),
                prebuild_source_binding: timings.source_before(),
                postbuild_binary_binding: timings.artifact_binding(),
                postbuild_source_binding: timings.source_after(),
            },
        })
    }

    pub(super) const fn source(&self) -> &PhysicalWorkSourceBinding {
        &self.source
    }

    pub(super) const fn feature_graph(&self) -> &PhysicalWorkFeatureGraphEvidence {
        &self.feature_graph
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

    pub(super) const fn recovery(&self) -> &BoundBinary {
        &self.recovery
    }

    pub(super) fn verify_source_unchanged(&self) -> Result<SourceClosureWorkload, String> {
        self.bundle.verify_source_unchanged().map(|workload| {
            SourceClosureWorkload::new(workload.source_files(), workload.source_bytes())
        })
    }

    pub(super) fn verify_executables_unchanged(&self) -> Result<(), String> {
        self.runner.verify_unchanged()?;
        self.bundle.verify_executables_unchanged()
    }

    pub(super) const fn cargo_build_elapsed(&self) -> Duration {
        self.cargo_build_elapsed
    }

    pub(super) const fn binding_timings(&self) -> ExecutableBindingTimings {
        self.binding_timings
    }

    pub(super) fn source_workload(&self) -> SourceClosureWorkload {
        SourceClosureWorkload::new(
            self.bundle.source().workload().source_files(),
            self.bundle.source().workload().source_bytes(),
        )
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

fn source_binding(
    source: &worth_store_process_bundle::BoundSource,
) -> Result<PhysicalWorkSourceBinding, String> {
    let digest = evidence_digest(source.digest(), "courtroom source closure")?;
    PhysicalWorkSourceBinding::new(
        format!(
            "{}#c8-process-bundle-source-closure",
            source.repository().display()
        ),
        digest,
    )
    .map_err(|denial| format!("source evidence binding denied: {denial:?}"))
}

fn feature_graph(
    bundle: &FreshRecoveryProcessBundle,
) -> Result<PhysicalWorkFeatureGraphEvidence, String> {
    let nodes = bundle
        .feature_graph()
        .iter()
        .map(|node| {
            PhysicalWorkFeatureNodeEvidence::new(
                node.package_id().to_owned(),
                node.features().iter().cloned(),
                node.dependencies().iter().cloned(),
            )
            .map_err(|denial| format!("process-bundle feature node denied: {denial:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    PhysicalWorkFeatureGraphEvidence::new(bundle.feature_roots().iter().cloned(), nodes)
        .map_err(|denial| format!("process-bundle feature graph denied: {denial:?}"))
}

fn bind_artifact<R>(
    artifact: &worth_store_process_bundle::BoundArtifact<R>,
) -> Result<BoundBinary, String> {
    let path = artifact.path().to_owned();
    let binding = PhysicalWorkSourceBinding::new(
        path.display().to_string(),
        evidence_digest(artifact.digest(), &path.display().to_string())?,
    )
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

#[cfg(test)]
fn bind_binary(target: &Path, name: &str) -> Result<BoundBinary, String> {
    let path = target.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let path = path
        .canonicalize()
        .map_err(|error| format!("cannot locate built binary {}: {error}", path.display()))?;
    let binding = PhysicalWorkSourceBinding::new(path.display().to_string(), hash_file(&path)?)
        .map_err(|denial| format!("binary evidence binding denied: {denial:?}"))?;
    Ok(BoundBinary { path, binding })
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
    use super::bind_binary;

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
}
