use crate::harness::correspondence_history_certification::model::CorrespondenceHistoryCertificationRejection;
use crate::harness::correspondence_history_certification::row_catalog::CorrespondenceHistoryRejectionRowSpec;

pub(crate) fn compile_fail_rejection(
    spec: &CorrespondenceHistoryRejectionRowSpec,
) -> CorrespondenceHistoryCertificationRejection {
    CorrespondenceHistoryCertificationRejection {
        failure_class: spec.failure_class,
        failure_digest: format!("compile_fail:{}", spec.row_name),
        counter_snapshot_digest: None,
        compile_fail_case: spec.compile_fail_case,
    }
}
