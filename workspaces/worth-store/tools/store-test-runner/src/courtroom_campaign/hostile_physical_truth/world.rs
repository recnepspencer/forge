use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const CONFIGURATION_SCHEMA: &str = "worth.store.c5_1.physical-work-courtroom.configuration.v1";
const PAYLOAD_BYTES: usize = 16 * 1024;
const CAMPAIGN_SEED: u64 = 0xc510_00b0_0001;
static CAMPAIGN_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) struct CampaignWorld {
    ownership: CampaignRootOwnership,
    configuration: PathBuf,
    seed_oracle: PathBuf,
    mutation_oracle: PathBuf,
}

enum CampaignRootOwnership {
    Temporary(tempfile::TempDir),
    Retained(PathBuf),
}

impl CampaignWorld {
    pub(super) fn create(target_root: Option<&Path>) -> Result<Self, String> {
        let ownership = match target_root {
            Some(target) => CampaignRootOwnership::Retained(create_retained_root(target)?),
            None => CampaignRootOwnership::Temporary(
                tempfile::Builder::new()
                    .prefix("worth-store-c5-1-courtroom-b-")
                    .tempdir()
                    .map_err(|error| format!("cannot create Courtroom B world: {error}"))?,
            ),
        };
        let root = ownership.path();
        let configuration = root.join("configuration");
        let seed_oracle = root.join("seed.oracle");
        let mutation_oracle = root.join("mutation.oracle");
        std::fs::write(
            &configuration,
            format!("{CONFIGURATION_SCHEMA}\npayload-bytes={PAYLOAD_BYTES}\n"),
        )
        .map_err(|error| format!("cannot write Courtroom B configuration: {error}"))?;
        std::fs::write(&seed_oracle, payload(CAMPAIGN_SEED))
            .map_err(|error| format!("cannot write Courtroom B seed oracle: {error}"))?;
        std::fs::write(&mutation_oracle, payload(!CAMPAIGN_SEED))
            .map_err(|error| format!("cannot write Courtroom B mutation oracle: {error}"))?;
        Ok(Self {
            ownership,
            configuration,
            seed_oracle,
            mutation_oracle,
        })
    }

    pub(super) fn root(&self) -> &Path {
        self.ownership.path()
    }

    pub(super) fn scenario_root(&self, label: &str) -> Result<PathBuf, String> {
        let root = self.root().join(label).join("store");
        std::fs::create_dir_all(&root)
            .map_err(|error| format!("cannot create scenario root {}: {error}", root.display()))?;
        Ok(root)
    }

    pub(super) const fn payload_bytes(&self) -> usize {
        PAYLOAD_BYTES
    }

    pub(super) fn configuration(&self) -> &Path {
        &self.configuration
    }

    pub(super) fn seed_oracle(&self) -> &Path {
        &self.seed_oracle
    }

    pub(super) fn mutation_oracle(&self) -> &Path {
        &self.mutation_oracle
    }

    pub(super) const fn seed(&self) -> u64 {
        CAMPAIGN_SEED
    }
}

impl CampaignRootOwnership {
    fn path(&self) -> &Path {
        match self {
            Self::Temporary(root) => root.path(),
            Self::Retained(root) => root,
        }
    }
}

fn create_retained_root(target: &Path) -> Result<PathBuf, String> {
    let target = if target.is_absolute() {
        target.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("cannot resolve Courtroom B current directory: {error}"))?
            .join(target)
    };
    if target.to_string_lossy().starts_with(r"\\?\") {
        return Err("Courtroom B target root cannot use the Windows verbatim namespace".into());
    }
    std::fs::create_dir_all(&target)
        .map_err(|error| format!("cannot create Courtroom B target root: {error}"))?;
    for _ in 0..32 {
        let sequence = CAMPAIGN_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = target.join(format!("courtroom-b-{}-{sequence}", std::process::id()));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create retained Courtroom B root {}: {error}",
                    candidate.display()
                ))
            }
        }
    }
    Err("cannot allocate a unique retained Courtroom B root".into())
}

fn payload(mut state: u64) -> Vec<u8> {
    let mut bytes = vec![0_u8; PAYLOAD_BYTES];
    for byte in &mut bytes {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::CampaignWorld;

    #[test]
    fn world_has_distinct_fixed_payloads_and_isolated_scenario_roots() {
        let world = CampaignWorld::create(None).unwrap();
        assert_eq!(std::fs::read(world.seed_oracle()).unwrap().len(), 16 * 1024);
        assert_ne!(
            std::fs::read(world.seed_oracle()).unwrap(),
            std::fs::read(world.mutation_oracle()).unwrap()
        );
        assert_ne!(
            world.scenario_root("first").unwrap(),
            world.scenario_root("second").unwrap()
        );
    }

    #[test]
    fn retained_world_resolves_relative_target_without_a_verbatim_namespace() {
        let current = std::env::current_dir().unwrap().canonicalize().unwrap();
        let temporary = tempfile::Builder::new()
            .prefix("courtroom-b-relative-")
            .tempdir_in(&current)
            .unwrap();
        let relative = temporary
            .path()
            .canonicalize()
            .unwrap()
            .strip_prefix(&current)
            .unwrap()
            .to_path_buf();
        assert!(!relative.is_absolute());
        let world = CampaignWorld::create(Some(Path::new(&relative))).unwrap();
        let root = world.scenario_root("absolute").unwrap();
        assert!(root.is_absolute());
        assert!(!root.to_string_lossy().starts_with(r"\\?\"));
    }
}
