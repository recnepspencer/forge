use super::*;

#[test]
fn identity_evolution_capability_disables_typed_and_early_when_query_section_is_off() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_query(ForgeQueryQueryConfig::disabled())
            .with_signal(ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(
                ForgeQueryRelationalConfig::disabled().with_historical_evaluation(false),
            ),
    )
    .expect("query-disabled facade config should still validate");

    let error = facade
        .identity_evolution_capability()
        .expect_err("disabled query section should deny identity evolution");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::IdentityEvolution)
    );
}

#[test]
fn support_report_tracks_disabled_signal_section_posture() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_signal(ForgeQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled facade config should still validate");
    let report = facade.support_report();

    let signal_posture = report
        .section_postures()
        .iter()
        .find(|posture| posture.section() == ForgeQueryConfigSectionFamily::Signal)
        .expect("signal posture should be present");
    assert_eq!(
        signal_posture.owner(),
        super::super::ForgeQuerySubsystemOwner::Signal
    );
    assert!(!signal_posture.enabled());
    assert!(report
        .unsupported_capability_families()
        .contains(&ForgeQueryCapabilityFamily::LiveQuery));
}
