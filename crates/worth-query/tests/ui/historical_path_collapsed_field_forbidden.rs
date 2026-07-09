use worth_query::facade::{RequestedHistoricalPathClass, ResolvedHistoricalPathClass};

fn main() {
    let requested = RequestedHistoricalPathClass::RequestedRetainedSnapshotPath;
    let _resolved: ResolvedHistoricalPathClass = requested;
}
