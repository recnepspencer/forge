use forge_store_readiness::{
    S6MaterializedCertificationAdoptionReceipt, S6ReadinessCertificationProofSummary,
    S6ReadinessCertificationProofTopology,
};

fn main() {
    let _ = S6MaterializedCertificationAdoptionReceipt::from_executed_store_law_evidence(
        42,
        42,
        0b111_1111_1111,
        0b111_1111_1111,
        6,
        true,
        5,
        Vec::new(),
        2,
        2,
        S6ReadinessCertificationProofSummary::new(true, 5, 2, 2),
        S6ReadinessCertificationProofTopology::new(
            true, true, true, true, true, true, true, true, true, true, true, true, 5, 5, 5,
        ),
        Vec::new(),
    );
}
