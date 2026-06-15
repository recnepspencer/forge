use crate::capability::{CapabilitySnapshotDigest, SnapshotMetrics};
use crate::facade::WorthUiApp;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactAssemblyDiagnosticCode, WorthUiArtifactDependencyDeriver,
    WorthUiArtifactDependencyMetrics, WorthUiArtifactDigest, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactEquivalenceComparator, WorthUiArtifactHandle, WorthUiArtifactInput,
    WorthUiArtifactInputResolver, WorthUiArtifactInspection, WorthUiArtifactInspectionBasisBuilder,
    WorthUiArtifactInspectionDeriver, WorthUiBindingDiagnosticCode, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiIdentitySeedingDiagnosticCode, WorthUiIncrementalInvalidationBasis,
    WorthUiParseDiagnosticCode, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiResolutionDiagnosticCode, WorthUiRustCompositionInput, WorthUiRustCompositionMetrics,
    WorthUiRustCompositionToArtifactInputLowerer, WorthUiSourceModuleId, WorthUiSourcePackage,
    WorthUiSourcePackageDigest, WorthUiSourceParser, WorthUiStructuralLegalityDiagnosticCode,
    WorthUiStructuralLegalityLowerer,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;

#[derive(Clone, Debug)]
pub(super) struct WorthUiSampleCertificate {
    authoring_evidence: WorthUiSampleAuthoringEvidence,
    snapshot_digest: CapabilitySnapshotDigest,
    snapshot_metrics: SnapshotMetrics,
    artifact: WorthUiArtifact,
    inspection: WorthUiArtifactInspection,
    dependency_basis: WorthUiIncrementalInvalidationBasis,
    dependency_metrics: WorthUiArtifactDependencyMetrics,
    semantic_digest: WorthUiArtifactDigest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorthUiSampleAuthoringEvidence {
    FileSourcePackage {
        package_digest: WorthUiSourcePackageDigest,
        module_count: usize,
    },
    RustComposition {
        metrics: WorthUiRustCompositionMetrics,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorthUiSampleCertificationFailure {
    Parse(Vec<WorthUiParseDiagnosticCode>),
    Resolution(Vec<WorthUiResolutionDiagnosticCode>),
    StructuralLegality(Vec<WorthUiStructuralLegalityDiagnosticCode>),
    BindingSemantics(Vec<WorthUiBindingDiagnosticCode>),
    IdentitySeeding(Vec<WorthUiIdentitySeedingDiagnosticCode>),
    ArtifactAssembly(Vec<WorthUiArtifactAssemblyDiagnosticCode>),
    ArtifactInspection,
}

pub(super) fn certify_file_source_package(
    source_package: WorthUiSourcePackage,
) -> Result<WorthUiSampleCertificate, WorthUiSampleCertificationFailure> {
    let authoring_evidence = WorthUiSampleAuthoringEvidence::FileSourcePackage {
        package_digest: source_package.digest(),
        module_count: source_package.module_ids().len(),
    };
    let parsed_package = WorthUiSourceParser::parse_package(&source_package)
        .map_err(|report| WorthUiSampleCertificationFailure::Parse(parse_codes(&report)))?;
    certify_artifact_input(
        WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_package)
            .expect("authoring entry should lower to artifact input"),
        authoring_evidence,
    )
}

pub(super) fn certify_rust_composition(
    composition: WorthUiRustCompositionInput,
) -> Result<WorthUiSampleCertificate, WorthUiSampleCertificationFailure> {
    let report = WorthUiRustCompositionToArtifactInputLowerer::lower_with_report(&composition);
    let metrics = report.metrics();
    certify_artifact_input(
        report.into_artifact_input(),
        WorthUiSampleAuthoringEvidence::RustComposition { metrics },
    )
}

pub(super) fn semantic_equivalence(
    left: &WorthUiSampleCertificate,
    right: &WorthUiSampleCertificate,
) -> WorthUiArtifactEquivalence {
    WorthUiArtifactEquivalenceComparator::compare(
        left.artifact(),
        right.artifact(),
        WorthUiArtifactEquivalenceBasis::semantic(),
    )
}

pub(super) fn source_module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(std::path::Path::new(path))
        .expect("sample module id should be valid")
}

impl WorthUiSampleCertificate {
    pub(super) fn authoring_evidence(&self) -> WorthUiSampleAuthoringEvidence {
        self.authoring_evidence
    }

