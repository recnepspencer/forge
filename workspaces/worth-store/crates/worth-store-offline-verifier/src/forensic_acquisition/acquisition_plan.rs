use std::path::PathBuf;

use sha2::{Digest, Sha256};
use worth_store_physical_backend::ReadOnlyOfflineMediaCapability;

use super::ForensicAcquisitionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicAcquisitionIntent {
    target_root: PathBuf,
    observer_identity: String,
    acquisition_method: String,
    clock_provenance: String,
    started_at_tick: u64,
    resident_buffer_bytes: usize,
}

impl ForensicAcquisitionIntent {
    pub fn new(
        target_root: impl Into<PathBuf>,
        observer_identity: impl Into<String>,
        acquisition_method: impl Into<String>,
        clock_provenance: impl Into<String>,
        started_at_tick: u64,
        resident_buffer_bytes: usize,
    ) -> Result<Self, ForensicAcquisitionDenial> {
        let observer_identity = observer_identity.into();
        let acquisition_method = acquisition_method.into();
        let clock_provenance = clock_provenance.into();
        if observer_identity.is_empty()
            || acquisition_method.is_empty()
            || clock_provenance.is_empty()
        {
            return Err(ForensicAcquisitionDenial::InvalidCustody);
        }
        if resident_buffer_bytes == 0 {
            return Err(ForensicAcquisitionDenial::InvalidBufferBudget);
        }
        Ok(Self {
            target_root: target_root.into(),
            observer_identity,
            acquisition_method,
            clock_provenance,
            started_at_tick,
            resident_buffer_bytes,
        })
    }

    pub fn plan(
        self,
        media: &ReadOnlyOfflineMediaCapability,
    ) -> Result<ForensicAcquisitionPlan, ForensicAcquisitionDenial> {
        let sources = (0..media.file_count())
            .map(|index| {
                let file = media.file(index).expect("bounded source index");
                ForensicSourceBinding {
                    metadata_fingerprint: file.metadata_fingerprint(),
                    byte_length: file.length(),
                }
            })
            .collect::<Vec<_>>();
        let consistency_basis_identity: [u8; 32] =
            Sha256::digest(media.basis().identity().as_bytes()).into();
        let plan_identity = plan_identity(&self, consistency_basis_identity, sources.as_slice());
        Ok(ForensicAcquisitionPlan {
            intent: self,
            consistency_basis_identity,
            sources,
            plan_identity,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ForensicSourceBinding {
    pub(crate) metadata_fingerprint: [u8; 32],
    pub(crate) byte_length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicAcquisitionPlan {
    pub(crate) intent: ForensicAcquisitionIntent,
    pub(crate) consistency_basis_identity: [u8; 32],
    pub(crate) sources: Vec<ForensicSourceBinding>,
    pub(crate) plan_identity: [u8; 32],
}

impl ForensicAcquisitionPlan {
    pub const fn plan_identity(&self) -> [u8; 32] {
        self.plan_identity
    }

    pub fn target_root(&self) -> &std::path::Path {
        &self.intent.target_root
    }

    pub const fn resident_buffer_bytes(&self) -> usize {
        self.intent.resident_buffer_bytes
    }

    pub(crate) fn observer_identity(&self) -> &str {
        &self.intent.observer_identity
    }

    pub(crate) fn acquisition_method(&self) -> &str {
        &self.intent.acquisition_method
    }

    pub(crate) fn clock_provenance(&self) -> &str {
        &self.intent.clock_provenance
    }

    pub(crate) const fn started_at_tick(&self) -> u64 {
        self.intent.started_at_tick
    }

    pub(crate) fn validate_media(
        &self,
        media: &ReadOnlyOfflineMediaCapability,
    ) -> Result<(), ForensicAcquisitionDenial> {
        let observed_consistency: [u8; 32] =
            Sha256::digest(media.basis().identity().as_bytes()).into();
        if self.sources.len() != media.file_count()
            || self.consistency_basis_identity != observed_consistency
        {
            return Err(ForensicAcquisitionDenial::SourceBindingChanged);
        }
        for (index, expected) in self.sources.iter().enumerate() {
            let observed = media.file(index).expect("bounded source index");
            if observed.metadata_fingerprint() != expected.metadata_fingerprint
                || observed.length() != expected.byte_length
            {
                return Err(ForensicAcquisitionDenial::SourceBindingChanged);
            }
        }
        Ok(())
    }
}

fn plan_identity(
    intent: &ForensicAcquisitionIntent,
    consistency: [u8; 32],
    sources: &[ForensicSourceBinding],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-forensic-acquisition-plan-v1");
    digest.update(intent.observer_identity.as_bytes());
    digest.update(intent.acquisition_method.as_bytes());
    digest.update(intent.clock_provenance.as_bytes());
    digest.update(intent.started_at_tick.to_be_bytes());
    digest.update((intent.resident_buffer_bytes as u64).to_be_bytes());
    digest.update(consistency);
    for source in sources {
        digest.update(source.metadata_fingerprint);
        digest.update(source.byte_length.to_be_bytes());
    }
    digest.finalize().into()
}
