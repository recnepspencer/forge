use forge_query::facade::runtime::ForgeQueryIntentAdmissionSeededCertificationReport;

fn main() {
    let _ = ForgeQueryIntentAdmissionSeededCertificationReport {
        rows: Vec::new(),
        seeded_sequence_digest: String::new(),
        seed_replay_digest: String::new(),
        seed_generator_class_digest: String::new(),
    };
}
