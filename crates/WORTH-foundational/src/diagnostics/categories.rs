#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticArtifactKind {
    Summary,
    Report,
    FailureBundle,
    ComparisonBundle,
    SupportReport,
    ExplanationBundle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticDeliveryClass {
    MustBeHot,
    CanDefer,
    ReconstructableFromReplay,
    UnavailableByPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticAvailability {
    RetainedHot,
    DeferredCold,
    Reconstructable,
    Redacted,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticArtifactKindDefinition {
    kind: FoundationalDiagnosticArtifactKind,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalDiagnosticArtifactKindDefinition {
    pub(crate) const fn new(
        kind: FoundationalDiagnosticArtifactKind,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            kind,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn kind(&self) -> FoundationalDiagnosticArtifactKind {
        self.kind
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

const SUMMARY: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::Summary,
        "summary",
        "compact diagnostic overview with bounded evidence",
        "a row-rich report, a failure/comparison bundle, or certified support coverage",
    );
const REPORT: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::Report,
        "report",
        "row-bearing diagnostic report with descriptive explanation content",
        "a compact summary, a replay-heavy explanation bundle, or certified coverage attestation",
    );
const FAILURE_BUNDLE: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::FailureBundle,
        "failure_bundle",
        "typed failure-oriented diagnostic bundle",
        "a success summary, a generic report, or support certification proof",
    );
const COMPARISON_BUNDLE: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::ComparisonBundle,
        "comparison_bundle",
        "typed comparison and mismatch-oriented diagnostic bundle",
        "a generic report, a failure bundle, or support certification proof",
    );
const SUPPORT_REPORT: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::SupportReport,
        "support_report",
        "typed support posture report over a diagnostic subject",
        "a certified coverage proof, a generic report, or authoritative receipt evidence",
    );
const EXPLANATION_BUNDLE: FoundationalDiagnosticArtifactKindDefinition =
    FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::ExplanationBundle,
        "explanation_bundle",
        "typed explanation bundle with descriptive diagnostic evidence",
        "a support certification proof, a generic event bag, or authoritative transition meaning",
    );

pub const fn diagnostic_summary_definition() -> &'static FoundationalDiagnosticArtifactKindDefinition
{
    &SUMMARY
}

pub const fn diagnostic_report_definition() -> &'static FoundationalDiagnosticArtifactKindDefinition
{
    &REPORT
}

pub const fn diagnostic_failure_bundle_definition(
) -> &'static FoundationalDiagnosticArtifactKindDefinition {
    &FAILURE_BUNDLE
}

pub const fn diagnostic_comparison_bundle_definition(
) -> &'static FoundationalDiagnosticArtifactKindDefinition {
    &COMPARISON_BUNDLE
}

pub const fn diagnostic_support_report_definition(
) -> &'static FoundationalDiagnosticArtifactKindDefinition {
    &SUPPORT_REPORT
}

pub const fn diagnostic_explanation_bundle_definition(
) -> &'static FoundationalDiagnosticArtifactKindDefinition {
    &EXPLANATION_BUNDLE
}

pub const fn diagnostic_artifact_kind_definitions(
) -> [FoundationalDiagnosticArtifactKindDefinition; 6] {
    [
        SUMMARY,
        REPORT,
        FAILURE_BUNDLE,
        COMPARISON_BUNDLE,
        SUPPORT_REPORT,
        EXPLANATION_BUNDLE,
    ]
}

pub trait FoundationalDiagnosticArtifactKindMarker: sealed::Sealed {
    const KIND: FoundationalDiagnosticArtifactKind;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSummaryArtifactKind(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalReportArtifactKind(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalFailureBundleArtifactKind(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalComparisonBundleArtifactKind(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSupportReportArtifactKind(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalExplanationBundleArtifactKind(());

impl FoundationalDiagnosticArtifactKindMarker for FoundationalSummaryArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind = FoundationalDiagnosticArtifactKind::Summary;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_summary_definition()
    }
}

impl FoundationalDiagnosticArtifactKindMarker for FoundationalReportArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind = FoundationalDiagnosticArtifactKind::Report;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_report_definition()
    }
}

impl FoundationalDiagnosticArtifactKindMarker for FoundationalFailureBundleArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind =
        FoundationalDiagnosticArtifactKind::FailureBundle;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_failure_bundle_definition()
    }
}

impl FoundationalDiagnosticArtifactKindMarker for FoundationalComparisonBundleArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind =
        FoundationalDiagnosticArtifactKind::ComparisonBundle;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_comparison_bundle_definition()
    }
}

