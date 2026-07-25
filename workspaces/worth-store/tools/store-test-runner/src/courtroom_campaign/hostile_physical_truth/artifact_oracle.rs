use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU32;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::PhysicalWorkHostileTruthScenario;

use super::offline_protocol::{OfflineArtifactObservation, OfflineObservation};

pub(super) fn validate_transition(
    scenario: PhysicalWorkHostileTruthScenario,
    baseline: &OfflineObservation,
    observed: &OfflineObservation,
) -> Result<(), String> {
    if baseline.artifacts().is_empty() || observed.artifacts().is_empty() {
        return Err("Courtroom B requires nonempty exact artifact inventories".into());
    }
    if baseline.recovery_obligations() != 0 {
        return Err("seeded baseline unexpectedly retained a recovery obligation".into());
    }
    let baseline_media = effect_artifacts(baseline.artifacts());
    let observed_media = effect_artifacts(observed.artifacts());
    let transition = ArtifactTransition {
        baseline: &baseline_media,
        observed: &observed_media,
        recovery_obligations: observed.recovery_obligations(),
        generation: observed.current().generation(),
    };
    match scenario {
        PhysicalWorkHostileTruthScenario::BeforeBackendDispatch => transition.before_dispatch(),
        PhysicalWorkHostileTruthScenario::DuringShortWrite => transition.short_write(),
        PhysicalWorkHostileTruthScenario::AfterExactWriteBeforeSchedulerSettlement => {
            transition.exact_write()
        }
        PhysicalWorkHostileTruthScenario::DuringRootPublication => transition.publication(),
        PhysicalWorkHostileTruthScenario::DuringShutdown => transition.shutdown(),
    }
}

struct ArtifactTransition<'artifact> {
    baseline: &'artifact [&'artifact OfflineArtifactObservation],
    observed: &'artifact [&'artifact OfflineArtifactObservation],
    recovery_obligations: u64,
    generation: u64,
}

impl ArtifactTransition<'_> {
    fn before_dispatch(&self) -> Result<(), String> {
        require(
            self.recovery_obligations == 0,
            "pre-dispatch death invented a recovery obligation",
        )?;
        require(
            self.baseline == self.observed,
            "pre-dispatch kill changed a media artifact",
        )
    }

    fn short_write(&self) -> Result<(), String> {
        require(
            self.recovery_obligations == 1,
            "missing short-write obligation",
        )?;
        require_exact_short_write_residue(self.baseline, self.observed)
    }

    fn exact_write(&self) -> Result<(), String> {
        require(
            self.recovery_obligations == 1,
            "missing exact-write obligation",
        )?;
        require(
            self.baseline == self.observed,
            "exact overwrite changed bytes or collateral media",
        )
    }

    fn publication(&self) -> Result<(), String> {
        require(
            self.recovery_obligations == 1,
            "missing publication obligation",
        )?;
        require_exact_publication_delta(self.baseline, self.observed, self.generation)
    }

    fn shutdown(&self) -> Result<(), String> {
        require(
            self.recovery_obligations == 0,
            "shutdown invented a recovery obligation",
        )?;
        require(
            self.baseline == self.observed,
            "shutdown changed an effect artifact",
        )
    }
}

pub(super) fn effect_artifacts(
    artifacts: &[OfflineArtifactObservation],
) -> Vec<&OfflineArtifactObservation> {
    artifacts
        .iter()
        .filter(|artifact| {
            !artifact.is_recovery_obligation() && artifact.path() != "namespace/mutation.lock"
        })
        .collect()
}

pub(super) fn validate_mutation_coordination(
    artifacts: &[OfflineArtifactObservation],
    expected_process: NonZeroU32,
    label: &str,
) -> Result<(), String> {
    let matching = artifacts
        .iter()
        .filter(|artifact| artifact.path() == "namespace/mutation.lock")
        .collect::<Vec<_>>();
    let [artifact] = matching.as_slice() else {
        return Err(format!("{label} did not contain exactly one mutation lock"));
    };
    if artifact.byte_length() != artifact.prefix().len() as u64
        || Sha256::digest(artifact.prefix()).as_slice() != artifact.digest()
    {
        return Err(format!("{label} mutation-lock bytes were not exact"));
    }
    let text = std::str::from_utf8(artifact.prefix())
        .map_err(|_| format!("{label} mutation lock was not UTF-8"))?;
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() != 4
        || lines[0] != "version=1"
        || lines[1] != format!("process={:010}", expected_process.get())
        || !valid_identity_line(lines[2], "runtime=")
        || !valid_identity_line(lines[3], "attempt=")
    {
        return Err(format!("{label} mutation lock was not process-bound"));
    }
    Ok(())
}

pub(super) fn require_exact_short_write_residue(
    baseline: &[&OfflineArtifactObservation],
    observed: &[&OfflineArtifactObservation],
) -> Result<(), String> {
    let baseline_by_path = unique_artifacts(baseline, "baseline")?;
    let observed_by_path = unique_artifacts(observed, "short-write observation")?;
    for (path, prior) in &baseline_by_path {
        let current = observed_by_path
            .get(path)
            .ok_or_else(|| format!("short-write kill removed media artifact `{path}`"))?;
        if current != prior {
            return Err(format!("short-write kill modified media artifact `{path}`"));
        }
    }
    let added = observed_by_path
        .iter()
        .filter(|(path, _)| !baseline_by_path.contains_key(*path))
        .map(|(_, artifact)| *artifact)
        .collect::<Vec<_>>();
    let [residue] = added.as_slice() else {
        return Err(format!(
            "short-write kill requires exactly one new residue artifact, found {}",
            added.len()
        ));
    };
    let digest: [u8; 32] = Sha256::digest(residue.prefix()).into();
    require(
        residue.byte_length() == 1 && residue.prefix().len() == 1 && residue.digest() == digest,
        "short-write residue was not one exact independently hashed byte",
    )
}

