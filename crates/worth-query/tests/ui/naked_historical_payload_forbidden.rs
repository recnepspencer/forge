use worth_query::facade::foundation::HistoricalPathResolved;

fn main() {
    let _: fn(&HistoricalPathResolved) -> &[u8] = HistoricalPathResolved::payload;
}
