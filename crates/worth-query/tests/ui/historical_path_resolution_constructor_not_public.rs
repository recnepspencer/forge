use worth_query::facade::{
    AdmittedHistoricalPathClass, HistoricalPathResolved, RequestedHistoricalPathClass,
    ResolvedHistoricalPathClass,
};

fn main() {
    let _ = HistoricalPathResolved {
        requested_path_class: RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
        admitted_path_class: AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath,
        resolved_path_class: ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
    };
}
