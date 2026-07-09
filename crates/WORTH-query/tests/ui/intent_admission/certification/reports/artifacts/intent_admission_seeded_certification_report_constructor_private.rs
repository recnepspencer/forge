use worth_query::facade::runtime::WorthQueryIntentAdmissionSeededCertificationReport;

fn main() {
    let _ = WorthQueryIntentAdmissionSeededCertificationReport {
        rows: Vec::new(),
        seeded_sequence_digest: String::new(),
        seed_replay_digest: String::new(),
        seed_generator_class_digest: String::new(),
    };
}
