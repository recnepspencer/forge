mod artifact_binding;
mod authority_contract;
mod cargo_invocation;
mod compiler_artifacts;
mod finalized_bundle;
mod metadata_graph;
mod source_snapshot;
mod target_directory;
mod targets;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use artifact_binding::bind;
use cargo_invocation::build_target;
use metadata_graph::{feature_graph, feature_roots, load, Metadata};
use source_snapshot::capture;
use targets::{recipe, Recipe, TargetSpec};

pub use artifact_binding::BoundArtifact;
pub use compiler_artifacts::CompilerArtifactRecord;
pub use finalized_bundle::{
    FinalizedFreshRecoveryProcessBundle, FINALIZED_OBSERVER_ENV, FINALIZED_RECOVERY_ENV,
    FINALIZED_WRITER_ENV,
};
pub use metadata_graph::FeatureNode;
pub use source_snapshot::{BoundSource, SourceWorkload};
pub use target_directory::{target_parent, FreshProcessCargoTarget};
pub use targets::{ObserverProcessRole, RecoveryProcessRole, WriterProcessRole};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BuildProfile {
    Debug,
    Release,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildTimings {
    source_discovery: Duration,
    source_before: Duration,
    cargo_build: Duration,
    artifact_binding: Duration,
    source_after: Duration,
}

impl BuildTimings {
    pub const fn source_discovery(self) -> Duration {
        self.source_discovery
    }

    pub const fn source_before(self) -> Duration {
        self.source_before
    }

    pub const fn cargo_build(self) -> Duration {
        self.cargo_build
    }

    pub const fn artifact_binding(self) -> Duration {
        self.artifact_binding
    }

    pub const fn source_after(self) -> Duration {
        self.source_after
    }
}

pub struct FreshRecoveryProcessBundle {
    cargo: OsString,
    workspace: PathBuf,
    repository: PathBuf,
    recipe: Recipe,
    source: BoundSource,
    feature_roots: Box<[String]>,
    feature_graph: Box<[FeatureNode]>,
    writer: BoundArtifact<WriterProcessRole>,
    observer: BoundArtifact<ObserverProcessRole>,
    recovery: BoundArtifact<RecoveryProcessRole>,
    timings: BuildTimings,
}

impl FreshRecoveryProcessBundle {
    pub fn build_production_finalized(
        workspace: &Path,
        repository: &Path,
    ) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
        finalized_bundle::build(recipe::production(), workspace, repository)
    }

    pub fn build_bounded_residency_finalized(
        workspace: &Path,
        repository: &Path,
    ) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
        finalized_bundle::build(recipe::bounded_residency(), workspace, repository)
    }

    pub fn build_production_finalized_at(
        workspace: &Path,
        repository: &Path,
        target_parent: &Path,
    ) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
        finalized_bundle::build_at(recipe::production(), workspace, repository, target_parent)
    }

    pub fn build_production(
        workspace: &Path,
        repository: &Path,
        target_directory: &FreshProcessCargoTarget,
    ) -> Result<Self, String> {
        build(
            recipe::production(),
            workspace,
            repository,
            target_directory,
        )
    }

    pub fn build_bounded_residency(
        workspace: &Path,
        repository: &Path,
        target_directory: &FreshProcessCargoTarget,
    ) -> Result<Self, String> {
        build(
            recipe::bounded_residency(),
            workspace,
            repository,
            target_directory,
        )
    }

    pub fn writer(&self) -> &BoundArtifact<WriterProcessRole> {
        &self.writer
    }

    pub fn observer(&self) -> &BoundArtifact<ObserverProcessRole> {
        &self.observer
    }

    pub fn recovery(&self) -> &BoundArtifact<RecoveryProcessRole> {
        &self.recovery
    }

    pub fn source(&self) -> &BoundSource {
        &self.source
    }

    pub fn feature_graph(&self) -> &[FeatureNode] {
        &self.feature_graph
    }

    pub fn feature_roots(&self) -> &[String] {
        &self.feature_roots
    }

    pub fn timings(&self) -> BuildTimings {
        self.timings
    }

    pub fn verify_source_unchanged(&self) -> Result<SourceWorkload, String> {
        let metadata = load(
            &self.cargo,
            &self.workspace,
            self.recipe.metadata_features(),
        )?;
        let snapshot = capture(
            &metadata,
            &self.repository,
            &self.recipe.targets,
            self.recipe.source_packages,
        )?;
        if snapshot.bound != self.source {
            return Err("source closure changed during the campaign".to_owned());
        }
        Ok(snapshot.bound.workload())
    }

    pub fn verify_executables_unchanged(&self) -> Result<(), String> {
        self.writer.verify_unchanged()?;
        self.observer.verify_unchanged()?;
        self.recovery.verify_unchanged()
    }
}

