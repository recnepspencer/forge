use super::{
    WorthQueryPublishedApplicationDisclosureIdentity as Identity,
    WorthQueryPublishedApplicationDisclosurePosture::{Governed, Public},
};

#[test]
fn structural_identity_changes_for_each_public_axis_only() {
    let baseline = Identity {
        posture: Governed,
        disclosure_decision_count: 2,
        disclosed_value_count: 1,
        omitted_value_count: 1,
        authorization_decision_fact_count: 2,
    };
    assert_eq!(baseline, baseline);
    for changed in [
        Identity {
            posture: Public,
            ..baseline
        },
        Identity {
            disclosure_decision_count: 3,
            ..baseline
        },
        Identity {
            disclosed_value_count: 2,
            ..baseline
        },
        Identity {
            omitted_value_count: 2,
            ..baseline
        },
        Identity {
            authorization_decision_fact_count: 3,
            ..baseline
        },
    ] {
        assert_ne!(baseline, changed);
    }
}
