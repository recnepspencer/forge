use super::baseline::{dag_path, facade_path};
use super::document::{CrateDagDocument, FacadeDocument};
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn commit_snapshot_pair(
    root: &Path,
    dag: &CrateDagDocument,
    facades: &FacadeDocument,
) -> Result<Vec<PathBuf>, String> {
    let paths = [dag_path(root), facade_path(root)];
    let rendered = [
        toml::to_string_pretty(dag).map_err(|e| e.to_string())?,
        toml::to_string_pretty(facades).map_err(|e| e.to_string())?,
    ];
    replace_pair(&paths, &rendered, |from, to| fs::rename(from, to))?;
    Ok(paths.into())
}

fn replace_pair<F>(
    paths: &[PathBuf; 2],
    rendered: &[String; 2],
    mut rename: F,
) -> Result<(), String>
where
    F: FnMut(&Path, &Path) -> std::io::Result<()>,
{
    let nonce = format!("{}.snapshot-update", std::process::id());
    let stages = paths
        .clone()
        .map(|path| path.with_extension(format!("toml.{nonce}.stage")));
    let backups = paths
        .clone()
        .map(|path| path.with_extension(format!("toml.{nonce}.backup")));
    for ((path, stage), text) in paths.iter().zip(&stages).zip(rendered) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
        }
        fs::write(stage, text).map_err(|e| format!("stage {}: {e}", stage.display()))?;
    }
    let existed = paths.clone().map(|path| path.exists());
    for index in 0..2 {
        if existed[index] {
            if let Err(error) = rename(&paths[index], &backups[index]) {
                rollback_backups(paths, &backups, &existed, index, &mut rename);
                cleanup(&stages);
                return Err(format!("backup {}: {error}", paths[index].display()));
            }
        }
    }
    for index in 0..2 {
        if let Err(error) = rename(&stages[index], &paths[index]) {
            for committed in 0..index {
                let _ = fs::remove_file(&paths[committed]);
            }
            rollback_backups(paths, &backups, &existed, 2, &mut rename);
            cleanup(&stages);
            return Err(format!("replace {}: {error}", paths[index].display()));
        }
    }
    cleanup(&backups);
    Ok(())
}

fn rollback_backups<F>(
    paths: &[PathBuf; 2],
    backups: &[PathBuf; 2],
    existed: &[bool; 2],
    count: usize,
    rename: &mut F,
) where
    F: FnMut(&Path, &Path) -> std::io::Result<()>,
{
    for index in 0..count {
        if existed[index] && backups[index].exists() {
            let _ = rename(&backups[index], &paths[index]);
        }
    }
}

fn cleanup(paths: &[PathBuf; 2]) {
    for path in paths {
        let _ = fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn second_replacement_failure_restores_the_original_pair() {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("boundary-snapshot-commit-{id}"));
        let paths = [root.join("crate-dag.toml"), root.join("facades.toml")];
        fs::create_dir_all(&root).unwrap();
        fs::write(&paths[0], "old dag").unwrap();
        fs::write(&paths[1], "old facades").unwrap();
        let mut replacements = 0;
        let result = replace_pair(
            &paths,
            &["new dag".into(), "new facades".into()],
            |from, to| {
                if from.extension().and_then(|value| value.to_str()) == Some("stage") {
                    replacements += 1;
                    if replacements == 2 {
                        return Err(std::io::Error::other("injected second replacement failure"));
                    }
                }
                fs::rename(from, to)
            },
        );
        assert!(result.is_err());
        assert_eq!(fs::read_to_string(&paths[0]).unwrap(), "old dag");
        assert_eq!(fs::read_to_string(&paths[1]).unwrap(), "old facades");
    }
}
