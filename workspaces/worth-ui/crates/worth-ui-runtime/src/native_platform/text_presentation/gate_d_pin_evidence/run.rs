use std::cell::RefCell;
use std::rc::Rc;

use worth_ui_host_native::{UiNativeWindowConfiguration, WorthUiPreparedNativeHost};

use super::{Evidence, GateDPinClient, UiGateDPinWorldEvidence};

pub(crate) fn run_gate_d_pin_world() -> UiGateDPinWorldEvidence {
    let evidence = Rc::new(RefCell::new(Evidence::default()));
    let host = WorthUiPreparedNativeHost::prepare_qualified();
    let (adapter, event_loop) = host.into_parts(UiNativeWindowConfiguration::qualified(
        "WORTH UI Gate D Pin Courtroom",
        [160, 96],
    ));
    let outcome = event_loop.run(GateDPinClient::new(adapter, Rc::clone(&evidence)));
    let report = outcome.expect_err("Gate E presentation remains deliberately absent");
    let peak = report.peak_census();
    let peak_pin_count = report.peak_text_pins().len();
    let peak_pins_match = report.peak_text_pins().iter().all(|observed| {
        evidence
            .borrow()
            .expected_pins
            .iter()
            .any(|expected| observed.matches(*expected))
    });
    let terminal_zero = finish_cleanup(report);
    let evidence = evidence.borrow();
    validate_runtime_observation(&evidence, peak, peak_pin_count, peak_pins_match);
    UiGateDPinWorldEvidence {
        mounted_bindings: u32::from(evidence.first_committed)
            + u32::from(evidence.second_committed),
        pinned_layouts: 1,
        expected_pin_count: u32::try_from(evidence.expected_pins.len()).unwrap_or(u32::MAX),
        native_committed_pin_count: evidence.committed_pin_census[1],
        native_peak_pin_count: u32::try_from(peak.text_atlas_pins).unwrap_or(u32::MAX),
        physical_signal_runtimes: u32::try_from(peak.physical_signal_runtimes).unwrap_or(u32::MAX),
        pressure_transactions: evidence.pressure_transactions,
        pressure_releases: evidence.pressure_releases,
        evictions: evidence.evictions,
        atlas_transactions: u32::from(evidence.first_committed)
            + u32::from(evidence.second_committed)
            + evidence.pressure_transactions
            + evidence.pressure_releases
            + u32::from(evidence.final_release_crossed_native),
        local_owner_releases: u32::from(evidence.first_release_was_local),
        native_final_releases: u32::from(evidence.final_release_crossed_native),
        terminal_zero,
        rasterized_glyphs: evidence.rasterized_glyphs,
    }
}

fn finish_cleanup(report: worth_ui_host_native::UiNativeEventLoopStopReport) -> bool {
    if report.terminal_census().is_zero() {
        return true;
    }
    let mut cleanup = report
        .into_cleanup()
        .expect("an unsettled physical transaction retains cleanup authority");
    for _ in 0..8 {
        match cleanup.retry() {
            Ok(census) => return census.is_zero(),
            Err(retained) => cleanup = retained,
        }
    }
    false
}

fn validate_runtime_observation(
    evidence: &Evidence,
    peak: worth_ui_host_native::UiNativeResourceCensus,
    peak_pin_count: usize,
    peak_pins_match: bool,
) {
    assert!(evidence.first_committed);
    assert!(evidence.second_committed);
    assert!(evidence.first_release_was_local);
    assert!(evidence.final_release_crossed_native);
    assert!(evidence.cleanup_complete);
    assert_eq!(evidence.committed_pin_census.len(), 7);
    assert!(evidence.committed_pin_census[0] > 0);
    assert_eq!(
        evidence.committed_pin_census[0],
        evidence.committed_pin_census[1]
    );
    assert_eq!(evidence.pressure_transactions, 5);
    assert_eq!(evidence.pressure_releases, 5);
    assert!(evidence.evictions > 0);
    assert_eq!(peak_pin_count, evidence.expected_pins.len());
    assert!(peak_pins_match);
    assert_eq!(peak.text_atlas_pins, evidence.expected_pins.len());
    assert_eq!(peak.physical_signal_runtimes, 1);
    assert_eq!(peak.physical_signal_workers, 1);
}
