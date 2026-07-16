use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBackupSource {
    source_path: PathBuf,
    output_name: String,
    expected_bytes: u64,
    expected_digest: [u8; 32],
    expected_physical_identity: [u8; 32],
}

impl PhysicalBackupSource {
    pub fn new(
        source_path: impl Into<PathBuf>,
        output_name: impl Into<String>,
        expected_bytes: u64,
        expected_digest: [u8; 32],
        expected_physical_identity: [u8; 32],
    ) -> Option<Self> {
        let source_path = source_path.into();
        let output_name = output_name.into();
        if source_path.as_os_str().is_empty()
            || !portable_output_name(&output_name)
            || expected_bytes == 0
        {
            None
        } else {
            Some(Self {
                source_path,
                output_name,
                expected_bytes,
                expected_digest,
                expected_physical_identity,
            })
        }
    }
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
    pub fn output_name(&self) -> &str {
        &self.output_name
    }
    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
    pub const fn expected_digest(&self) -> [u8; 32] {
        self.expected_digest
    }
    pub const fn expected_physical_identity(&self) -> [u8; 32] {
        self.expected_physical_identity
    }
}

fn portable_output_name(name: &str) -> bool {
    if name.is_empty()
        || name.len() > 255
        || matches!(name, "." | "..")
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return false;
    }
    let stem = name.split('.').next().unwrap_or_default();
    let reserved_device = matches!(
        stem.to_ascii_uppercase().as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    );
    !reserved_device
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_portable_single_components() {
        for invalid in [
            "",
            ".",
            "..",
            "nested/name",
            "nested\\name",
            "CON",
            "nul.bin",
            "trailing ",
            "unicode-λ",
        ] {
            assert!(PhysicalBackupSource::new("source", invalid, 1, [0; 32], [0; 32]).is_none());
        }
        assert!(
            PhysicalBackupSource::new("source", "00000001-page.media", 1, [0; 32], [0; 32])
                .is_some()
        );
    }
}
