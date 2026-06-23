use std::path::PathBuf;
use std::{fs, io};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationManualReloadEdit {
    SourceFile {
        source_path: PathBuf,
        source_text: String,
    },
    ThemeFile {
        source_path: PathBuf,
        source_text: String,
    },
    CommandFile {
        source_path: PathBuf,
        source_text: String,
    },
    CommandProjectionFile {
        source_path: PathBuf,
        source_text: String,
    },
    ComponentFile {
        source_path: PathBuf,
        source_text: String,
    },
    AppearanceFile {
        source_path: PathBuf,
        source_text: String,
    },
    DensityFile {
        source_path: PathBuf,
        source_text: String,
    },
    AppearanceAndDensityFiles {
        appearance_path: PathBuf,
        appearance_text: String,
        density_path: PathBuf,
        density_text: String,
    },
}

impl ValidationManualReloadEdit {
    pub fn source_file(source_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self::SourceFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn theme_file(source_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self::ThemeFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn command_file(source_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self::CommandFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn command_projection_file(
        source_path: impl Into<PathBuf>,
        source_text: impl Into<String>,
    ) -> Self {
        Self::CommandProjectionFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn component_file(source_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self::ComponentFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn appearance_file(
        source_path: impl Into<PathBuf>,
        source_text: impl Into<String>,
    ) -> Self {
        Self::AppearanceFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn density_file(source_path: impl Into<PathBuf>, source_text: impl Into<String>) -> Self {
        Self::DensityFile {
            source_path: source_path.into(),
            source_text: source_text.into(),
        }
    }

    pub fn appearance_and_density_files(
        appearance_path: impl Into<PathBuf>,
        appearance_text: impl Into<String>,
        density_path: impl Into<PathBuf>,
        density_text: impl Into<String>,
    ) -> Self {
        Self::AppearanceAndDensityFiles {
            appearance_path: appearance_path.into(),
            appearance_text: appearance_text.into(),
            density_path: density_path.into(),
            density_text: density_text.into(),
        }
    }

    pub fn write_to_disk(&self) -> io::Result<()> {
        match self {
            Self::SourceFile {
                source_path,
                source_text,
            }
            | Self::ThemeFile {
                source_path,
                source_text,
            }
            | Self::CommandFile {
                source_path,
                source_text,
            }
            | Self::CommandProjectionFile {
                source_path,
                source_text,
            }
            | Self::ComponentFile {
                source_path,
                source_text,
            }
            | Self::AppearanceFile {
                source_path,
                source_text,
            }
            | Self::DensityFile {
                source_path,
                source_text,
            } => fs::write(source_path, source_text),
            Self::AppearanceAndDensityFiles {
                appearance_path,
                appearance_text,
                density_path,
                density_text,
            } => {
                fs::write(appearance_path, appearance_text)?;
                fs::write(density_path, density_text)
            }
        }
    }
}