impl FoundationalDiagnosticArtifactKindMarker for FoundationalSupportReportArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind =
        FoundationalDiagnosticArtifactKind::SupportReport;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_support_report_definition()
    }
}

impl FoundationalDiagnosticArtifactKindMarker for FoundationalExplanationBundleArtifactKind {
    const KIND: FoundationalDiagnosticArtifactKind =
        FoundationalDiagnosticArtifactKind::ExplanationBundle;

    fn definition() -> &'static FoundationalDiagnosticArtifactKindDefinition {
        diagnostic_explanation_bundle_definition()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticMaterializationLegalityDenial {
    MustBeHotRequiresRetainedHotAvailability,
    CanDeferRequiresRetainedOrDeferredAvailability,
    ReconstructableDeliveryRequiresReconstructableAvailability,
    UnavailableByPolicyRequiresRedactedOrUnavailableAvailability,
    SummaryDoesNotSupportReplayReconstruction,
    ReportDoesNotSupportReplayReconstruction,
}

pub fn evaluate_diagnostic_materialization_legality(
    kind: FoundationalDiagnosticArtifactKind,
    delivery: FoundationalDiagnosticDeliveryClass,
    availability: FoundationalDiagnosticAvailability,
) -> Result<(), FoundationalDiagnosticMaterializationLegalityDenial> {
    match delivery {
        FoundationalDiagnosticDeliveryClass::MustBeHot => {
            if availability != FoundationalDiagnosticAvailability::RetainedHot {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::MustBeHotRequiresRetainedHotAvailability,
                );
            }
        }
        FoundationalDiagnosticDeliveryClass::CanDefer => {
            if !matches!(
                availability,
                FoundationalDiagnosticAvailability::RetainedHot
                    | FoundationalDiagnosticAvailability::DeferredCold
            ) {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::CanDeferRequiresRetainedOrDeferredAvailability,
                );
            }
        }
        FoundationalDiagnosticDeliveryClass::ReconstructableFromReplay => {
            if availability != FoundationalDiagnosticAvailability::Reconstructable {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::ReconstructableDeliveryRequiresReconstructableAvailability,
                );
            }
            if kind == FoundationalDiagnosticArtifactKind::Summary {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::SummaryDoesNotSupportReplayReconstruction,
                );
            }
            if kind == FoundationalDiagnosticArtifactKind::Report {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::ReportDoesNotSupportReplayReconstruction,
                );
            }
        }
        FoundationalDiagnosticDeliveryClass::UnavailableByPolicy => {
            if !matches!(
                availability,
                FoundationalDiagnosticAvailability::Redacted
                    | FoundationalDiagnosticAvailability::Unavailable
            ) {
                return Err(
                    FoundationalDiagnosticMaterializationLegalityDenial::UnavailableByPolicyRequiresRedactedOrUnavailableAvailability,
                );
            }
        }
    }

    Ok(())
}

pub const fn foundational_summary_artifact_kind() -> FoundationalSummaryArtifactKind {
    FoundationalSummaryArtifactKind(())
}

pub const fn foundational_report_artifact_kind() -> FoundationalReportArtifactKind {
    FoundationalReportArtifactKind(())
}

pub const fn foundational_failure_bundle_artifact_kind() -> FoundationalFailureBundleArtifactKind {
    FoundationalFailureBundleArtifactKind(())
}

pub const fn foundational_comparison_bundle_artifact_kind(
) -> FoundationalComparisonBundleArtifactKind {
    FoundationalComparisonBundleArtifactKind(())
}

pub const fn foundational_support_report_artifact_kind() -> FoundationalSupportReportArtifactKind {
    FoundationalSupportReportArtifactKind(())
}

pub const fn foundational_explanation_bundle_artifact_kind(
) -> FoundationalExplanationBundleArtifactKind {
    FoundationalExplanationBundleArtifactKind(())
}

mod sealed {
    use super::{
        FoundationalComparisonBundleArtifactKind, FoundationalExplanationBundleArtifactKind,
        FoundationalFailureBundleArtifactKind, FoundationalReportArtifactKind,
        FoundationalSummaryArtifactKind, FoundationalSupportReportArtifactKind,
    };

    pub trait Sealed {}

    impl Sealed for FoundationalSummaryArtifactKind {}
    impl Sealed for FoundationalReportArtifactKind {}
    impl Sealed for FoundationalFailureBundleArtifactKind {}
    impl Sealed for FoundationalComparisonBundleArtifactKind {}
    impl Sealed for FoundationalSupportReportArtifactKind {}
    impl Sealed for FoundationalExplanationBundleArtifactKind {}
}
