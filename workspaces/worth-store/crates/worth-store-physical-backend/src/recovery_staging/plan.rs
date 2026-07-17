use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_proof::{CanonicalVec, NonEmpty};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonCurrentStagingArtifact {
    output_name: String,
    source_bytes: u64,
    source_digest: [u8; 32],
    output_bytes: u64,
    output_digest: [u8; 32],
    source: NonCurrentStagingArtifactSource,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NonCurrentStagingArtifactSource {
    SourceFile,
    Inline(Vec<u8>),
}

impl NonCurrentStagingArtifact {
    pub fn admit(
        output_name: impl Into<String>,
        source_bytes: u64,
        source_digest: [u8; 32],
    ) -> Option<Self> {
        let output_name = output_name.into();
        if output_name.is_empty()
            || output_name.contains(['/', '\\'])
            || output_name == "."
            || output_name == ".."
            || source_bytes == 0
            || source_digest == [0; 32]
        {
            None
        } else {
            Some(Self {
                output_name,
                source_bytes,
                source_digest,
                output_bytes: source_bytes,
                output_digest: source_digest,
                source: NonCurrentStagingArtifactSource::SourceFile,
            })
        }
    }

    pub fn admit_inline(output_name: impl Into<String>, bytes: Vec<u8>) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }
        let digest = Sha256::digest(&bytes).into();
        let length = bytes.len() as u64;
        let mut artifact = Self::admit(output_name, length, digest)?;
        artifact.source = NonCurrentStagingArtifactSource::Inline(bytes);
        Some(artifact)
    }

    pub fn admit_prefix(
        output_name: impl Into<String>,
        source_bytes: u64,
        source_digest: [u8; 32],
        output_bytes: u64,
        output_digest: [u8; 32],
    ) -> Option<Self> {
        let mut artifact = Self::admit(output_name, source_bytes, source_digest)?;
        if output_bytes == 0 || output_bytes > source_bytes || output_digest == [0; 32] {
            return None;
        }
        artifact.output_bytes = output_bytes;
        artifact.output_digest = output_digest;
        Some(artifact)
    }

    pub fn output_name(&self) -> &str {
        &self.output_name
    }
    pub const fn source_bytes(&self) -> u64 {
        self.source_bytes
    }
    pub const fn source_digest(&self) -> [u8; 32] {
        self.source_digest
    }
    pub const fn output_bytes(&self) -> u64 {
        self.output_bytes
    }
    pub const fn output_digest(&self) -> [u8; 32] {
        self.output_digest
    }
    pub(crate) fn inline_bytes(&self) -> Option<&[u8]> {
        match &self.source {
            NonCurrentStagingArtifactSource::SourceFile => None,
            NonCurrentStagingArtifactSource::Inline(bytes) => Some(bytes),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NonCurrentStagingPlanRequest {
    operation_identity: [u8; 32],
    source_root: PathBuf,
    target_parent: PathBuf,
    artifacts: Vec<NonCurrentStagingArtifact>,
    admitted_capacity_bytes: u64,
    copy_buffer_bytes: usize,
}

impl NonCurrentStagingPlanRequest {
    pub fn new(
        operation_identity: [u8; 32],
        source_root: impl Into<PathBuf>,
        target_parent: impl Into<PathBuf>,
        artifacts: Vec<NonCurrentStagingArtifact>,
        admitted_capacity_bytes: u64,
        copy_buffer_bytes: usize,
    ) -> Self {
        Self {
            operation_identity,
            source_root: source_root.into(),
            target_parent: target_parent.into(),
            artifacts,
            admitted_capacity_bytes,
            copy_buffer_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonCurrentStagingLoweringDenial {
    EmptyOperationIdentity,
    EmptyArtifactSet,
    DuplicateArtifact,
    SourceUnavailable,
    TargetUnavailable,
    SourceTargetAlias,
    InsufficientCapacity { required: u64, admitted: u64 },
    InvalidBuffer,
    AllocationFailed,
    SizeOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonCurrentStagingPlanBinding {
    fingerprint: [u8; 32],
    operation_identity: [u8; 32],
    source_root: PathBuf,
    staging_root: PathBuf,
    artifact_count: u64,
    expected_bytes: u64,
    copy_buffer_bytes: usize,
}

impl NonCurrentStagingPlanBinding {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    pub const fn operation_identity(&self) -> [u8; 32] {
        self.operation_identity
    }
    pub fn source_root(&self) -> &Path {
        &self.source_root
    }
    pub fn staging_root(&self) -> &Path {
        &self.staging_root
    }
    pub const fn artifact_count(&self) -> u64 {
        self.artifact_count
    }
    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
    pub const fn copy_buffer_bytes(&self) -> usize {
        self.copy_buffer_bytes
    }
}

#[derive(Debug, Clone)]
pub struct LoweredNonCurrentStagingPlan {
    binding: NonCurrentStagingPlanBinding,
    artifacts: CanonicalVec<NonCurrentStagingArtifact>,
    non_empty_artifacts: NonEmpty<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicalRecoveryStagingOwner;

impl PhysicalRecoveryStagingOwner {
    pub fn lower(
        request: NonCurrentStagingPlanRequest,
    ) -> Result<LoweredNonCurrentStagingPlan, NonCurrentStagingLoweringDenial> {
        Self::lower_for_execution(request)
    }

    pub(crate) fn lower_for_execution(
        mut request: NonCurrentStagingPlanRequest,
    ) -> Result<LoweredNonCurrentStagingPlan, NonCurrentStagingLoweringDenial> {
        if request.operation_identity == [0; 32] {
            return Err(NonCurrentStagingLoweringDenial::EmptyOperationIdentity);
        }
        if request.artifacts.is_empty() {
            return Err(NonCurrentStagingLoweringDenial::EmptyArtifactSet);
        }
        if request.copy_buffer_bytes == 0 {
            return Err(NonCurrentStagingLoweringDenial::InvalidBuffer);
        }
        let source_root = std::fs::canonicalize(&request.source_root)
            .map_err(|_| NonCurrentStagingLoweringDenial::SourceUnavailable)?;
        let target_parent = std::fs::canonicalize(&request.target_parent)
            .map_err(|_| NonCurrentStagingLoweringDenial::TargetUnavailable)?;
        if source_root.starts_with(&target_parent) || target_parent.starts_with(&source_root) {
            return Err(NonCurrentStagingLoweringDenial::SourceTargetAlias);
        }
        request.artifacts.sort();
        if request
            .artifacts
            .windows(2)
            .any(|rows| rows[0].output_name == rows[1].output_name)
        {
            return Err(NonCurrentStagingLoweringDenial::DuplicateArtifact);
        }
        let expected_bytes = request
            .artifacts
            .iter()
            .try_fold(0_u64, |total, artifact| {
                total.checked_add(artifact.output_bytes)
            })
            .ok_or(NonCurrentStagingLoweringDenial::SizeOverflow)?;
        if expected_bytes > request.admitted_capacity_bytes {
            return Err(NonCurrentStagingLoweringDenial::InsufficientCapacity {
                required: expected_bytes,
                admitted: request.admitted_capacity_bytes,
            });
        }
        let staging_root = target_parent.join(staging_directory_name(request.operation_identity));
        let fingerprint = plan_fingerprint(
            request.operation_identity,
            &source_root,
            &staging_root,
            &request.artifacts,
            request.copy_buffer_bytes,
        );
        let names = request
            .artifacts
            .iter()
            .map(|row| row.output_name.clone())
            .collect::<Vec<_>>();
        let non_empty_artifacts = NonEmpty::try_from_vec(names)
            .map_err(|_| NonCurrentStagingLoweringDenial::EmptyArtifactSet)?;
        let artifact_count = u64::try_from(request.artifacts.len())
            .map_err(|_| NonCurrentStagingLoweringDenial::SizeOverflow)?;
        Ok(LoweredNonCurrentStagingPlan {
            binding: NonCurrentStagingPlanBinding {
                fingerprint,
                operation_identity: request.operation_identity,
                source_root,
                staging_root,
                artifact_count,
                expected_bytes,
                copy_buffer_bytes: request.copy_buffer_bytes,
            },
            artifacts: CanonicalVec::try_from_sorted(request.artifacts)
                .map_err(|_| NonCurrentStagingLoweringDenial::AllocationFailed)?,
            non_empty_artifacts,
        })
    }
}

impl LoweredNonCurrentStagingPlan {
    pub const fn binding(&self) -> &NonCurrentStagingPlanBinding {
        &self.binding
    }
    pub(crate) fn artifacts(&self) -> &[NonCurrentStagingArtifact] {
        self.artifacts.as_slice()
    }
    pub(crate) fn artifact_names(&self) -> &[String] {
        self.non_empty_artifacts.as_slice()
    }
}

fn staging_directory_name(identity: [u8; 32]) -> String {
    let mut name = String::from(".worth-recovery-");
    for byte in &identity[..12] {
        name.push_str(&format!("{byte:02x}"));
    }
    name
}

fn plan_fingerprint(
    operation_identity: [u8; 32],
    source_root: &Path,
    staging_root: &Path,
    artifacts: &[NonCurrentStagingArtifact],
    copy_buffer_bytes: usize,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-non-current-staging-plan-v1");
    digest.update(operation_identity);
    update_path(&mut digest, source_root);
    update_path(&mut digest, staging_root);
    digest.update((copy_buffer_bytes as u64).to_be_bytes());
    for artifact in artifacts {
        digest.update((artifact.output_name.len() as u64).to_be_bytes());
        digest.update(artifact.output_name.as_bytes());
        digest.update(artifact.source_bytes.to_be_bytes());
        digest.update(artifact.source_digest);
        digest.update(artifact.output_bytes.to_be_bytes());
        digest.update(artifact.output_digest);
        digest.update([u8::from(artifact.inline_bytes().is_some())]);
    }
    digest.finalize().into()
}

fn update_path(digest: &mut Sha256, path: &Path) {
    let bytes = path.as_os_str().to_string_lossy();
    digest.update((bytes.len() as u64).to_be_bytes());
    digest.update(bytes.as_bytes());
}
