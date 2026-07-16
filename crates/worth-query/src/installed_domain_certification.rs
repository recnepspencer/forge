mod evidence_manifest;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::consumer_kit::{
    audit_workspace_domain_authority_inventory, query_consumer_residue_audit,
    worth_query_consumer_residue_registry, worth_query_domain_authority_inventory_rows,
    worth_query_reference_consumer_adoption_rows, worth_query_reference_consumer_deleted_residue,
    WorthQueryConsumerResidueClass,
};
use crate::domain_capabilities::{
    certify_domain_capabilities, worth_query_domain_capability_compile_fail_boundaries,
};
use crate::identity::hash_parts;

pub use evidence_manifest::{
    worth_query_milestone_nine_thirteen_installed_domain_evidence_rows,
    WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind {
    AuthorityInventoryReadFailed,
    ConsumerAuditFailed,
    ConsumerResidueFound,
    EvidenceSourceReadFailed,
    EvidenceProbeDrift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenInstalledDomainCertificationError {
    kind: WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind,
    path: Option<String>,
    message: String,
}

impl WorthQueryMilestoneNineThirteenInstalledDomainCertificationError {
    fn new(
        kind: WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind,
        path: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            path,
            message: message.into(),
        }
    }

    pub const fn kind(
        &self,
    ) -> WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind {
        self.kind
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for WorthQueryMilestoneNineThirteenInstalledDomainCertificationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for WorthQueryMilestoneNineThirteenInstalledDomainCertificationError {}

const REQUIRED_INSTALLED_BOUNDARIES: &[&str] = &[
    "admitted_package_type_private",
    "application_facade_executable_authority_not_public",
    "certification_digests_cannot_be_caller_authored",
    "closed_installed_live_handle_cannot_be_revived",
    "contribution_admission_not_public",
    "contribution_evaluation_not_public",
    "contribution_preparation_not_public",
    "domain_capability_vocabulary_not_in_runtime_facade",
    "installation_generation_constructor_private",
    "installation_receipt_constructor_private",
    "installed_contribution_target_cannot_be_restamped",
    "installed_handle_fields_private",
    "inspection_cannot_be_promoted_to_rebind_authority",
    "installed_operation_declaration_cannot_be_restamped",
    "low_level_materializer_not_public",
    "manual_operation_registry_not_public",
    "operating_context_digest_authoring_forbidden",
    "package_admission_not_consumer_callable",
    "package_execution_callback_forbidden",
    "package_identity_constructor_private",
    "raw_domain_constructor_not_public",
    "raw_operation_owner_constructor_private",
];

const REQUIRED_INSTALLED_RESIDUE: &[WorthQueryConsumerResidueClass] = &[
    WorthQueryConsumerResidueClass::RawDomainStringAuthority,
    WorthQueryConsumerResidueClass::ConsumerAuthoredContextDigest,
    WorthQueryConsumerResidueClass::ApplicationFacadeDomainAuthority,
    WorthQueryConsumerResidueClass::IndependentOperationRegistry,
    WorthQueryConsumerResidueClass::CallerSuppliedOperationRegistry,
    WorthQueryConsumerResidueClass::QueryPhaseMaterializerImport,
    WorthQueryConsumerResidueClass::ConsumerSemanticDomainAdapter,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle {
    authority_inventory_digest: String,
    compile_fail_manifest_digest: String,
    consumer_residue_manifest_digest: String,
    domain_capability_certification_digest: String,
    reference_consumer_journey_digest: String,
    consumer_source_inventory_digest: String,
    certification_digest: String,
    authority_finding_count: usize,
    missing_compile_fail_boundary_count: usize,
    missing_consumer_residue_class_count: usize,
    reference_consumer_count: usize,
    deleted_consumer_residue_count: usize,
}

impl WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle {
    pub fn is_closed(&self) -> bool {
        self.authority_finding_count == 0
            && self.missing_compile_fail_boundary_count == 0
            && self.missing_consumer_residue_class_count == 0
            && self.reference_consumer_count >= 2
            && self.deleted_consumer_residue_count > 0
            && !self.reference_consumer_journey_digest.is_empty()
            && !self.consumer_source_inventory_digest.is_empty()
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn authority_inventory_digest(&self) -> &str {
        &self.authority_inventory_digest
    }

    pub fn compile_fail_manifest_digest(&self) -> &str {
        &self.compile_fail_manifest_digest
    }

    pub fn consumer_residue_manifest_digest(&self) -> &str {
        &self.consumer_residue_manifest_digest
    }

    pub fn domain_capability_certification_digest(&self) -> &str {
        &self.domain_capability_certification_digest
    }

    pub fn reference_consumer_journey_digest(&self) -> &str {
        &self.reference_consumer_journey_digest
    }

    pub fn consumer_source_inventory_digest(&self) -> &str {
        &self.consumer_source_inventory_digest
    }

    pub fn authority_finding_count(&self) -> usize {
        self.authority_finding_count
    }

    pub fn missing_compile_fail_boundary_count(&self) -> usize {
        self.missing_compile_fail_boundary_count
    }

    pub fn missing_consumer_residue_class_count(&self) -> usize {
        self.missing_consumer_residue_class_count
    }
}

pub fn certify_milestone_nine_thirteen_installed_domain(
    repository_root: impl AsRef<Path>,
) -> Result<
    WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle,
    WorthQueryMilestoneNineThirteenInstalledDomainCertificationError,
> {
    let repository_root = repository_root.as_ref();
    let consumer_report = query_consumer_residue_audit("milestone-9.13-installed-domain-consumers")
        .required_root(repository_root.join("crates/hadwiger-research/src"))
        .required_root(
            repository_root.join("workspaces/worth-ui/crates/worth-ui-query-binding/src"),
        )
        .required_root(repository_root.join("workspaces/worth-ui/crates/worth-ui-runtime/src"))
        .evaluate()
        .map_err(|error| {
            WorthQueryMilestoneNineThirteenInstalledDomainCertificationError::new(
                WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind::ConsumerAuditFailed,
                error.source_label().map(str::to_string),
                format!("installed-domain consumer audit failed: {error}"),
            )
        })?;
    if consumer_report.finding_count() != 0 {
        return Err(
            WorthQueryMilestoneNineThirteenInstalledDomainCertificationError::new(
                WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind::ConsumerResidueFound,
                None,
                format!(
                    "installed-domain consumer audit found {} competing-authority sites",
                    consumer_report.finding_count()
                ),
            ),
        );
    }
    let reference_consumer_journey_digest = source_backed_evidence_digest(repository_root)?;
    let consumer_source_inventory_digest = consumer_report.source_inventory_digest().to_string();

    let authority_audit = audit_workspace_domain_authority_inventory(repository_root).map_err(
        |error| {
            WorthQueryMilestoneNineThirteenInstalledDomainCertificationError::new(
                WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind::AuthorityInventoryReadFailed,
                Some(
                    repository_root
                        .join("crates/worth-query/src")
                        .display()
                        .to_string(),
                ),
                format!("installed-domain authority inventory read failed: {error}"),
            )
        },
    )?;
    let authority_inventory_digest = hash_parts(
        &worth_query_domain_authority_inventory_rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.symbol(),
                    row.defining_path(),
                    row.current_class().as_str(),
                    row.final_owner()
                )
            })
            .collect::<Vec<_>>(),
    );

    let boundaries = worth_query_domain_capability_compile_fail_boundaries();
    let boundary_labels = boundaries
        .iter()
        .map(|row| row.label())
        .collect::<BTreeSet<_>>();
    let missing_compile_fail_boundary_count = REQUIRED_INSTALLED_BOUNDARIES
        .iter()
        .filter(|label| !boundary_labels.contains(**label))
        .count();
    let compile_fail_manifest_digest = hash_parts(
        &boundaries
            .iter()
            .map(|row| format!("{}:{}", row.label(), row.path()))
            .collect::<Vec<_>>(),
    );

    let residue_rows = worth_query_consumer_residue_registry();
    let residue_classes = residue_rows
        .iter()
        .map(|row| row.class())
        .collect::<BTreeSet<_>>();
    let missing_consumer_residue_class_count = REQUIRED_INSTALLED_RESIDUE
        .iter()
        .filter(|class| !residue_classes.contains(class))
        .count();
    let consumer_residue_manifest_digest = hash_parts(
        &residue_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.class().as_str(),
                    row.detection().as_str(),
                    row.replacement_lane()
                )
            })
            .collect::<Vec<_>>(),
    );
    let domain_capability_certification_digest = certify_domain_capabilities()
        .certification_bundle_digest()
        .to_string();
    let reference_consumer_count = worth_query_reference_consumer_adoption_rows().len();
    let deleted_consumer_residue_count = worth_query_reference_consumer_deleted_residue().len();
    let authority_finding_count = authority_audit.findings().len();
    let certification_digest = hash_parts(&[
        authority_inventory_digest.clone(),
        compile_fail_manifest_digest.clone(),
        consumer_residue_manifest_digest.clone(),
        domain_capability_certification_digest.clone(),
        reference_consumer_journey_digest.clone(),
        consumer_source_inventory_digest.clone(),
        format!("authority_findings:{authority_finding_count}"),
        format!("missing_boundaries:{missing_compile_fail_boundary_count}"),
        format!("missing_residue:{missing_consumer_residue_class_count}"),
    ]);

    Ok(
        WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle {
            authority_inventory_digest,
            compile_fail_manifest_digest,
            consumer_residue_manifest_digest,
            domain_capability_certification_digest,
            reference_consumer_journey_digest,
            consumer_source_inventory_digest,
            certification_digest,
            authority_finding_count,
            missing_compile_fail_boundary_count,
            missing_consumer_residue_class_count,
            reference_consumer_count,
            deleted_consumer_residue_count,
        },
    )
}

