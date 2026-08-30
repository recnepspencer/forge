use sha2::{Digest, Sha256};

use super::test_world::{fresh_row, root_fixture};
use super::*;

#[test]
fn role_bound_wire_and_connector_requests_reject_substitution_without_editor_truth() {
    let fixture = root_fixture();
    let scenario = fixture.scenario.identity();
    let run: [u8; 32] = Sha256::digest(b"c9-root-run-1").into();
    let recovery_wire = RootWireIdentity::bind(RootWireRole::Recovery, scenario, run).unwrap();
    let verifier_wire =
        RootWireIdentity::bind(RootWireRole::OfflineVerifier, scenario, run).unwrap();
    for role in [
        RootWireRole::Producer,
        RootWireRole::ArtifactEditor,
        RootWireRole::Recovery,
        RootWireRole::OfflineVerifier,
        RootWireRole::ParentOracle,
    ] {
        let wire = RootWireIdentity::bind(role, scenario, run).unwrap();
        assert_eq!(wire.role(), role);
        assert_eq!(wire.scenario_identity(), scenario);
        assert_eq!(wire.run_identity(), run);
        assert_ne!(wire.identity(), [0; 32]);
    }
    assert_eq!(verifier_wire.protocol(), "store.physical.c9-root-localization");
    assert_eq!(verifier_wire.version(), 1);
    assert_eq!(verifier_wire.scenario_identity(), scenario);
    assert_eq!(verifier_wire.run_identity(), run);
    assert_ne!(verifier_wire.identity(), [0; 32]);

    let mut counters = RootLocalizationCounters::default();
    let row = fresh_row(&fixture, "connector-row", &mut counters);
    let target = fixture
        .manifest
        .target_for_role(RootArtifactRole::AddressedRootManifest);
    let edit =
        DeclaredRootCorruption::for_code(&fixture.manifest, target, RootCorruptionCode::P).unwrap();
    apply_declared_corruption(row.root(), &fixture.manifest, &edit, &mut counters).unwrap();
    let runtime = RuntimeRootObservationConnectorRequest::new(
        &fixture.scenario,
        &row,
        run,
        recovery_wire.clone(),
    )
    .unwrap();
    let offline = OfflineRootObservationConnectorRequest::new(
        &fixture.scenario,
        &row,
        run,
        verifier_wire.clone(),
    )
    .unwrap();
    assert_eq!(runtime.isolated_store_root(), row.root());
    assert_eq!(offline.isolated_store_root(), row.root());
    assert_eq!(
        runtime.external_report_path(),
        fixture.scenario.reports().runtime()
    );
    assert_eq!(
        offline.external_report_path(),
        fixture.scenario.reports().offline()
    );
    assert_eq!(runtime.wire().role(), RootWireRole::Recovery);
    assert_eq!(offline.wire().role(), RootWireRole::OfflineVerifier);

    assert_eq!(
        RuntimeRootObservationConnectorRequest::new(&fixture.scenario, &row, run, verifier_wire),
        Err(RootWireDenial::RoleSubstitution)
    );
    let mut encoded_wire = bincode::serialize(&recovery_wire).unwrap();
    let protocol = recovery_wire.protocol().as_bytes();
    let protocol_offset = encoded_wire
        .windows(protocol.len())
        .position(|window| window == protocol)
        .unwrap();
    encoded_wire[protocol_offset] ^= 1;
    let substituted_wire: RootWireIdentity = bincode::deserialize(&encoded_wire).unwrap();
    assert_eq!(
        RuntimeRootObservationConnectorRequest::new(
            &fixture.scenario,
            &row,
            run,
            substituted_wire,
        ),
        Err(RootWireDenial::ProtocolSubstitution)
    );
    let wrong_scenario = RootWireIdentity::bind(RootWireRole::Recovery, [8; 32], run).unwrap();
    let wrong_offline_scenario =
        RootWireIdentity::bind(RootWireRole::OfflineVerifier, [8; 32], run).unwrap();
    assert_eq!(
        RuntimeRootObservationConnectorRequest::new(
            &fixture.scenario,
            &row,
            run,
            wrong_scenario,
        ),
        Err(RootWireDenial::ScenarioSubstitution)
    );
    assert_eq!(
        OfflineRootObservationConnectorRequest::new(
            &fixture.scenario,
            &row,
            run,
            wrong_offline_scenario,
        ),
        Err(RootWireDenial::ScenarioSubstitution)
    );
    let wrong_run = RootWireIdentity::bind(RootWireRole::Recovery, scenario, [9; 32]).unwrap();
    let wrong_offline_run =
        RootWireIdentity::bind(RootWireRole::OfflineVerifier, scenario, [9; 32]).unwrap();
    assert_eq!(
        RuntimeRootObservationConnectorRequest::new(&fixture.scenario, &row, run, wrong_run),
        Err(RootWireDenial::RunSubstitution)
    );
    assert_eq!(
        OfflineRootObservationConnectorRequest::new(
            &fixture.scenario,
            &row,
            run,
            wrong_offline_run,
        ),
        Err(RootWireDenial::RunSubstitution)
    );
}
