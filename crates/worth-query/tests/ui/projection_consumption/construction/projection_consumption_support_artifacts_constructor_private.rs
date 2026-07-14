use worth_query::facade::foundation::{ProjectionConsumptionSupportReport, ProjectionConsumptionSupportRow};

fn impossible<T>() -> T {
    panic!("fixture should fail before construction")
}

fn main() {
    let row = ProjectionConsumptionSupportRow {
        source_family: impossible(),
        fact_kind: impossible(),
        posture: impossible(),
        support_digest: String::new(),
    };
    let _ = ProjectionConsumptionSupportReport { rows: vec![row] };
}
