use super::{UiNativeRecoveryCause, UiNativeRecoveryLineage, UiNativeRecoveryRegistry};

#[test]
fn one_binding_retains_the_strongest_current_recovery_cause() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(7, UiNativeRecoveryCause::Resize);
    recovery.require(7, UiNativeRecoveryCause::SurfaceLost);
    recovery.require(7, UiNativeRecoveryCause::Dpi);

    let preparation = recovery
        .physical_preparation(7)
        .expect("surface loss owns one physical preparation");
    assert!(recovery.commit_physical(preparation, 11, 13));
    let requirement = recovery.take(7).expect("binding requires recovery");
    assert_eq!(requirement.binding(), 7);
    assert_eq!(requirement.cause(), UiNativeRecoveryCause::SurfaceLost);
    assert_eq!(recovery.len(), 1);
    assert!(recovery.settle(requirement));
    assert_eq!(recovery.len(), 0);
}

#[test]
fn two_bindings_consume_one_generation_bearing_physical_recovery_fact() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(11, UiNativeRecoveryCause::SurfaceLost);
    recovery.require(17, UiNativeRecoveryCause::SurfaceLost);

    let preparation = recovery
        .physical_preparation(11)
        .expect("the first binding exposes the global preparation");
    assert_eq!(recovery.physical_preparation(17), Some(preparation));
    assert!(recovery.commit_physical(preparation, 19, 23));
    assert_eq!(recovery.physical_preparation(11), None);
    assert_eq!(recovery.physical_preparation(17), None);
    assert_eq!(recovery.physical_fact(11).unwrap().generations(), [19, 23]);
    assert_eq!(recovery.physical_fact(17).unwrap().generations(), [19, 23]);

    let first = recovery.take(11).expect("first semantic reconstruction");
    assert!(recovery.settle(first));
    assert!(recovery.physical_fact(17).is_some());
    let second = recovery.take(17).expect("second semantic reconstruction");
    assert!(recovery.settle(second));
    assert_eq!(recovery.len(), 0);
}

#[test]
fn stronger_physical_loss_supersedes_a_prepared_epoch() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(29, UiNativeRecoveryCause::SurfaceLost);
    let surface = recovery.physical_preparation(29).unwrap();
    assert!(recovery.commit_physical(surface, 31, 37));

    recovery.require(29, UiNativeRecoveryCause::DeviceLost);
    assert!(recovery.physical_fact(29).is_none());
    let device = recovery
        .physical_preparation(29)
        .expect("stronger loss starts one successor epoch");
    assert_ne!(surface, device);
    assert!(recovery.commit_physical(device, 41, 37));
    assert_eq!(recovery.physical_fact(29).unwrap().generations(), [41, 37]);
}

#[test]
fn successor_binding_inherits_the_predecessor_recovery_authority() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(43, UiNativeRecoveryCause::SurfaceLost);
    let preparation = recovery.physical_preparation(43).unwrap();
    assert!(recovery.commit_physical(preparation, 47, 53));

    assert!(recovery.transfer(43, 59));
    assert!(!recovery.requires(43));
    assert_eq!(recovery.cause(59), Some(UiNativeRecoveryCause::SurfaceLost));
    assert_eq!(recovery.physical_fact(59).unwrap().generations(), [47, 53]);
}

#[test]
fn deregistration_gap_parks_one_exact_recovery_lineage_for_its_successor() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    let lineage = UiNativeRecoveryLineage {
        host_session: 61,
        semantic_surface: 67,
        host_surface: 71,
    };
    recovery.require(73, UiNativeRecoveryCause::Resize);

    assert!(recovery.park(73, lineage));
    assert!(!recovery.requires(73));
    assert_eq!(recovery.len(), 1);
    assert!(recovery.claim(lineage, 79));
    assert_eq!(recovery.cause(79), Some(UiNativeRecoveryCause::Resize));
    assert_eq!(recovery.len(), 1);
}

#[test]
fn indeterminate_semantics_do_not_hide_later_surface_loss() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(83, UiNativeRecoveryCause::PresentationIndeterminate);
    recovery.require(83, UiNativeRecoveryCause::SurfaceLost);

    assert_eq!(
        recovery.cause(83),
        Some(UiNativeRecoveryCause::PresentationIndeterminate)
    );
    assert!(!recovery.ready(83));
    let physical = recovery
        .physical_preparation(83)
        .expect("surface loss remains an independent physical obligation");
    assert!(recovery.commit_physical(physical, 89, 97));
    assert!(recovery.ready(83));
}

#[test]
fn derived_state_loss_does_not_hide_later_surface_outdated_repair() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(101, UiNativeRecoveryCause::DerivedStateLost);
    recovery.require(101, UiNativeRecoveryCause::SurfaceOutdated);

    assert_eq!(
        recovery.cause(101),
        Some(UiNativeRecoveryCause::DerivedStateLost)
    );
    assert!(recovery.physical_preparation(101).is_some());
    assert!(!recovery.ready(101));
}

#[test]
fn one_physical_repair_gates_all_current_semantic_reconstructions() {
    let mut recovery = UiNativeRecoveryRegistry::default();
    recovery.require(103, UiNativeRecoveryCause::DerivedStateLost);
    recovery.require(107, UiNativeRecoveryCause::PresentationIndeterminate);
    recovery.require(103, UiNativeRecoveryCause::SurfaceLost);

    let physical = recovery.physical_preparation(103).unwrap();
    assert_eq!(recovery.physical_preparation(107), Some(physical));
    assert!(!recovery.ready(103));
    assert!(!recovery.ready(107));
    assert!(recovery.commit_physical(physical, 109, 113));

    let first = recovery.take(103).unwrap();
    let second = recovery.take(107).unwrap();
    assert!(recovery.settle(first));
    assert!(recovery.settle(second));
    assert_eq!(recovery.len(), 0);
}
