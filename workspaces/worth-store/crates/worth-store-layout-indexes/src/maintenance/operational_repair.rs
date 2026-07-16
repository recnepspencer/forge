use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexRepairRequest {
    operation_identity: [u8; 32],
    target: PathBuf,
    expected_target_digest: [u8; 32],
    replacement: PathBuf,
    replacement_digest: [u8; 32],
    expected_generation: u64,
    replacement_generation: u64,
    maximum_bytes: u64,
}

impl DerivedIndexRepairRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_identity: [u8; 32],
        target: impl Into<PathBuf>,
        expected_target_digest: [u8; 32],
        replacement: impl Into<PathBuf>,
        replacement_digest: [u8; 32],
        expected_generation: u64,
        replacement_generation: u64,
        maximum_bytes: u64,
    ) -> Self {
        Self {
            operation_identity,
            target: target.into(),
            expected_target_digest,
            replacement: replacement.into(),
            replacement_digest,
            expected_generation,
            replacement_generation,
            maximum_bytes,
        }
    }
    pub fn target(&self) -> &Path {
        &self.target
    }
    pub const fn expected_target_digest(&self) -> [u8; 32] {
        self.expected_target_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexRepairExecutionDenial {
    InvalidIdentity,
    InvalidGenerationAdvance,
    SourceTargetAlias,
    SourceMismatch,
    StaleTarget,
    BudgetExceeded,
    PersistedEffectMismatch,
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexRepairPlan {
    fingerprint: [u8; 32],
    request: DerivedIndexRepairRequest,
}

impl DerivedIndexRepairPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    pub fn target(&self) -> &Path {
        &self.request.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedIndexRepairReceipt {
    plan_fingerprint: [u8; 32],
    published_generation: u64,
    content_digest: [u8; 32],
}

impl DerivedIndexRepairReceipt {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn published_generation(self) -> u64 {
        self.published_generation
    }
    pub const fn content_digest(self) -> [u8; 32] {
        self.content_digest
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutOperationalRepairOwner;

impl LayoutOperationalRepairOwner {
    pub fn lower(
        request: DerivedIndexRepairRequest,
    ) -> Result<DerivedIndexRepairPlan, DerivedIndexRepairExecutionDenial> {
        validate_request(&request)?;
        validate_file(
            &request.replacement,
            request.replacement_digest,
            request.maximum_bytes,
            DerivedIndexRepairExecutionDenial::SourceMismatch,
        )?;
        validate_file(
            &request.target,
            request.expected_target_digest,
            request.maximum_bytes,
            DerivedIndexRepairExecutionDenial::StaleTarget,
        )?;
        let fingerprint = request_fingerprint(&request);
        Ok(DerivedIndexRepairPlan {
            fingerprint,
            request,
        })
    }

    pub fn execute(
        plan: DerivedIndexRepairPlan,
    ) -> Result<DerivedIndexRepairReceipt, DerivedIndexRepairExecutionDenial> {
        validate_file(
            &plan.request.replacement,
            plan.request.replacement_digest,
            plan.request.maximum_bytes,
            DerivedIndexRepairExecutionDenial::SourceMismatch,
        )?;
        let (_, target_digest) = digest_file(&plan.request.target, plan.request.maximum_bytes)?;
        if target_digest == plan.request.replacement_digest {
            return Ok(repair_receipt(&plan));
        }
        if target_digest != plan.request.expected_target_digest {
            return Err(DerivedIndexRepairExecutionDenial::StaleTarget);
        }
        let pending = pending_path(&plan.request.target, plan.fingerprint);
        copy_candidate(
            &plan.request.replacement,
            &pending,
            plan.request.replacement_digest,
            plan.request.maximum_bytes,
        )?;
        std::fs::rename(&pending, &plan.request.target)
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
        sync_parent(&plan.request.target)?;
        Ok(repair_receipt(&plan))
    }

    /// Reconstructs an owner receipt only after re-reading the durable target.
    ///
    /// This is the restart lane: orchestration may not trust a copied journal
    /// fingerprint as proof that the owner-local atomic replacement survived.
    pub fn recover_applied(
        plan: &DerivedIndexRepairPlan,
    ) -> Result<DerivedIndexRepairReceipt, DerivedIndexRepairExecutionDenial> {
        validate_file(
            &plan.request.target,
            plan.request.replacement_digest,
            plan.request.maximum_bytes,
            DerivedIndexRepairExecutionDenial::PersistedEffectMismatch,
        )?;
        Ok(repair_receipt(plan))
    }
}

const fn repair_receipt(plan: &DerivedIndexRepairPlan) -> DerivedIndexRepairReceipt {
    DerivedIndexRepairReceipt {
        plan_fingerprint: plan.fingerprint,
        published_generation: plan.request.replacement_generation,
        content_digest: plan.request.replacement_digest,
    }
}

fn validate_request(
    request: &DerivedIndexRepairRequest,
) -> Result<(), DerivedIndexRepairExecutionDenial> {
    if request.operation_identity == [0; 32] || request.maximum_bytes == 0 {
        return Err(DerivedIndexRepairExecutionDenial::InvalidIdentity);
    }
    if request.replacement_generation <= request.expected_generation {
        return Err(DerivedIndexRepairExecutionDenial::InvalidGenerationAdvance);
    }
    let target = std::fs::canonicalize(&request.target)
        .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    let replacement = std::fs::canonicalize(&request.replacement)
        .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    if target == replacement {
        return Err(DerivedIndexRepairExecutionDenial::SourceTargetAlias);
    }
    Ok(())
}

fn validate_file(
    path: &Path,
    expected: [u8; 32],
    maximum: u64,
    mismatch: DerivedIndexRepairExecutionDenial,
) -> Result<(), DerivedIndexRepairExecutionDenial> {
    let (bytes, digest) = digest_file(path, maximum)?;
    if bytes == 0 || digest != expected {
        Err(mismatch)
    } else {
        Ok(())
    }
}

fn digest_file(
    path: &Path,
    maximum: u64,
) -> Result<(u64, [u8; 32]), DerivedIndexRepairExecutionDenial> {
    let mut file = File::open(path).map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    let length = file
        .metadata()
        .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?
        .len();
    if length > maximum {
        return Err(DerivedIndexRepairExecutionDenial::BudgetExceeded);
    }
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok((length, digest.finalize().into()))
}

fn copy_candidate(
    source: &Path,
    target: &Path,
    expected: [u8; 32],
    maximum: u64,
) -> Result<(), DerivedIndexRepairExecutionDenial> {
    if target.exists() {
        return validate_file(
            target,
            expected,
            maximum,
            DerivedIndexRepairExecutionDenial::SourceMismatch,
        );
    }
    let mut input = File::open(source).map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    let mut output = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(target)
        .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = input
            .read(&mut buffer)
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
        if read == 0 {
            break;
        }
        output
            .write_all(&buffer[..read])
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)?;
    }
    output
        .sync_all()
        .map_err(|_| DerivedIndexRepairExecutionDenial::Io)
}

fn request_fingerprint(request: &DerivedIndexRepairRequest) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-derived-index-repair-plan-v1");
    digest.update(request.operation_identity);
    digest.update(request.expected_target_digest);
    digest.update(request.replacement_digest);
    digest.update(request.expected_generation.to_be_bytes());
    digest.update(request.replacement_generation.to_be_bytes());
    digest.finalize().into()
}
fn pending_path(target: &Path, fingerprint: [u8; 32]) -> PathBuf {
    target.with_extension(format!(
        "repair-{:02x}{:02x}.pending",
        fingerprint[0], fingerprint[1]
    ))
}
fn sync_parent(path: &Path) -> Result<(), DerivedIndexRepairExecutionDenial> {
    let parent = path.parent().ok_or(DerivedIndexRepairExecutionDenial::Io)?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(0x0200_0000)
            .open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)
    }
    #[cfg(not(windows))]
    {
        File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|_| DerivedIndexRepairExecutionDenial::Io)
    }
}
