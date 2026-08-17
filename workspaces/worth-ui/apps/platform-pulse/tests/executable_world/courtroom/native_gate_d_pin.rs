use std::io::Read;
use std::time::{Duration, Instant};

use crate::product_process::{CargoBuiltPlatformPulse, SuccessfulPlatformPulseExit};

#[test]
#[ignore = "requires the serialized interactive Windows 11 DX12 desktop"]
fn live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner() {
    let mut launch = CargoBuiltPlatformPulse::exact()
        .and_then(CargoBuiltPlatformPulse::launch_native_gate_d_pin_world)
        .expect("the Gate D pin product world launches under the desktop lease");
    let deadline = Instant::now() + Duration::from_secs(120);
    let exit = SuccessfulPlatformPulseExit::wait(&mut launch.process, deadline)
        .expect("the Gate D pin product world closes without residue");
    assert!(exit.status().success());
    let mut stdout = String::new();
    launch
        .stdout
        .read_to_string(&mut stdout)
        .expect("the Gate D pin evidence is readable");
    let evidence: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("the Gate D pin evidence is exact JSON");
    assert_eq!(evidence["schema"], "worth-ui-native-gate-d-pin-world-v2");
    assert_eq!(evidence["mounted_bindings"], 1);
    assert_eq!(evidence["pinned_layouts"], 2);
    assert!(evidence["native_peak_pin_count"].as_u64().unwrap() > 0);
    assert_eq!(evidence["physical_signal_runtimes"], 1);
    assert_eq!(evidence["physical_signal_workers"], 1);
    assert!(evidence["alpha_entries"].as_u64().unwrap() > 0);
    assert_eq!(evidence["color_entries"], 0);
    assert_eq!(evidence["terminal_zero"], true);
    println!("WORTH_UI_LEDGER_OBSERVATION={evidence}");
    println!("WORTH_UI_LEDGER_CASES={{\"P5-ATLAS-PINNING-01\":[\"runtime-transaction-owner\",\"native-signal-settlement\",\"last-owner-release\",\"terminal-census\"]}}");
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-ATLAS-PINNING-01\":2}}");
}
