use forge_store_certification::LayoutCourtroomReport;
use forge_store_layout_indexes::BTreeLookupExecutionOutcome;

fn require_owner_outcome(_: BTreeLookupExecutionOutcome) {}

fn certification_cannot_become_owner(report: LayoutCourtroomReport) {
    require_owner_outcome(report);
}

fn main() {}
