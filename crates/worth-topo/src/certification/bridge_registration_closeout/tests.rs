use super::{
    certify_topology_bridge_registration_closeout, TopologyBridgeRegistrationArea,
    TopologyBridgeRegistrationStatus,
};

#[test]
fn bridge_registration_closeout_certifies_all_phase_eight_areas() {
    let report = certify_topology_bridge_registration_closeout()
        .expect("worth-topo bridge registration closeout should certify");

    assert!(report.phase_eight_ready());
    assert_eq!(
        report.rows().len(),
        TopologyBridgeRegistrationArea::ALL.len()
    );
    for area in TopologyBridgeRegistrationArea::ALL {
        assert_eq!(
            report.status(area),
            TopologyBridgeRegistrationStatus::Closed
        );
    }
    assert!(report.closeout_digest().row_count > 0);
}

#[test]
fn bridge_registration_closeout_names_designated_survivors_for_every_area() {
    let report = certify_topology_bridge_registration_closeout()
        .expect("worth-topo bridge registration closeout should certify");

    for row in report.rows() {
        assert!(!row.reason().is_empty());
        assert!(!row.designated_survivor().is_empty());
        assert!(!row.row_digest().digest_hex.is_empty());
    }
}

#[test]
fn bridge_registration_closeout_proves_phase_eight_public_entry_rule() {
    let report = certify_topology_bridge_registration_closeout()
        .expect("worth-topo bridge registration closeout should certify");

    let public_entry_row = report
        .rows()
        .iter()
        .find(|row| row.area() == TopologyBridgeRegistrationArea::PublicEntry)
        .expect("bridge registration closeout should cover public entry");

    assert_eq!(
        public_entry_row.status(),
        TopologyBridgeRegistrationStatus::Closed
    );
    assert_eq!(
        public_entry_row.designated_survivor(),
        "src/query_domain.rs"
    );
    assert!(
        public_entry_row
            .reason()
            .contains("no longer teaches bridge builders or bridge registration packs"),
        "phase eight closeout should say the bridge entry story is gone from the public surface",
    );
}
