use crate::identity::hash_parts;

use super::super::activation::SubscriptionActivationInput;
use super::super::active_lane::ActiveSubscriptionLaneAdmission;
use super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::continuation::SubscriptionContinuationReport;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::family::QuerySubscriptionFamily;
use super::super::future_selection::QuerySubscriptionFutureSelection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionSupportClass {
    Declaration,
    Activation,
    ActiveLifecycle,
    Continuation,
    PreviewCloseout,
    DurableReplay,
    StoreBackedRestart,
}

impl QuerySubscriptionSupportClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::Activation => "activation",
            Self::ActiveLifecycle => "active_lifecycle",
            Self::Continuation => "continuation",
            Self::PreviewCloseout => "preview_closeout",
            Self::DurableReplay => "durable_replay",
            Self::StoreBackedRestart => "store_backed_restart",
        }
    }

    pub(crate) fn all() -> [Self; 7] {
        [
            Self::Declaration,
            Self::Activation,
            Self::ActiveLifecycle,
            Self::Continuation,
            Self::PreviewCloseout,
            Self::DurableReplay,
            Self::StoreBackedRestart,
        ]
    }

    pub(crate) fn requires_admission_evidence(&self) -> bool {
        matches!(
            self,
            Self::Activation | Self::ActiveLifecycle | Self::Continuation | Self::PreviewCloseout
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionSupportPosture {
    RuntimeBackedCertified,
    RuntimeBackedDenied,
    RuntimeBackedDeferred,
    UncertifiedDenied,
}

impl QuerySubscriptionSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeBackedCertified => "runtime_backed_certified",
            Self::RuntimeBackedDenied => "runtime_backed_denied",
            Self::RuntimeBackedDeferred => "runtime_backed_deferred",
            Self::UncertifiedDenied => "uncertified_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionFamilyCapabilityDigest {
    digest: String,
}

impl SubscriptionFamilyCapabilityDigest {
    pub(crate) fn for_family(family: &QuerySubscriptionFamily) -> Self {
        Self {
            digest: hash_parts(&[
                "query_subscription_family_capability_digest_v1".to_string(),
                family.as_str().to_string(),
            ]),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportSubject {
    support_class: QuerySubscriptionSupportClass,
    family: QuerySubscriptionFamily,
    future_selection: QuerySubscriptionFutureSelection,
    declaration_digest: String,
    admission_digest: Option<String>,
    source_digest: String,
    digest: String,
}

impl QuerySubscriptionSupportSubject {
    pub fn declaration(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::Declaration,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            None,
            declaration.declaration_digest().as_str(),
        )
    }

    pub fn activation(
        declaration: &QuerySubscriptionDeclarationArtifact,
        activation: &SubscriptionActivationInput,
    ) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::Activation,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            Some(activation.admission_digest()),
            activation.activation_digest(),
        )
    }

    pub fn active_lifecycle(
        declaration: &QuerySubscriptionDeclarationArtifact,
        active_admission: &ActiveSubscriptionLaneAdmission,
    ) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::ActiveLifecycle,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            Some(active_admission.admission_digest()),
            active_admission.lane_digest().as_str(),
        )
    }

    pub fn continuation(
        declaration: &QuerySubscriptionDeclarationArtifact,
        admission: &QuerySubscriptionAdmissionArtifact,
        continuation: &SubscriptionContinuationReport,
    ) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::Continuation,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            Some(admission.admission_digest()),
            continuation.report_digest(),
        )
    }

    pub fn preview_closeout(
        declaration: &QuerySubscriptionDeclarationArtifact,
        admission: &QuerySubscriptionAdmissionArtifact,
        closeout: &SubscriptionLifecycleCloseout,
    ) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::PreviewCloseout,
            declaration.family().clone(),
            closeout.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            Some(admission.admission_digest()),
            closeout.closeout_digest(),
        )
    }

    pub fn durable_replay(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::DurableReplay,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            None,
            declaration.declaration_digest().as_str(),
        )
    }

    pub fn store_backed_restart(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::StoreBackedRestart,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_digest().as_str(),
            None,
            declaration.declaration_digest().as_str(),
        )
    }

    fn new(
        support_class: QuerySubscriptionSupportClass,
        family: QuerySubscriptionFamily,
        future_selection: QuerySubscriptionFutureSelection,
        declaration_digest: &str,
        admission_digest: Option<&str>,
        source_digest: &str,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_support_subject_v1".to_string(),
            support_class.as_str().to_string(),
            family.as_str().to_string(),
            format!("future_selection:{}", future_selection.projection_digest()),
            format!("declaration:{declaration_digest}"),
            format!("admission:{}", admission_digest.unwrap_or("none")),
            format!("source:{source_digest}"),
        ]);
        Self {
            support_class,
            family,
            future_selection,
            declaration_digest: declaration_digest.to_string(),
            admission_digest: admission_digest.map(ToOwned::to_owned),
            source_digest: source_digest.to_string(),
            digest,
        }
    }

    pub fn support_class(&self) -> &QuerySubscriptionSupportClass {
        &self.support_class
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn admission_digest(&self) -> Option<&str> {
        self.admission_digest.as_deref()
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportEvidence {
    kind: QuerySubscriptionSupportEvidenceKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportEvidenceError {
    message: &'static str,
    failure_digest: String,
}

impl QuerySubscriptionSupportEvidenceError {
    fn new(
        message: &'static str,
        declaration_digest: &str,
        admission_declaration_digest: &str,
    ) -> Self {
        Self {
            message,
            failure_digest: hash_parts(&[
                "query_subscription_support_evidence_error_v1".to_string(),
                message.to_string(),
                format!("declaration:{declaration_digest}"),
                format!("admission_declaration:{admission_declaration_digest}"),
            ]),
        }
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionSupportEvidenceKind {
    Declaration {
        declaration_digest: String,
        family: QuerySubscriptionFamily,
        source_digest: String,
    },
    Admission {
        declaration_digest: String,
        family: QuerySubscriptionFamily,
        admission_digest: String,
        support_profile: super::profile::QuerySubscriptionSupportProfile,
        source_digest: String,
    },
}

impl QuerySubscriptionSupportEvidence {
    pub fn declaration(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self {
            kind: QuerySubscriptionSupportEvidenceKind::Declaration {
                declaration_digest: declaration.declaration_digest().as_str().to_string(),
                family: declaration.family().clone(),
                source_digest: declaration.declaration_digest().as_str().to_string(),
            },
        }
    }

    pub fn admission(
        declaration: &QuerySubscriptionDeclarationArtifact,
        admission: &QuerySubscriptionAdmissionArtifact,
    ) -> Result<Self, QuerySubscriptionSupportEvidenceError> {
        if declaration.declaration_digest().as_str() != admission.query_declaration_digest() {
            return Err(QuerySubscriptionSupportEvidenceError::new(
                "subscription support evidence requires declaration and admission artifacts from the same canonical query subscription family",
                declaration.declaration_digest().as_str(),
                admission.query_declaration_digest(),
            ));
        }

        Ok(Self {
            kind: QuerySubscriptionSupportEvidenceKind::Admission {
                declaration_digest: declaration.declaration_digest().as_str().to_string(),
                family: declaration.family().clone(),
                admission_digest: admission.admission_digest().to_string(),
                support_profile: admission.support_profile().clone(),
                source_digest: admission.admission_digest().to_string(),
            },
        })
    }

    pub(crate) fn declaration_digest(&self) -> &str {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration {
                declaration_digest, ..
            }
            | QuerySubscriptionSupportEvidenceKind::Admission {
                declaration_digest, ..
            } => declaration_digest,
        }
    }

    pub(crate) fn family(&self) -> &QuerySubscriptionFamily {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { family, .. }
            | QuerySubscriptionSupportEvidenceKind::Admission { family, .. } => family,
        }
    }

    pub(crate) fn admission_digest(&self) -> Option<&str> {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { .. } => None,
            QuerySubscriptionSupportEvidenceKind::Admission {
                admission_digest, ..
            } => Some(admission_digest),
        }
    }

    pub(crate) fn support_profile(
        &self,
    ) -> Option<&super::profile::QuerySubscriptionSupportProfile> {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { .. } => None,
            QuerySubscriptionSupportEvidenceKind::Admission {
                support_profile, ..
            } => Some(support_profile),
        }
    }

    pub(crate) fn source_digest(&self) -> &str {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { source_digest, .. }
            | QuerySubscriptionSupportEvidenceKind::Admission { source_digest, .. } => {
                source_digest
            }
        }
    }
}
