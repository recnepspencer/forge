use worth_query::facade::CorrespondenceHistoricalEnvelope;

fn main() {
    let _: fn(&CorrespondenceHistoricalEnvelope) -> &[String] =
        CorrespondenceHistoricalEnvelope::payload;
}
