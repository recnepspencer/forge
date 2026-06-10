use worth_spatial::facade::planar_m6_closeout::{
    M6PlanarCloseoutQueryCertification, M6PremetabossFamily,
};

use super::fixture::{closeout_contracts, complete_certification};

#[test]
fn m6_certification_bundle_proves_all_premetaboss_families() {
    let certification = complete_certification("m6-closeout-premetaboss");
    let contracts = closeout_contracts("m6-closeout-premetaboss");
    let plan = M6PlanarCloseoutQueryCertification::from_certification(certification)
        .compile(&contracts)
        .expect("M6 closeout plan");
    assert_eq!(
        plan.inspected_closeout_rows(),
        M6PremetabossFamily::ALL.len() + 6 + 1
    );

    let receipt = plan.certify().expect("M6 closeout receipt");
    assert!(receipt.proves_all_premetaboss_families());
    assert_eq!(
        receipt.counters().premetaboss_rows(),
        M6PremetabossFamily::ALL.len()
    );
    assert_eq!(receipt.boolean_result(), None);
}
