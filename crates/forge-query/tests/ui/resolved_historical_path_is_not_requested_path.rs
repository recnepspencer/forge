use forge_query::facade::{RequestedHistoricalPathClass, ResolvedHistoricalPathClass};

fn expects_requested(_: RequestedHistoricalPathClass) {}

fn main() {
    expects_requested(ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath);
}
