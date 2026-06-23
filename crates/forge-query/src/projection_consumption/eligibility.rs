use super::contracts::MaterializedProjectionContract;
use super::declaration::ProjectionConsumptionDeclaration;
use super::facts::{ProjectionFactKind, ProjectionFactRequest};
use super::identity::{
    compose_eligibility_admitted_digest, compose_eligibility_deferred_failure_digest,
    compose_eligibility_denied_failure_digest, compose_eligibility_source_mismatch_failure_digest,
    compose_eligibility_warning_kinds_digest,
};
use super::source::ProjectionSourceFamily;
use super::support::{support_for_kind, ProjectionConsumptionSupportPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionDenialReason {
    FactFamilyNotVisible { field_key: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeferredProjectionConsumptionReason {
    WriteReceiptContractBindingPending,
    SourceFamilySupportPending,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionWarningKind {
    QueryContextRowBound,
    PreviewDerivedContext,
}

impl ProjectionConsumptionWarningKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryContextRowBound => "query_context_row_bound",
            Self::PreviewDerivedContext => "preview_derived_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionWarnings {
    warning_kinds: Vec<ProjectionConsumptionWarningKind>,
    warning_digest: String,
}

impl ProjectionConsumptionWarnings {
    pub fn warning_kinds(&self) -> &[ProjectionConsumptionWarningKind] {
        &self.warning_kinds
    }

    pub fn warning_digest(&self) -> &str {
        &self.warning_digest
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectionConsumptionEligibilityCounters {
    requested_fact_count: usize,
    evaluated_fact_count: usize,
    warning_count: usize,
    denied_count: usize,
    deferred_count: usize,
    source_mismatch_count: usize,
}

impl ProjectionConsumptionEligibilityCounters {
    pub fn requested_fact_count(&self) -> usize {
        self.requested_fact_count
    }

    pub fn evaluated_fact_count(&self) -> usize {
        self.evaluated_fact_count
    }

    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    pub fn denied_count(&self) -> usize {
        self.denied_count
    }

    pub fn deferred_count(&self) -> usize {
        self.deferred_count
    }

    pub fn source_mismatch_count(&self) -> usize {
        self.source_mismatch_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionEligibilityTrace {
    rule_label: &'static str,
    explanation: &'static str,
}

impl ProjectionConsumptionEligibilityTrace {
    pub fn rule_label(&self) -> &'static str {
        self.rule_label
    }

    pub fn explanation(&self) -> &'static str {
        &self.explanation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedProjectionConsumption {
    declaration: ProjectionConsumptionDeclaration,
    declaration_digest: String,
    query_digest: String,
    basis_digest: String,
    result_shape_digest: String,
    authorized_projection_identity: String,
    warning_kinds: Vec<ProjectionConsumptionWarningKind>,
    counters: ProjectionConsumptionEligibilityCounters,
    trace: ProjectionConsumptionEligibilityTrace,
    eligibility_digest: String,
}

impl AdmittedProjectionConsumption {
    pub(crate) fn declaration(&self) -> &ProjectionConsumptionDeclaration {
        &self.declaration
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub(crate) fn warning_kinds(&self) -> &[ProjectionConsumptionWarningKind] {
        &self.warning_kinds
    }

    pub fn bind_contract(&self) -> MaterializedProjectionContract {
        super::contracts::bind_materialized_projection_contract(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedProjectionConsumption {
    declaration_digest: String,
    reason: ProjectionConsumptionDenialReason,
    counters: ProjectionConsumptionEligibilityCounters,
    trace: ProjectionConsumptionEligibilityTrace,
    failure_digest: String,
}

impl DeniedProjectionConsumption {
    pub fn reason(&self) -> &ProjectionConsumptionDenialReason {
        &self.reason
    }

    pub fn counters(&self) -> &ProjectionConsumptionEligibilityCounters {
        &self.counters
    }

    pub(crate) fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredProjectionConsumption {
    declaration_digest: String,
    source_family: ProjectionSourceFamily,
    reason: DeferredProjectionConsumptionReason,
    counters: ProjectionConsumptionEligibilityCounters,
    trace: ProjectionConsumptionEligibilityTrace,
    failure_digest: String,
}

impl DeferredProjectionConsumption {
    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn reason(&self) -> &DeferredProjectionConsumptionReason {
        &self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceMismatchedProjectionConsumption {
    declaration_digest: String,
    source_family: ProjectionSourceFamily,
    requested_fact_kind: ProjectionFactKind,
    counters: ProjectionConsumptionEligibilityCounters,
    trace: ProjectionConsumptionEligibilityTrace,
    failure_digest: String,
}

impl SourceMismatchedProjectionConsumption {
    pub fn requested_fact_kind(&self) -> ProjectionFactKind {
        self.requested_fact_kind
    }

    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub(crate) fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionEligibility {
    Admitted(AdmittedProjectionConsumption),
    AdmittedWithWarnings(AdmittedProjectionConsumption, ProjectionConsumptionWarnings),
    Denied(DeniedProjectionConsumption),
    Deferred(DeferredProjectionConsumption),
    SourceMismatch(SourceMismatchedProjectionConsumption),
}

pub fn evaluate_projection_consumption_eligibility(
    declaration: &ProjectionConsumptionDeclaration,
) -> ProjectionConsumptionEligibility {
    let mut counters = ProjectionConsumptionEligibilityCounters {
        requested_fact_count: declaration.requested().requested_count(),
        ..ProjectionConsumptionEligibilityCounters::default()
    };
    let mut warnings = Vec::new();

    for request in declaration.requested().requested() {
        counters.evaluated_fact_count += 1;
        match support_for_kind(declaration.source(), request.kind()) {
            ProjectionConsumptionSupportPosture::Admitted => {}
            ProjectionConsumptionSupportPosture::AdmittedWithWarnings(kind) => {
                counters.warning_count += 1;
                warnings.push(kind);
            }
            ProjectionConsumptionSupportPosture::Deferred(reason) => {
                counters.deferred_count = 1;
                return ProjectionConsumptionEligibility::Deferred(DeferredProjectionConsumption {
                    declaration_digest: declaration.declaration_digest().to_string(),
                    source_family: declaration.source().family(),
                    reason,
                    counters,
                    trace: ProjectionConsumptionEligibilityTrace {
                        rule_label: "fact_family_deferred_for_source",
                        explanation:
                            "the requested fact family remains deferred for this source family in the current milestone slice",
                    },
                    failure_digest: compose_eligibility_deferred_failure_digest(
                        declaration.declaration_digest(),
                    ),
                });
            }
            ProjectionConsumptionSupportPosture::SourceMismatch => {
                counters.source_mismatch_count = 1;
                return ProjectionConsumptionEligibility::SourceMismatch(
                    SourceMismatchedProjectionConsumption {
                        declaration_digest: declaration.declaration_digest().to_string(),
                        source_family: declaration.source().family(),
                        requested_fact_kind: request.kind(),
                        counters,
                        trace: ProjectionConsumptionEligibilityTrace {
                            rule_label: "source_family_does_not_prove_fact_family",
                            explanation:
                                "the named source family does not prove the requested fact family",
                        },
                        failure_digest: compose_eligibility_source_mismatch_failure_digest(
                            declaration.declaration_digest(),
                            declaration.source().family(),
                            request.kind(),
                        ),
                    },
                );
            }
        }

        if let Some(denial) = visibility_denial(declaration, request) {
            counters.denied_count = 1;
            return ProjectionConsumptionEligibility::Denied(DeniedProjectionConsumption {
                declaration_digest: declaration.declaration_digest().to_string(),
                reason: denial,
                counters,
                trace: ProjectionConsumptionEligibilityTrace {
                    rule_label: "authorized_projection_visibility_denial",
                    explanation:
                        "the requested field-backed fact is not visible in the bound authorized projection",
                },
                failure_digest: compose_eligibility_denied_failure_digest(
                    declaration.declaration_digest(),
                ),
            });
        }
    }

    let admitted = AdmittedProjectionConsumption {
        declaration: declaration.clone(),
        declaration_digest: declaration.declaration_digest().to_string(),
        query_digest: declaration.source().query_digest().unwrap_or("").to_string(),
        basis_digest: declaration.source().basis_digest().unwrap_or("").to_string(),
        result_shape_digest: declaration.binding().result_shape_digest().to_string(),
        authorized_projection_identity: declaration
            .binding()
            .authorized_projection_identity()
            .to_string(),
        warning_kinds: warnings.clone(),
        counters: counters.clone(),
        trace: ProjectionConsumptionEligibilityTrace {
            rule_label: "all_requested_fact_families_admitted",
            explanation:
                "every requested fact family remained within the supported and visible surface for the declared source family",
        },
        eligibility_digest: compose_eligibility_admitted_digest(
            declaration.declaration_digest(),
            warnings.len(),
        ),
    };
    if warnings.is_empty() {
        ProjectionConsumptionEligibility::Admitted(admitted)
    } else {
        let warning_digest = compose_eligibility_warning_kinds_digest(&warnings);
        ProjectionConsumptionEligibility::AdmittedWithWarnings(
            admitted,
            ProjectionConsumptionWarnings {
                warning_kinds: warnings,
                warning_digest,
            },
        )
    }
}

fn visibility_denial(
    declaration: &ProjectionConsumptionDeclaration,
    request: &ProjectionFactRequest,
) -> Option<ProjectionConsumptionDenialReason> {
    let field_key = request.field_key()?;
    let visible = declaration
        .binding()
        .authorized_visible_fields()
        .iter()
        .any(|candidate| candidate == field_key);
    if visible {
        None
    } else {
        Some(ProjectionConsumptionDenialReason::FactFamilyNotVisible {
            field_key: field_key.to_string(),
        })
    }
}
