use forge_store_physical_format::{
    ManifestDiscoveryReport, MinimalManifestVerifierReport, PhysicalReference,
    PhysicalReferenceAuthority,
};

use crate::{RuntimeVerifierParityTrace, RuntimeVerifierRelationship};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLayoutObserver {
    discovered_references: Vec<PhysicalReference>,
}

impl RuntimeLayoutObserver {
    pub fn from_manifest_report(report: ManifestDiscoveryReport<'_>) -> Self {
        Self {
            discovered_references: references_from_manifest_report(report),
        }
    }

    pub fn discovered_references(&self) -> &[PhysicalReference] {
        &self.discovered_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineVerifierObserver {
    discovered_references: Vec<PhysicalReference>,
}

impl OfflineVerifierObserver {
    pub fn from_report(report: &MinimalManifestVerifierReport) -> Self {
        Self {
            discovered_references: report.layout().discovered_references().to_vec(),
        }
    }

    pub fn discovered_references(&self) -> &[PhysicalReference] {
        &self.discovered_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLayoutParityReport {
    compared_references: u32,
    parity_trace: RuntimeVerifierParityTrace,
}

impl PhysicalLayoutParityReport {
    pub const fn matches(&self) -> bool {
        matches!(
            self.parity_trace.relationship(),
            RuntimeVerifierRelationship::RuntimeMustMatchVerifier
        )
    }

    pub const fn compared_references(&self) -> u32 {
        self.compared_references
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLayoutParity;

impl PhysicalLayoutParity {
    pub fn compare(
        runtime: RuntimeLayoutObserver,
        offline: OfflineVerifierObserver,
    ) -> Result<PhysicalLayoutParityReport, PhysicalLayoutParityDenial> {
        if runtime.discovered_references() != offline.discovered_references() {
            return Err(PhysicalLayoutParityDenial::new(
                runtime.discovered_references().len() as u32,
                offline.discovered_references().len() as u32,
            ));
        }
        Ok(PhysicalLayoutParityReport {
            compared_references: offline.discovered_references().len() as u32,
            parity_trace: RuntimeVerifierParityTrace::new(
                RuntimeVerifierRelationship::RuntimeMustMatchVerifier,
            ),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalLayoutParityDenial {
    runtime_reference_count: u32,
    offline_reference_count: u32,
    parity_trace: RuntimeVerifierParityTrace,
}

impl PhysicalLayoutParityDenial {
    const fn new(runtime_reference_count: u32, offline_reference_count: u32) -> Self {
        Self {
            runtime_reference_count,
            offline_reference_count,
            parity_trace: RuntimeVerifierParityTrace::new(
                RuntimeVerifierRelationship::RuntimeMustDisagreeWithVerifier,
            ),
        }
    }

    pub const fn runtime_reference_count(self) -> u32 {
        self.runtime_reference_count
    }

    pub const fn offline_reference_count(self) -> u32 {
        self.offline_reference_count
    }

    pub const fn parity_trace(self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }
}

fn references_from_manifest_report(report: ManifestDiscoveryReport<'_>) -> Vec<PhysicalReference> {
    let references = PhysicalReferenceAuthority::s1();
    let root = report.root();
    let mut discovered = Vec::new();
    discovered.push(
        references
            .admit_root_publication(root.root_publication())
            .reference(),
    );
    for page_slot in root.page_slots() {
        discovered.push(
            references
                .admit_page_slot(page_slot.page_slot())
                .reference(),
        );
    }
    for extent in root.extents() {
        discovered.push(references.admit_extent(extent.extent()).reference());
    }
    for free_space in root.free_space() {
        discovered.push(
            references
                .admit_free_space_reuse(free_space.reuse_cell())
                .reference(),
        );
    }
    discovered
}
