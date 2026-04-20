use super::counters::CompositionCounters;
use super::families::{ScopeFamily, TemplateFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryCompositionAdmissionFailureClass {
    UnsupportedScopeFamily,
    UnsupportedTemplateFamily,
    DeferredTemplateFamily,
    TemplateBindingMismatch,
    DuplicateTemplateBinding,
    MissingTemplateBinding,
    IllegalScopeWidening,
    LoweredAuthoredBoundaryRejected,
    BasisEvidenceQueryMismatch,
}

impl QueryCompositionAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedScopeFamily => "unsupported_scope_family",
            Self::UnsupportedTemplateFamily => "unsupported_template_family",
            Self::DeferredTemplateFamily => "deferred_template_family",
            Self::TemplateBindingMismatch => "template_binding_mismatch",
            Self::DuplicateTemplateBinding => "duplicate_template_binding",
            Self::MissingTemplateBinding => "missing_template_binding",
            Self::IllegalScopeWidening => "illegal_scope_widening",
            Self::LoweredAuthoredBoundaryRejected => "lowered_authored_boundary_rejected",
            Self::BasisEvidenceQueryMismatch => "basis_evidence_query_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCompositionError {
    failure_class: QueryCompositionAdmissionFailureClass,
    scope_family: Option<ScopeFamily>,
    template_family: Option<TemplateFamily>,
    counters: CompositionCounters,
    message: String,
}

impl QueryCompositionError {
    #[cfg(test)]
    pub(crate) fn unsupported_scope(
        family: ScopeFamily,
        counters: CompositionCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            failure_class: QueryCompositionAdmissionFailureClass::UnsupportedScopeFamily,
            scope_family: Some(family),
            template_family: None,
            counters,
            message: message.into(),
        }
    }

    pub(crate) fn unsupported_template(
        family: TemplateFamily,
        failure_class: QueryCompositionAdmissionFailureClass,
        counters: CompositionCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            failure_class,
            scope_family: None,
            template_family: Some(family),
            counters,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_scope(
        family: ScopeFamily,
        counters: CompositionCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            failure_class: QueryCompositionAdmissionFailureClass::IllegalScopeWidening,
            scope_family: Some(family),
            template_family: None,
            counters,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_template(
        family: TemplateFamily,
        failure_class: QueryCompositionAdmissionFailureClass,
        counters: CompositionCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            failure_class,
            scope_family: None,
            template_family: Some(family),
            counters,
            message: message.into(),
        }
    }

    pub(crate) fn lowered_authored_boundary_rejected(
        counters: CompositionCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            failure_class: QueryCompositionAdmissionFailureClass::LoweredAuthoredBoundaryRejected,
            scope_family: None,
            template_family: None,
            counters,
            message: message.into(),
        }
    }

    pub(crate) fn basis_query_mismatch(
        family: ScopeFamily,
        counters: CompositionCounters,
        expected: &str,
        actual: &str,
    ) -> Self {
        Self {
            failure_class: QueryCompositionAdmissionFailureClass::BasisEvidenceQueryMismatch,
            scope_family: Some(family),
            template_family: None,
            counters,
            message: format!(
                "basis evidence expected canonical query digest '{}' but composition lowered '{}'",
                expected, actual
            ),
        }
    }

    pub fn failure_class(&self) -> &QueryCompositionAdmissionFailureClass {
        &self.failure_class
    }

    pub fn scope_family(&self) -> Option<ScopeFamily> {
        self.scope_family
    }

    pub fn template_family(&self) -> Option<TemplateFamily> {
        self.template_family
    }

    pub fn counters(&self) -> &CompositionCounters {
        &self.counters
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
