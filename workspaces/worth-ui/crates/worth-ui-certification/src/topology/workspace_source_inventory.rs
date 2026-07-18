use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct WorkspaceSourceFile {
    absolute_path: PathBuf,
    relative_path: PathBuf,
    text: Arc<str>,
}

impl WorkspaceSourceFile {
    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug)]
pub struct WorkspaceSourceInventory {
    root: PathBuf,
    entries: BTreeSet<PathBuf>,
    text_files: BTreeMap<PathBuf, WorkspaceSourceFile>,
}

impl WorkspaceSourceInventory {
    pub fn capture(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        let mut inventory = Self {
            root,
            entries: BTreeSet::new(),
            text_files: BTreeMap::new(),
        };
        inventory.capture_directory(Path::new(""));
        inventory
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn absolute_path(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        self.root.join(relative_path)
    }

    pub fn contains(&self, relative_path: impl AsRef<Path>) -> bool {
        self.entries.contains(relative_path.as_ref())
    }

    pub fn source(&self, path: impl AsRef<Path>) -> Option<&WorkspaceSourceFile> {
        let relative_path = self.relative_path(path.as_ref())?;
        self.text_files.get(relative_path)
    }

    pub fn text(&self, path: impl AsRef<Path>) -> &str {
        let path = path.as_ref();
        self.source(path)
            .unwrap_or_else(|| panic!("{} is not a captured source file", path.display()))
            .text()
    }

    pub fn rust_files_under(
        &self,
        relative_root: impl AsRef<Path>,
    ) -> impl Iterator<Item = &WorkspaceSourceFile> {
        let relative_root = relative_root.as_ref().to_path_buf();
        self.text_files.values().filter(move |source| {
            source.relative_path.starts_with(&relative_root)
                && source
                    .relative_path
                    .extension()
                    .is_some_and(|extension| extension == "rs")
        })
    }

    pub fn rust_file_count(&self) -> usize {
        self.text_files
            .keys()
            .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
            .count()
    }

    pub fn direct_entries_under(
        &self,
        relative_root: impl AsRef<Path>,
    ) -> impl Iterator<Item = &Path> {
        let relative_root = relative_root.as_ref().to_path_buf();
        self.entries.iter().filter_map(move |entry| {
            (entry.parent() == Some(relative_root.as_path())).then_some(entry.as_path())
        })
    }

    pub fn entries_under(&self, relative_root: impl AsRef<Path>) -> impl Iterator<Item = &Path> {
        let relative_root = relative_root.as_ref().to_path_buf();
        self.entries
            .iter()
            .filter(move |entry| entry.starts_with(&relative_root))
            .map(PathBuf::as_path)
    }

    fn capture_directory(&mut self, relative_directory: &Path) {
        let absolute_directory = self.root.join(relative_directory);
        let entries = fs::read_dir(&absolute_directory).unwrap_or_else(|error| {
            panic!(
                "{} should be readable: {error}",
                absolute_directory.display()
            )
        });

        for entry in entries {
            let entry = entry.expect("workspace source entry should load");
            let name = entry.file_name();
            let root_build_target = relative_directory.as_os_str().is_empty() && name == "target";
            if name == ".git" || root_build_target {
                continue;
            }

            let relative_path = relative_directory.join(name);
            self.entries.insert(relative_path.clone());
            let file_type = entry.file_type().expect("workspace entry type should load");
            if file_type.is_dir() {
                self.capture_directory(&relative_path);
            } else if is_source_text(&relative_path) {
                let absolute_path = self.root.join(&relative_path);
                let text = fs::read_to_string(&absolute_path).unwrap_or_else(|error| {
                    panic!(
                        "{} should decode as source text: {error}",
                        absolute_path.display()
                    )
                });
                self.text_files.insert(
                    relative_path.clone(),
                    WorkspaceSourceFile {
                        absolute_path,
                        relative_path,
                        text: Arc::from(text),
                    },
                );
            }
        }
    }

    fn relative_path<'a>(&self, path: &'a Path) -> Option<&'a Path> {
        if path.is_absolute() {
            path.strip_prefix(&self.root).ok()
        } else {
            Some(path)
        }
    }
}

impl Deref for WorkspaceSourceInventory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.root()
    }
}

fn is_source_text(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
        || path.file_name().is_some_and(|name| name == "Cargo.toml")
}
