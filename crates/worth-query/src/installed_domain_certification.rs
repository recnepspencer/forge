use std::collections::BTreeSet;
use std::path::Path;

use crate::consumer_kit::{
    audit_workspace_domain_authority_inventory, worth_query_consumer_residue_registry,
    worth_query_domain_authority_inventory_rows, WorthQueryConsumerResidueClass,
};
use crate::domain_capabilities::certify_domain_capabilities;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMilestoneNineThirteenInstalledDomainCertificationErrorKind {
    AuthorityInventoryReadFailed,
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
    consumer_residue_manifest_digest: String,
    domain_capability_certification_digest: String,
    certification_digest: String,
    authority_finding_count: usize,
    missing_consumer_residue_class_count: usize,
}

impl WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle {
    pub fn is_closed(&self) -> bool {
        self.authority_finding_count == 0 && self.missing_consumer_residue_class_count == 0
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn authority_inventory_digest(&self) -> &str {
        &self.authority_inventory_digest
    }

    pub fn consumer_residue_manifest_digest(&self) -> &str {
        &self.consumer_residue_manifest_digest
    }

    pub fn domain_capability_certification_digest(&self) -> &str {
        &self.domain_capability_certification_digest
    }

    pub fn authority_finding_count(&self) -> usize {
        self.authority_finding_count
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
    let authority_finding_count = authority_audit.findings().len();
    let certification_digest = hash_parts(&[
        authority_inventory_digest.clone(),
        consumer_residue_manifest_digest.clone(),
        domain_capability_certification_digest.clone(),
        format!("authority_findings:{authority_finding_count}"),
        format!("missing_residue:{missing_consumer_residue_class_count}"),
    ]);

    Ok(
        WorthQueryMilestoneNineThirteenInstalledDomainCertificationBundle {
            authority_inventory_digest,
            consumer_residue_manifest_digest,
            domain_capability_certification_digest,
            certification_digest,
            authority_finding_count,
            missing_consumer_residue_class_count,
        },
    )
}
