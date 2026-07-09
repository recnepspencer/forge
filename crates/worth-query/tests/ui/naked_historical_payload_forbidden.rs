use worth_query::facade::HistoricalPathResolved;

fn main() {
    let _: fn(&HistoricalPathResolved) -> &[u8] = HistoricalPathResolved::payload;
}
