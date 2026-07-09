use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::activation::SubscriptionActivationInput;
use super::super::active_lane::ActiveSubscriptionLaneAdmission;
use super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::continuation::SubscriptionContinuationReport;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::evidence_identities::{
    subscription_family_capability_identity, support_subject_identity,
};
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
    capability_identity: WorthQueryEvidenceIdentity,
}

impl SubscriptionFamilyCapabilityDigest {
    pub(crate) fn for_family(family: &QuerySubscriptionFamily) -> Self {
        Self {
            capability_identity: subscription_family_capability_identity(family),
        }
    }

    pub fn capability_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.capability_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportSubject {
    support_class: QuerySubscriptionSupportClass,
    family: QuerySubscriptionFamily,
    future_selection: QuerySubscriptionFutureSelection,
    declaration_identity: WorthQueryEvidenceIdentity,
    admission_identity: Option<WorthQueryEvidenceIdentity>,
    source_identity: WorthQueryEvidenceIdentity,
    subject_identity: WorthQueryEvidenceIdentity,
}

impl QuerySubscriptionSupportSubject {
    pub fn declaration(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::Declaration,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_identity(),
            None,
            declaration.declaration_identity(),
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
            declaration.declaration_identity(),
            Some(activation.admission_identity()),
            activation.evidence_identity(),
        )
    }

    pub fn active_lifecycle(
        declaration: &QuerySubscriptionDeclarationArtifact,
        admission: &QuerySubscriptionAdmissionArtifact,
        active_admission: &ActiveSubscriptionLaneAdmission,
    ) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::ActiveLifecycle,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_identity(),
            Some(admission.evidence_identity()),
            active_admission.lane_digest().evidence_identity(),
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
            declaration.declaration_identity(),
            Some(admission.evidence_identity()),
            continuation.evidence_identity(),
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
            declaration.declaration_identity(),
            Some(admission.evidence_identity()),
            closeout.evidence_identity(),
        )
    }

    pub fn durable_replay(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::DurableReplay,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_identity(),
            None,
            declaration.declaration_identity(),
        )
    }

    pub fn store_backed_restart(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self::new(
            QuerySubscriptionSupportClass::StoreBackedRestart,
            declaration.family().clone(),
            declaration.future_selection().clone(),
            declaration.declaration_identity(),
            None,
            declaration.declaration_identity(),
        )
    }

    fn new(
        support_class: QuerySubscriptionSupportClass,
        family: QuerySubscriptionFamily,
        future_selection: QuerySubscriptionFutureSelection,
        declaration_identity: &WorthQueryEvidenceIdentity,
        admission_identity: Option<&WorthQueryEvidenceIdentity>,
        source_identity: &WorthQueryEvidenceIdentity,
    ) -> Self {
        let subject_identity = support_subject_identity(
            support_class.as_str(),
            &family,
            future_selection.projection_identity(),
            declaration_identity,
            admission_identity,
            source_identity,
        );
        Self {
            support_class,
            family,
            future_selection,
            declaration_identity: declaration_identity.clone(),
            admission_identity: admission_identity.cloned(),
            source_identity: source_identity.clone(),
            subject_identity,
        }
    }

    pub fn support_class(&self) -> &QuerySubscriptionSupportClass {
        &self.support_class
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.declaration_identity
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn admission_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.admission_identity.as_ref()
    }

    pub fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn subject_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subject_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportEvidence {
    kind: QuerySubscriptionSupportEvidenceKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportEvidenceError {
    message: &'static str,
    pub(in crate::subscription::support) failure_identity: WorthQueryEvidenceIdentity,
}

impl QuerySubscriptionSupportEvidenceError {
    fn new(
        message: &'static str,
        declaration_identity: &WorthQueryEvidenceIdentity,
        admission_declaration_identity: &WorthQueryEvidenceIdentity,
    ) -> Self {
        let failure_identity = WorthQueryEvidenceIdentity::compose(
            crate::evidence_identity::WorthQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            crate::evidence_identity::WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_evidence_error_v1",
        )
        .field_shape(
            crate::evidence_identity::WorthQueryEvidenceTag::new("message"),
            message,
        )
        .field_evidence_identity(
            crate::evidence_identity::WorthQueryEvidenceTag::new("declaration"),
            declaration_identity,
        )
        .field_evidence_identity(
            crate::evidence_identity::WorthQueryEvidenceTag::new("admission_declaration"),
            admission_declaration_identity,
        )
        .seal();
        Self {
            message,
            failure_identity,
        }
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.failure_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionSupportEvidenceKind {
    Declaration {
        declaration_identity: WorthQueryEvidenceIdentity,
        family: QuerySubscriptionFamily,
        source_identity: WorthQueryEvidenceIdentity,
    },
    Admission {
        declaration_identity: WorthQueryEvidenceIdentity,
        family: QuerySubscriptionFamily,
        admission_identity: WorthQueryEvidenceIdentity,
        support_profile: super::profile::QuerySubscriptionSupportProfile,
        source_identity: WorthQueryEvidenceIdentity,
    },
}

impl QuerySubscriptionSupportEvidence {
    pub fn declaration(declaration: &QuerySubscriptionDeclarationArtifact) -> Self {
        Self {
            kind: QuerySubscriptionSupportEvidenceKind::Declaration {
                declaration_identity: declaration.declaration_identity().clone(),
                family: declaration.family().clone(),
                source_identity: declaration.declaration_identity().clone(),
            },
        }
    }

    pub fn admission(
        declaration: &QuerySubscriptionDeclarationArtifact,
        admission: &QuerySubscriptionAdmissionArtifact,
    ) -> Result<Self, QuerySubscriptionSupportEvidenceError> {
        if crate::subscription::evidence_identities::typed_identity_drift(
            declaration.declaration_identity(),
            admission.query_declaration_identity(),
        ) {
            return Err(QuerySubscriptionSupportEvidenceError::new(
                "subscription support evidence requires declaration and admission artifacts from the same canonical query subscription family",
                declaration.declaration_identity(),
                admission.query_declaration_identity(),
            ));
        }

        Ok(Self {
            kind: QuerySubscriptionSupportEvidenceKind::Admission {
                declaration_identity: declaration.declaration_identity().clone(),
                family: declaration.family().clone(),
                admission_identity: admission.evidence_identity().clone(),
                support_profile: admission.support_profile().clone(),
                source_identity: admission.evidence_identity().clone(),
            },
        })
    }

    pub(crate) fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration {
                declaration_identity,
                ..
            }
            | QuerySubscriptionSupportEvidenceKind::Admission {
                declaration_identity,
                ..
            } => declaration_identity,
        }
    }

    pub(crate) fn family(&self) -> &QuerySubscriptionFamily {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { family, .. }
            | QuerySubscriptionSupportEvidenceKind::Admission { family, .. } => family,
        }
    }

    pub(crate) fn admission_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration { .. } => None,
            QuerySubscriptionSupportEvidenceKind::Admission {
                admission_identity, ..
            } => Some(admission_identity),
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

    pub(crate) fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        match &self.kind {
            QuerySubscriptionSupportEvidenceKind::Declaration {
                source_identity, ..
            }
            | QuerySubscriptionSupportEvidenceKind::Admission {
                source_identity, ..
            } => source_identity,
        }
    }
}
