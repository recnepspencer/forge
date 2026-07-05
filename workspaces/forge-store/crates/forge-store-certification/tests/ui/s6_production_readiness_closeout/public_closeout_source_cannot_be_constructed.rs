use forge_store_readiness::{
    close_s6_production_readiness, S6MaterializedCertificationCloseoutSource,
    S6ProductionReadinessClosureInput,
};

fn main() {
    let source =
        S6MaterializedCertificationCloseoutSource::from_certification_materialized_evidence(
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
            todo!(),
            todo!(),
            Vec::new(),
        );
    let adoption =
        forge_store_readiness::adopt_materialized_s6_certification_evidence_for_closeout(&source)
            .unwrap();
    let _ = close_s6_production_readiness(
        S6ProductionReadinessClosureInput::from_phase13_adoption(adoption),
    );
}