fn build(
    recipe: Recipe,
    workspace: &Path,
    repository: &Path,
    target_directory: &FreshProcessCargoTarget,
) -> Result<FreshRecoveryProcessBundle, String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let workspace = workspace
        .canonicalize()
        .map_err(|error| format!("canonicalize process-bundle workspace: {error}"))?;
    let repository = repository
        .canonicalize()
        .map_err(|error| format!("canonicalize process-bundle repository: {error}"))?;
    let discovery_started = Instant::now();
    let metadata = load(&cargo, &workspace, recipe.metadata_features())?;
    let source_discovery = discovery_started.elapsed();
    let before_started = Instant::now();
    let before = capture(
        &metadata,
        &repository,
        &recipe.targets,
        recipe.source_packages,
    )?;
    let source_before = before_started.elapsed();

    let build_started = Instant::now();
    let writer = build_role(
        &cargo,
        &workspace,
        &target_directory,
        &metadata,
        &recipe.targets.writer,
        recipe.profile,
    )?;
    let observer = build_role(
        &cargo,
        &workspace,
        &target_directory,
        &metadata,
        &recipe.targets.observer,
        recipe.profile,
    )?;
    let recovery = build_role(
        &cargo,
        &workspace,
        &target_directory,
        &metadata,
        &recipe.targets.recovery,
        recipe.profile,
    )?;
    let cargo_build = build_started.elapsed();

    let artifact_started = Instant::now();
    let writer = bind(writer)?;
    let observer = bind(observer)?;
    let recovery = bind(recovery)?;
    let artifact_binding = artifact_started.elapsed();

    let after_started = Instant::now();
    let after_metadata = load(&cargo, &workspace, recipe.metadata_features())?;
    let after = capture(
        &after_metadata,
        &repository,
        &recipe.targets,
        recipe.source_packages,
    )?;
    if before != after {
        return Err(format!(
            "source closure changed during the build: before={:x?} after={:x?}",
            before.bound.digest(),
            after.bound.digest()
        ));
    }
    let source_after = after_started.elapsed();

    authority_contract::verify(&recipe, &writer, &observer, &recovery)?;
    let feature_graph = feature_graph(&metadata, recipe.source_packages).into_boxed_slice();
    let feature_roots = feature_roots(&metadata, recipe.source_packages).into_boxed_slice();
    Ok(FreshRecoveryProcessBundle {
        cargo,
        workspace,
        repository,
        recipe,
        source: after.bound,
        feature_roots,
        feature_graph,
        writer,
        observer,
        recovery,
        timings: BuildTimings {
            source_discovery,
            source_before,
            cargo_build,
            artifact_binding,
            source_after,
        },
    })
}

fn build_role<R>(
    cargo: &std::ffi::OsStr,
    workspace: &Path,
    target_directory: &FreshProcessCargoTarget,
    metadata: &Metadata,
    target: &TargetSpec<R>,
    profile: BuildProfile,
) -> Result<cargo_invocation::BuiltTarget<R>, String> {
    build_target(
        cargo,
        workspace,
        target_directory,
        metadata,
        target,
        profile,
    )
}