    pub(super) fn snapshot_digest(&self) -> CapabilitySnapshotDigest {
        self.snapshot_digest
    }

    pub(super) fn snapshot_metrics(&self) -> SnapshotMetrics {
        self.snapshot_metrics
    }

    pub(super) fn artifact(&self) -> &WorthUiArtifact {
        &self.artifact
    }

    pub(super) fn semantic_digest(&self) -> WorthUiArtifactDigest {
        self.semantic_digest
    }

    pub(super) fn inspection(&self) -> &WorthUiArtifactInspection {
        &self.inspection
    }

    pub(super) fn dependency_basis(&self) -> &WorthUiIncrementalInvalidationBasis {
        &self.dependency_basis
    }

    pub(super) fn dependency_metrics(&self) -> WorthUiArtifactDependencyMetrics {
        self.dependency_metrics
    }

    pub(super) fn handles(&self) -> Vec<WorthUiArtifactHandle> {
        self.artifact
            .module_ids()
            .iter()
            .filter_map(|module_id| self.artifact.module(module_id))
            .flat_map(|module| module.nodes().iter().map(|node| node.handle().clone()))
            .collect()
    }
}

fn certify_artifact_input(
    artifact_input: WorthUiArtifactInput,
    authoring_evidence: WorthUiSampleAuthoringEvidence,
) -> Result<WorthUiSampleCertificate, WorthUiSampleCertificationFailure> {
    let app = identity_test_app();
    certify_artifact_input_for_app(artifact_input, app, authoring_evidence)
}

fn certify_artifact_input_for_app(
    artifact_input: WorthUiArtifactInput,
    app: WorthUiApp,
    authoring_evidence: WorthUiSampleAuthoringEvidence,
) -> Result<WorthUiSampleCertificate, WorthUiSampleCertificationFailure> {
    let snapshot = app.capabilities();
    let snapshot_digest = snapshot.digest();
    let snapshot_metrics = snapshot.metrics();
    let resolved =
        WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot).map_err(|report| {
            WorthUiSampleCertificationFailure::Resolution(resolution_codes(&report))
        })?;
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).map_err(|report| {
            WorthUiSampleCertificationFailure::StructuralLegality(structural_codes(&report))
        })?;
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).map_err(|report| {
        WorthUiSampleCertificationFailure::BindingSemantics(binding_codes(&report))
    })?;
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .map_err(|report| {
            WorthUiSampleCertificationFailure::IdentitySeeding(identity_codes(&report))
        })?
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble_with_metrics(&identity_seeded)
        .map_err(|report| {
            WorthUiSampleCertificationFailure::ArtifactAssembly(assembly_codes(&report))
        })?
        .0;
    let inspection_basis =
        WorthUiArtifactInspectionBasisBuilder::build(&artifact, &identity_seeded)
            .map_err(|_| WorthUiSampleCertificationFailure::ArtifactInspection)?;
    let inspection = WorthUiArtifactInspectionDeriver::derive(&artifact, &inspection_basis)
        .map_err(|_| WorthUiSampleCertificationFailure::ArtifactInspection)?;
    let dependency_report = WorthUiArtifactDependencyDeriver::derive_with_report(&artifact);
    let semantic_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());

    Ok(WorthUiSampleCertificate {
        authoring_evidence,
        snapshot_digest,
        snapshot_metrics,
        artifact,
        inspection,
        dependency_basis: dependency_report.basis().clone(),
        dependency_metrics: dependency_report.metrics(),
        semantic_digest,
    })
}

fn parse_codes(report: &crate::source::WorthUiParseReport) -> Vec<WorthUiParseDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

pub(super) fn resolution_codes(
    report: &crate::source::WorthUiResolutionReport,
) -> Vec<WorthUiResolutionDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

fn structural_codes(
    report: &crate::source::WorthUiStructuralLegalityReport,
) -> Vec<WorthUiStructuralLegalityDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

pub(super) fn binding_codes(
    report: &crate::source::WorthUiBindingSemanticsReport,
) -> Vec<WorthUiBindingDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

fn identity_codes(
    report: &crate::source::WorthUiIdentitySeedingReport,
) -> Vec<WorthUiIdentitySeedingDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

fn assembly_codes(
    report: &crate::source::WorthUiArtifactAssemblyReport,
) -> Vec<WorthUiArtifactAssemblyDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}
