use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::cargo_invocation::BuiltTarget;
use super::compiler_artifacts::CompilerTranscript;

pub struct BoundArtifact<R> {
    path: PathBuf,
    digest: [u8; 32],
    transcript: CompilerTranscript,
    _role: std::marker::PhantomData<fn() -> R>,
}

impl<R> BoundArtifact<R> {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub fn raw_cargo_stdout(&self) -> &str {
        self.transcript.raw_stdout()
    }

    pub fn compiler_artifacts(&self) -> &[super::CompilerArtifactRecord] {
        self.transcript.records()
    }

    pub fn verify_unchanged(&self) -> Result<(), String> {
        if digest(&self.path)? != self.digest {
            return Err(format!(
                "source-bound executable was replaced after the build: {}",
                self.path.display()
            ));
        }
        Ok(())
    }

    pub(crate) fn rebind_promoted(&self, path: PathBuf) -> Result<Self, String> {
        let digest = digest(&path)?;
        if digest != self.digest {
            return Err(format!(
                "promoted executable digest changed for {}",
                path.display()
            ));
        }
        Ok(Self {
            path,
            digest,
            transcript: self.transcript.clone(),
            _role: std::marker::PhantomData,
        })
    }
}

pub(crate) fn bind<R>(target: BuiltTarget<R>) -> Result<BoundArtifact<R>, String> {
    let digest = digest(&target.path)?;
    Ok(BoundArtifact {
        path: target.path,
        digest,
        transcript: target.transcript,
        _role: std::marker::PhantomData,
    })
}

#[cfg(test)]
pub(crate) fn test_bound<R>(path: PathBuf) -> BoundArtifact<R> {
    let digest = digest(&path).expect("test artifact must be readable");
    BoundArtifact {
        path,
        digest,
        transcript: CompilerTranscript::test_empty(),
        _role: std::marker::PhantomData,
    }
}

fn digest(path: &Path) -> Result<[u8; 32], String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("read source-bound executable {}: {error}", path.display()))?;
    Ok(Sha256::digest(bytes).into())
}
