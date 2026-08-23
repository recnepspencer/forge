use std::path::Path;

use super::directory;
use crate::fresh_recovery_processes::artifact_binding::BoundArtifact;
use crate::fresh_recovery_processes::targets::{
    ObserverProcessRole, RecoveryProcessRole, WriterProcessRole,
};
use crate::fresh_recovery_processes::FreshRecoveryProcessBundle;

pub(super) struct PromotedArtifacts {
    pub(super) writer: BoundArtifact<WriterProcessRole>,
    pub(super) observer: BoundArtifact<ObserverProcessRole>,
    pub(super) recovery: BoundArtifact<RecoveryProcessRole>,
}

pub(super) fn promote(
    bundle: &FreshRecoveryProcessBundle,
    directory: &Path,
) -> Result<PromotedArtifacts, String> {
    let writer = promote_artifact(bundle.writer(), directory, "writer")?;
    let observer = promote_artifact(bundle.observer(), directory, "observer")?;
    let recovery = promote_artifact(bundle.recovery(), directory, "recovery")?;
    directory::seal(directory)?;
    Ok(PromotedArtifacts {
        writer,
        observer,
        recovery,
    })
}

pub(super) fn promote_artifact<R>(
    artifact: &BoundArtifact<R>,
    directory: &Path,
    role: &str,
) -> Result<BoundArtifact<R>, String> {
    let extension = artifact
        .path()
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| format!(".{extension}"))
        .unwrap_or_default();
    let destination = directory.join(format!("{role}{extension}"));
    std::fs::copy(artifact.path(), &destination).map_err(|error| {
        format!(
            "promote source-bound {role} executable to {}: {error}",
            destination.display()
        )
    })?;
    artifact.rebind_promoted(destination)
}
