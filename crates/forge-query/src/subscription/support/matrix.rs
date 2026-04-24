use crate::identity::hash_parts;

use super::super::family::QuerySubscriptionFamily;
use super::profile::{
    QuerySubscriptionActiveLifecycleSupport, QuerySubscriptionLifecycleCloseoutSupport,
    QuerySubscriptionRuntimeBackedSupport, QuerySubscriptionSupportProfile,
};
use super::subject::{
    QuerySubscriptionSupportClass, QuerySubscriptionSupportPosture,
    QuerySubscriptionSupportSubject, SubscriptionFamilyCapabilityDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportMatrixRow {
    support_class: QuerySubscriptionSupportClass,
    posture: QuerySubscriptionSupportPosture,
    row_digest: String,
}

impl QuerySubscriptionSupportMatrixRow {
    pub(crate) fn new(
        family: &QuerySubscriptionFamily,
        support_class: QuerySubscriptionSupportClass,
        posture: QuerySubscriptionSupportPosture,
    ) -> Self {
        let row_digest = hash_parts(&[
            "query_subscription_support_matrix_row_v1".to_string(),
            family.as_str().to_string(),
            support_class.as_str().to_string(),
            posture.as_str().to_string(),
        ]);
        Self {
            support_class,
            posture,
            row_digest,
        }
    }

    pub fn support_class(&self) -> &QuerySubscriptionSupportClass {
        &self.support_class
    }

    pub fn posture(&self) -> &QuerySubscriptionSupportPosture {
        &self.posture
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportMatrix {
    family: QuerySubscriptionFamily,
    capability_digest: SubscriptionFamilyCapabilityDigest,
    rows: Vec<QuerySubscriptionSupportMatrixRow>,
    digest: String,
}

impl QuerySubscriptionSupportMatrix {
    pub(crate) fn for_family(
        family: &QuerySubscriptionFamily,
        profile: Option<&QuerySubscriptionSupportProfile>,
        subject: &QuerySubscriptionSupportSubject,
    ) -> Self {
        let capability_digest = SubscriptionFamilyCapabilityDigest::for_family(family);
        let coverage = SupportCoverageStage::for_subject(subject);
        let rows = QuerySubscriptionSupportClass::all()
            .into_iter()
            .map(|support_class| {
                QuerySubscriptionSupportMatrixRow::new(
                    family,
                    support_class,
                    posture_for_class(support_class, coverage, profile),
                )
            })
            .collect::<Vec<_>>();
        let mut digest_parts = vec![
            "query_subscription_support_matrix_v1".to_string(),
            family.as_str().to_string(),
            format!("capability:{}", capability_digest.as_str()),
        ];
        digest_parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        let digest = hash_parts(&digest_parts);
        Self {
            family: family.clone(),
            capability_digest,
            rows,
            digest,
        }
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn capability_digest(&self) -> &SubscriptionFamilyCapabilityDigest {
        &self.capability_digest
    }

    pub fn rows(&self) -> &[QuerySubscriptionSupportMatrixRow] {
        &self.rows
    }

    pub fn row_for_class(
        &self,
        support_class: QuerySubscriptionSupportClass,
    ) -> Option<&QuerySubscriptionSupportMatrixRow> {
        self.rows
            .iter()
            .find(|row| row.support_class() == &support_class)
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

fn posture_for_class(
    support_class: QuerySubscriptionSupportClass,
    coverage: SupportCoverageStage,
    profile: Option<&QuerySubscriptionSupportProfile>,
) -> QuerySubscriptionSupportPosture {
    match support_class {
        QuerySubscriptionSupportClass::Declaration => {
            QuerySubscriptionSupportPosture::RuntimeBackedCertified
        }
        QuerySubscriptionSupportClass::Activation => stage_bounded_posture(
            coverage.allows(QuerySubscriptionSupportClass::Activation),
            profile
                .map(|profile| runtime_backed_posture(profile.runtime_backed_support()))
                .unwrap_or(QuerySubscriptionSupportPosture::UncertifiedDenied),
        ),
        QuerySubscriptionSupportClass::ActiveLifecycle
        | QuerySubscriptionSupportClass::Continuation => stage_bounded_posture(
            coverage.allows(support_class),
            profile
                .map(|profile| lifecycle_posture(profile.active_lifecycle_support()))
                .unwrap_or(QuerySubscriptionSupportPosture::UncertifiedDenied),
        ),
        QuerySubscriptionSupportClass::PreviewCloseout => stage_bounded_posture(
            coverage.allows(QuerySubscriptionSupportClass::PreviewCloseout),
            profile
                .map(|profile| closeout_posture(profile.lifecycle_closeout_support()))
                .unwrap_or(QuerySubscriptionSupportPosture::UncertifiedDenied),
        ),
        QuerySubscriptionSupportClass::DurableReplay
        | QuerySubscriptionSupportClass::StoreBackedRestart => {
            QuerySubscriptionSupportPosture::RuntimeBackedDeferred
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SupportCoverageStage {
    DeclarationOnly,
    Activation,
    ActiveLifecycle,
    Continuation,
    PreviewCloseout,
}

impl SupportCoverageStage {
    fn for_subject(subject: &QuerySubscriptionSupportSubject) -> Self {
        match subject.support_class() {
            QuerySubscriptionSupportClass::Declaration
            | QuerySubscriptionSupportClass::DurableReplay
            | QuerySubscriptionSupportClass::StoreBackedRestart => Self::DeclarationOnly,
            QuerySubscriptionSupportClass::Activation => Self::Activation,
            QuerySubscriptionSupportClass::ActiveLifecycle => Self::ActiveLifecycle,
            QuerySubscriptionSupportClass::Continuation => Self::Continuation,
            QuerySubscriptionSupportClass::PreviewCloseout => Self::PreviewCloseout,
        }
    }

    fn allows(&self, support_class: QuerySubscriptionSupportClass) -> bool {
        match self {
            Self::DeclarationOnly => false,
            Self::Activation => matches!(support_class, QuerySubscriptionSupportClass::Activation),
            Self::ActiveLifecycle => matches!(
                support_class,
                QuerySubscriptionSupportClass::Activation
                    | QuerySubscriptionSupportClass::ActiveLifecycle
            ),
            Self::Continuation => matches!(
                support_class,
                QuerySubscriptionSupportClass::Activation
                    | QuerySubscriptionSupportClass::ActiveLifecycle
                    | QuerySubscriptionSupportClass::Continuation
            ),
            Self::PreviewCloseout => matches!(
                support_class,
                QuerySubscriptionSupportClass::Activation
                    | QuerySubscriptionSupportClass::ActiveLifecycle
                    | QuerySubscriptionSupportClass::Continuation
                    | QuerySubscriptionSupportClass::PreviewCloseout
            ),
        }
    }
}

fn stage_bounded_posture(
    phase_proven: bool,
    posture: QuerySubscriptionSupportPosture,
) -> QuerySubscriptionSupportPosture {
    if phase_proven {
        posture
    } else {
        QuerySubscriptionSupportPosture::UncertifiedDenied
    }
}

fn runtime_backed_posture(
    support: &QuerySubscriptionRuntimeBackedSupport,
) -> QuerySubscriptionSupportPosture {
    match support {
        QuerySubscriptionRuntimeBackedSupport::Admitted => {
            QuerySubscriptionSupportPosture::RuntimeBackedCertified
        }
        QuerySubscriptionRuntimeBackedSupport::Denied => {
            QuerySubscriptionSupportPosture::RuntimeBackedDenied
        }
    }
}

fn lifecycle_posture(
    support: &QuerySubscriptionActiveLifecycleSupport,
) -> QuerySubscriptionSupportPosture {
    match support {
        QuerySubscriptionActiveLifecycleSupport::Admitted => {
            QuerySubscriptionSupportPosture::RuntimeBackedCertified
        }
        QuerySubscriptionActiveLifecycleSupport::Denied => {
            QuerySubscriptionSupportPosture::RuntimeBackedDenied
        }
    }
}

fn closeout_posture(
    support: &QuerySubscriptionLifecycleCloseoutSupport,
) -> QuerySubscriptionSupportPosture {
    match support {
        QuerySubscriptionLifecycleCloseoutSupport::Admitted => {
            QuerySubscriptionSupportPosture::RuntimeBackedCertified
        }
        QuerySubscriptionLifecycleCloseoutSupport::Denied => {
            QuerySubscriptionSupportPosture::RuntimeBackedDenied
        }
    }
}