pub(super) fn require_exact_publication_delta(
    baseline: &[&OfflineArtifactObservation],
    observed: &[&OfflineArtifactObservation],
    generation: u64,
) -> Result<(), String> {
    let baseline_by_path = unique_artifacts(baseline, "baseline")?;
    let observed_by_path = unique_artifacts(observed, "publication observation")?;
    require_publication_survival(&baseline_by_path, &observed_by_path)?;
    let added = observed_by_path
        .keys()
        .filter(|path| !baseline_by_path.contains_key(*path))
        .copied()
        .collect::<BTreeSet<_>>();
    require(
        added.len() == 6,
        "publication requires exactly six new durable record artifacts",
    )?;
    require_publication_artifact_set(&added, generation)
}

fn require_publication_survival(
    baseline: &BTreeMap<&str, &OfflineArtifactObservation>,
    observed: &BTreeMap<&str, &OfflineArtifactObservation>,
) -> Result<(), String> {
    let mut catalog_changed = false;
    for (path, prior) in baseline {
        let current = observed
            .get(path)
            .ok_or_else(|| format!("publication removed baseline artifact `{path}`"))?;
        if *path == "families/records/bootstrap.catalog" {
            catalog_changed = current != prior;
        } else if current != prior {
            return Err(format!("publication modified collateral artifact `{path}`"));
        }
    }
    require(
        catalog_changed,
        "publication did not replace the bootstrap catalog",
    )
}

fn require_publication_artifact_set(added: &BTreeSet<&str>, generation: u64) -> Result<(), String> {
    let extent = unique_stem(
        added,
        "families/records/extents/",
        ".data",
        "extent payload",
    )?;
    let extent_manifest = unique_stem(
        added,
        "families/records/extent-manifests/",
        ".manifest",
        "extent manifest",
    )?;
    require(
        extent == extent_manifest && valid_extent_stem(extent),
        "publication extent and manifest identities disagree",
    )?;
    let generation = format!("{generation:016x}");
    let exact = [
        format!("families/records/free-space/free-space-{generation}.manifest"),
        format!("families/records/roots/root-{generation}.manifest"),
    ];
    require(
        exact.iter().all(|path| added.contains(path.as_str())),
        "publication omitted its current-generation aggregate manifest",
    )?;
    let free_space_block = format!("families/records/free-space/free-space-{generation}-block-");
    let root_block = format!("families/records/roots/root-{generation}-block-");
    require(
        one_fixed_hex_block(added, &free_space_block) && one_fixed_hex_block(added, &root_block),
        "publication omitted or multiplied its current-generation routing block",
    )
}

fn unique_stem<'path>(
    paths: &BTreeSet<&'path str>,
    prefix: &str,
    suffix: &str,
    label: &str,
) -> Result<&'path str, String> {
    let matching = paths
        .iter()
        .filter_map(|path| path.strip_prefix(prefix)?.strip_suffix(suffix))
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [stem] => Ok(*stem),
        _ => Err(format!(
            "publication requires one {label}, found {}",
            matching.len()
        )),
    }
}

fn valid_extent_stem(stem: &str) -> bool {
    let Some(identity) = stem.strip_prefix("extent-") else {
        return false;
    };
    let Some((extent, generation)) = identity.split_once('-') else {
        return false;
    };
    fixed_nonzero_hex(extent) && fixed_nonzero_hex(generation)
}

fn one_fixed_hex_block(paths: &BTreeSet<&str>, prefix: &str) -> bool {
    paths
        .iter()
        .filter_map(|path| path.strip_prefix(prefix)?.strip_suffix(".manifest"))
        .filter(|identity| fixed_nonzero_hex(identity))
        .count()
        == 1
}

fn fixed_nonzero_hex(encoded: &str) -> bool {
    encoded.len() == 16
        && encoded.bytes().all(|byte| byte.is_ascii_hexdigit())
        && encoded.bytes().any(|byte| byte != b'0')
}

fn unique_artifacts<'artifact>(
    artifacts: &[&'artifact OfflineArtifactObservation],
    label: &str,
) -> Result<BTreeMap<&'artifact str, &'artifact OfflineArtifactObservation>, String> {
    let mut indexed = BTreeMap::new();
    for artifact in artifacts {
        if indexed.insert(artifact.path(), *artifact).is_some() {
            return Err(format!("{label} repeated artifact `{}`", artifact.path()));
        }
    }
    Ok(indexed)
}

fn valid_identity_line(line: &str, prefix: &str) -> bool {
    let Some(encoded) = line.strip_prefix(prefix) else {
        return false;
    };
    encoded.len() == 32
        && encoded.bytes().all(|byte| byte.is_ascii_hexdigit())
        && encoded.bytes().any(|byte| byte != b'0')
}

fn require(predicate: bool, failure: &str) -> Result<(), String> {
    if predicate {
        Ok(())
    } else {
        Err(failure.to_owned())
    }
}
