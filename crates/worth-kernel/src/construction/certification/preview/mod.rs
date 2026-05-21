mod bundle;
mod report;
mod suite;

pub use bundle::{
    prepare_primitive_construction_preview_bundle_from_hostility_suite,
    prepare_primitive_construction_preview_report_bundle, PrimitiveConstructionPreviewReportBundle,
    PrimitiveConstructionPreviewReportBundleError,
};
pub(crate) use report::prepare_primitive_construction_preview_row;
pub use report::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewRow, PrimitiveConstructionPreviewSurfaceReport,
    PrimitiveConstructionPreviewSurfaceReportError,
};
pub use suite::{
    prepare_primitive_construction_preview_hostility_suite_report,
    PrimitiveConstructionPreviewHostilitySuiteReport,
};

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_preview_hostility_suite_report,
        prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    };
    use crate::spatial_intent::{
        SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning, SpatialPreviewRichness,
    };

    #[test]
    fn preview_surface_report_makes_profile_dependent_classification_explicit() {
        let report = prepare_primitive_construction_preview_surface_report().expect("report");

        assert_eq!(
            report
                .row(PrimitiveConstructionPreviewCase::GrazingAskFirst)
                .expect("ask first")
                .commit_disposition(),
            SpatialIntentPreviewCommitDisposition::WouldRequireClarification
        );
        assert_eq!(
            report
                .row(PrimitiveConstructionPreviewCase::GrazingAggressiveSnap)
                .expect("aggressive")
                .commit_disposition(),
            SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
                worth_spatial::facade::SpatialIntentCandidate::SnapFlush
            )
        );
        assert!(report
            .row(PrimitiveConstructionPreviewCase::OverlapBlockedMerge)
            .expect("blocked")
            .warnings()
            .contains(&SpatialIntentPreviewWarning::BlockedFutureCandidate(
                worth_spatial::facade::SpatialBlockedCapability::MergeBoolean
            )));
        assert_eq!(
            report
                .row(PrimitiveConstructionPreviewCase::OverlapHighFidelity)
                .expect("high fidelity")
                .preview_richness(),
            SpatialPreviewRichness::HighFidelity
        );
    }

    #[test]
    fn preview_hostility_suite_is_reusable_for_later_compound_pressure() {
        let suite = prepare_primitive_construction_preview_hostility_suite_report().expect("suite");

        assert!(suite.suite_verified());
        assert!(suite
            .row(PrimitiveConstructionPreviewCase::GrazingAggressiveSnap)
            .is_some());
    }
}
