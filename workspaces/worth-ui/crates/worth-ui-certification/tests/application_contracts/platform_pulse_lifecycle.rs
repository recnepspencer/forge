#[path = "platform_pulse_lifecycle/native_oracle.rs"]
mod native_oracle;
#[path = "platform_pulse_lifecycle/observed_lifecycle.rs"]
mod observed_lifecycle;
#[path = "platform_pulse_lifecycle/real_watcher_world.rs"]
mod real_watcher_world;

use observed_lifecycle::ObservedPulseLifecycle;
use real_watcher_world::{RealWatcherPulseWorld, ValidPulseEdit};

#[test]
fn in_process_real_watcher_replaces_green_preserves_malformed_and_recovers_blue() {
    let mut world = RealWatcherPulseWorld::new();
    let initial = world.launch();
    let initial_generation = initial.shell.generation_identity().clone();
    let mut observations = ObservedPulseLifecycle::start();
    observations.first_publication(&initial);
    let mut shell = initial.shell;

    let green = world.replace(&mut shell, ValidPulseEdit::Green);
    observations.replacement(&green);
    observations.reject_stale_replacement(&green);
    assert_eq!(green.receipt.prior_generation(), &initial_generation);
    assert_eq!(
        green.receipt.active_generation(),
        shell.generation_identity()
    );
    let green_mounted = green
        .receipt
        .mounted_publication()
        .expect("green rebind has mounted publication")
        .clone();
    drop(green);

    let preserved = world.preserve_malformed(&mut shell);
    observations.preservation(&preserved);

    let recovered = world.replace(&mut shell, ValidPulseEdit::BlueRecovery);
    observations.reject_mismatched_mounted_receipt(&recovered, &green_mounted);
    observations.replacement(&recovered);
    assert_ne!(shell.generation_identity(), &preserved.generation);

    world.shutdown(shell, &mut observations);
}