fn source_backed_evidence_digest(
    repository_root: &Path,
) -> Result<String, WorthQueryMilestoneNineThirteenInstalledDomainCertificationError> {
    let mut parts = Vec::new();
    for row in worth_query_milestone_nine_thirteen_installed_domain_evidence_rows() {
        let path = repository_root.join(row.path());
        let source = std::fs::read_to_string(&path).map_err(|error| {
            evidence_error(
                WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind::EvidenceSourceReadFailed,
                &path,
                format!("failed to read installed-domain evidence source: {error}"),
            )
        })?;
        let probe_count = source.match_indices(row.probe()).count();
        if probe_count != 1 {
            return Err(evidence_error(
                WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind::EvidenceProbeDrift,
                &path,
                format!(
                    "phase {} evidence probe must occur exactly once, found {probe_count}: {}",
                    row.phase(),
                    row.probe()
                ),
            ));
        }
        parts.push(format!(
            "{}:{}:{}:{}",
            row.phase(),
            row.path(),
            row.probe(),
            source
        ));
    }
    Ok(hash_parts(&parts))
}

fn evidence_error(
    kind: WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind,
    path: &PathBuf,
    message: impl Into<String>,
) -> WorthQueryMilestoneNineThirteenInstalledDomainCertificationError {
    WorthQueryMilestoneNineThirteenInstalledDomainCertificationError::new(
        kind,
        Some(path.display().to_string()),
        message,
    )
}
