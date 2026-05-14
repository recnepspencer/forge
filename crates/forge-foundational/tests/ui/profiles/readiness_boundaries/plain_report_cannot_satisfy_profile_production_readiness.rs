use forge_foundational::{
    require_foundational_profile_milestone3_production_test_readiness,
    FoundationalProfileProductionReadinessReport,
};

fn main() {
    let report: FoundationalProfileProductionReadinessReport =
        forge_foundational::foundational_profile_milestone3_readiness_report();
    let _ = require_foundational_profile_milestone3_production_test_readiness(&report);
}
