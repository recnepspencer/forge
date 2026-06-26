#[cfg(test)]
use crate::runtime::source_ingress::digest::fold_texts;
use crate::source::WorthUiArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiWatchedArtifactInput {
    label: String,
    digest: u64,
    artifact: Option<WorthUiArtifact>,
}

impl WorthUiWatchedArtifactInput {
    pub fn rust_authored(label: impl Into<String>, digest: u64) -> Self {
        Self {
            label: label.into(),
            digest,
            artifact: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_rust_authored_artifact(
        label: impl Into<String>,
        artifact: WorthUiArtifact,
    ) -> Self {
        let label = label.into();
        let digest = fold_texts([format!("rust-artifact:{label}")]);
        Self {
            label,
            digest,
            artifact: Some(artifact),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }

    pub(crate) fn artifact(&self) -> Option<&WorthUiArtifact> {
        self.artifact.as_ref()
    }
}
