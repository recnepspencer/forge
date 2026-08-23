use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const WINDOWS_CARGO_TARGET_PATH_BUDGET: usize = 160;

/// A Cargo target directory allocated exclusively for one certification campaign.
///
/// The value cannot be constructed from an existing path.  Callers must retain it
/// for as long as the bound artifacts are used and must explicitly close it when
/// the campaign has released those artifacts.
pub struct FreshProcessCargoTarget {
    path: PathBuf,
}

impl FreshProcessCargoTarget {
    pub fn allocate(parent: &Path) -> Result<Self, String> {
        let parent = parent
            .canonicalize()
            .or_else(|_| std::fs::create_dir_all(parent).and_then(|()| parent.canonicalize()))
            .map(normalize_windows_path)
            .map_err(|error| format!("prepare fresh-process Cargo target parent: {error}"))?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("clock before fresh-process target allocation: {error}"))?
            .as_nanos();
        let process = std::process::id();
        for attempt in 0..64_u32 {
            let candidate = parent.join(format!(
                ".worth-store-fresh-process-{process}-{nonce}-{attempt}"
            ));
            let candidate = if candidate.to_string_lossy().len() > WINDOWS_CARGO_TARGET_PATH_BUDGET
            {
                let compact = parent.join(format!(".wsp-{process}-{nonce}-{attempt}"));
                if compact.to_string_lossy().len() > WINDOWS_CARGO_TARGET_PATH_BUDGET {
                    parent.join(format!(".w-{process}-{attempt}"))
                } else {
                    compact
                }
            } else {
                candidate
            };
            match std::fs::create_dir(&candidate) {
                Ok(()) => {
                    let path = candidate
                        .canonicalize()
                        .map(normalize_windows_path)
                        .map_err(|error| {
                            format!("canonicalize allocated fresh-process Cargo target: {error}")
                        })?;
                    if path == parent {
                        return Err(
                            "fresh-process target unexpectedly equals its parent".to_owned()
                        );
                    }
                    return Ok(Self { path });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(format!(
                        "allocate exclusive fresh-process Cargo target {}: {error}",
                        candidate.display()
                    ));
                }
            }
        }
        Err(format!(
            "could not allocate an exclusive fresh-process Cargo target beneath {}",
            parent.display()
        ))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn close(self) -> Result<(), String> {
        let path = self.path;
        if path.exists() {
            std::fs::remove_dir_all(&path).map_err(|error| {
                format!(
                    "remove fresh-process Cargo target {}: {error}",
                    path.display()
                )
            })?;
        }
        if path.exists() {
            return Err(format!(
                "fresh-process Cargo target remained after close: {}",
                path.display()
            ));
        }
        Ok(())
    }
}

fn normalize_windows_path(path: PathBuf) -> PathBuf {
    let normalized = path
        .to_string_lossy()
        .strip_prefix(r"\\?\")
        .map(str::to_owned);
    normalized.map_or(path, PathBuf::from)
}

pub fn target_parent(workspace: &Path) -> PathBuf {
    match std::env::var_os("CARGO_TARGET_DIR") {
        Some(target) if Path::new(&target).is_absolute() => PathBuf::from(target),
        Some(target) => workspace.join(target),
        None => workspace.join("target"),
    }
}

#[cfg(test)]
mod tests {
    use super::FreshProcessCargoTarget;

    #[test]
    fn allocations_are_distinct_children_and_close_explicitly() {
        let parent = std::env::temp_dir().join(format!(
            "worth-store-process-bundle-target-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&parent);
        std::fs::create_dir(&parent).unwrap();
        let first = FreshProcessCargoTarget::allocate(&parent)
            .unwrap_or_else(|error| panic!("MUTANT_PREDICATE:c8-fresh-target-isolation {error}"));
        let second = FreshProcessCargoTarget::allocate(&parent)
            .unwrap_or_else(|error| panic!("MUTANT_PREDICATE:c8-fresh-target-isolation {error}"));
        assert_ne!(first.path(), second.path());
        assert!(first.path().starts_with(&parent));
        assert!(second.path().starts_with(&parent));
        let first_path = first.path().to_owned();
        let second_path = second.path().to_owned();
        first.close().unwrap();
        second.close().unwrap();
        assert!(!first_path.exists());
        assert!(!second_path.exists());
        std::fs::remove_dir(&parent).unwrap();
    }

    #[test]
    fn close_reports_a_target_cleanup_failure() {
        let parent = std::env::temp_dir().join(format!(
            "worth-store-process-bundle-target-failure-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&parent);
        std::fs::create_dir(&parent).unwrap();
        let path = parent.join("target-file");
        std::fs::write(&path, b"not a directory").unwrap();
        let target = FreshProcessCargoTarget { path: path.clone() };

        let error = target.close().unwrap_err();

        assert!(
            error.contains("remove fresh-process Cargo target"),
            "{error}"
        );
        assert!(path.is_file());
        std::fs::remove_dir_all(parent).unwrap();
    }
}
