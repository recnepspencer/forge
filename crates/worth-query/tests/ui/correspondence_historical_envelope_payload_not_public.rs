use worth_query::facade::foundation::CorrespondenceHistoricalEnvelope;

fn main() {
    let _: fn(&CorrespondenceHistoricalEnvelope) -> &[String] =
        CorrespondenceHistoricalEnvelope::payload;
}
