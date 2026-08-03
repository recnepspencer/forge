use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationCanonicalCore, UiHostObservationCanonicalCoreInput,
    UiHostObservationIntegrity, UiHostObservationLoss, UiHostObservationMountedBasis,
    UiHostObservationPayload, UiHostObservationReport, UiHostObservationReportDenial,
    UiHostObservationReportOutcome, UiHostObservationSequence, UiHostObservationSequenceRange,
    UiHostObservationTimeBasis,
};
use worth_ui_host_contract::{
    UiHostMeasurementSchemaVersion, UiHostObservationSchemaVersion, UiHostPresentationEpoch,
    UiHostProtocolContract, UiHostProtocolDenial, UiHostProtocolIdentity,
    UiHostProtocolNegotiation, UiHostProtocolSchemaFamily, UiHostProtocolVersion,
    UiMountedFrameSchemaVersion, UiMountedPresentationSchemaVersion,
};
use worth_ui_runtime::facade::mounted::{UiMountedFrameIdentity, UiSurfaceBindingGeneration};

use super::host_observation_fixture::{batch, pointer, report, source};
use super::mounted_application_lifecycle::published_mounted_world::published_observation_world;

#[path = "host_observation_denials/presentation_epoch.rs"]
mod presentation_epoch;
#[path = "host_observation_denials/receipt_forgery.rs"]
mod receipt_forgery;

struct CanonicalCorruptionCase {
    label: &'static str,
    mutate: fn(UiHostObservationBatch) -> UiHostObservationBatch,
    expected: UiHostObservationReportDenial,
}

#[test]
fn foreign_duplicate_instance_and_shutdown_reports_have_typed_effect_free_outcomes() {
    assert_foreign_protocol();
    assert_foreign_session();
    assert_foreign_binding();
    presentation_epoch::assert_wrong_presentation_epoch();
    assert_unknown_frame();
    receipt_forgery::assert_receipt_coordinate_denials();
    assert_reordered_sequence();
    assert_duplicate_suppression();
}

#[test]
fn corrupt_canonical_fields_deny_before_coalescing_or_retention() {
    let mutations = [
        CanonicalCorruptionCase {
            label: "integrity",
            mutate: corrupt_integrity,
            expected: UiHostObservationReportDenial::IntegrityMismatch,
        },
        CanonicalCorruptionCase {
            label: "sequence range",
            mutate: corrupt_sequence_range,
            expected: UiHostObservationReportDenial::SequenceGap,
        },
        CanonicalCorruptionCase {
            label: "payload byte count",
            mutate: corrupt_byte_count,
            expected: UiHostObservationReportDenial::MalformedBatch,
        },
        CanonicalCorruptionCase {
            label: "surface binding basis",
            mutate: corrupt_binding_without_resealing,
            expected: UiHostObservationReportDenial::IntegrityMismatch,
        },
        CanonicalCorruptionCase {
            label: "presentation epoch basis",
            mutate: presentation_epoch::corrupt_without_resealing,
            expected: UiHostObservationReportDenial::IntegrityMismatch,
        },
    ];
    for case in mutations {
        let mut world =
            published_observation_world(&format!("observation-integrity-{}", case.label));
        let valid = batch(
            source(&world.session, world.binding, &world.current),
            (1, 1),
            UiHostObservationLoss::Complete,
            vec![report(1, pointer(1, 10), &world.current)],
        );
        assert_eq!(
            world
                .session
                .validate_host_observation_batch((case.mutate)(valid)),
            UiHostObservationReportOutcome::Denied(case.expected),
            "corruption row `{}` reached the wrong boundary",
            case.label,
        );
        assert_eq!(
            world.session.retained_host_observation_report_count(),
            0,
            "corruption row `{}` must not retain or coalesce",
            case.label,
        );
    }
}

fn assert_foreign_session() {
    let mut world = published_observation_world("observation-foreign-session");
    let valid = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &world.current)],
    );
    let core = valid.canonical_core();
    let foreign = core_with(
        core,
        CanonicalCoreMutation {
            host_session: Some(core.host_session() + 1),
            ..Default::default()
        },
    );
    let raw = reseal(foreign, valid.reports().to_vec());
    assert_denial(
        &mut world,
        raw,
        UiHostObservationReportDenial::ForeignHostSession,
    );
}

fn assert_foreign_protocol() {
    assert_eq!(
        old_observation_contract().negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooOld(
            UiHostProtocolSchemaFamily::Observation
        ))
    );
    let mut world = published_observation_world("observation-foreign-protocol");
    let valid = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &world.current)],
    );
    let core = valid.canonical_core();
    let foreign = core_with(
        core,
        CanonicalCoreMutation {
            protocol: Some(compatible_noncurrent_protocol()),
            ..Default::default()
        },
    );
    assert_denial(
        &mut world,
        reseal(foreign, valid.reports().to_vec()),
        UiHostObservationReportDenial::ForeignProtocol,
    );
}

fn assert_foreign_binding() {
    let mut world = published_observation_world("observation-foreign-binding");
    let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let raw = batch(
        source(&world.session, binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &world.current)],
    );
    assert_denial(
        &mut world,
        raw,
        UiHostObservationReportDenial::BindingNotPresented,
    );
}

fn assert_unknown_frame() {
    let mut world = published_observation_world("observation-unknown-frame");
    let mut unknown = world.current;
    unknown.frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let raw = batch(
        source(&world.session, world.binding, &unknown),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &unknown)],
    );
    assert_denial(&mut world, raw, UiHostObservationReportDenial::UnknownFrame);
}

fn assert_duplicate_suppression() {
    let mut world = published_observation_world("observation-duplicate");
    let raw = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &world.current)],
    );
    assert!(matches!(
        world.session.validate_host_observation_batch(raw.clone()),
        UiHostObservationReportOutcome::Validated(_)
    ));
    assert!(matches!(
        world.session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Duplicate(_)
    ));
    assert_eq!(world.session.retained_host_observation_report_count(), 1);
}

fn assert_reordered_sequence() {
    let mut world = published_observation_world("observation-reordered-sequence");
    let first = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &world.current)],
    );
    assert!(matches!(
        world.session.validate_host_observation_batch(first),
        UiHostObservationReportOutcome::Validated(_)
    ));
    let reordered = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::Focus { focused: true },
            &world.current,
        )],
    );
    assert_eq!(
        world.session.validate_host_observation_batch(reordered),
        UiHostObservationReportOutcome::Denied(UiHostObservationReportDenial::SequenceReordered)
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 1);
}

fn corrupt_integrity(valid: UiHostObservationBatch) -> UiHostObservationBatch {
    UiHostObservationBatch::from_untrusted_parts(
        valid.canonical_core(),
        valid.reports().to_vec(),
        UiHostObservationIntegrity::from_untrusted(
            valid.integrity().diagnostic_value().wrapping_add(1),
        ),
    )
}

fn corrupt_sequence_range(valid: UiHostObservationBatch) -> UiHostObservationBatch {
    let core = valid.canonical_core();
    let corrupt = core_with(
        core,
        CanonicalCoreMutation {
            sequences: Some(range(1, 2)),
            ..Default::default()
        },
    );
    reseal(corrupt, valid.reports().to_vec())
}

fn corrupt_byte_count(valid: UiHostObservationBatch) -> UiHostObservationBatch {
    let core = valid.canonical_core();
    let corrupt = core_with(
        core,
        CanonicalCoreMutation {
            byte_count: Some(core.byte_count() + 1),
            ..Default::default()
        },
    );
    reseal(corrupt, valid.reports().to_vec())
}

fn corrupt_binding_without_resealing(valid: UiHostObservationBatch) -> UiHostObservationBatch {
    let core = valid.canonical_core();
    let corrupt = core_with(
        core,
        CanonicalCoreMutation {
            binding: Some(UiSurfaceBindingGeneration::mint_unbound().unwrap()),
            ..Default::default()
        },
    );
    UiHostObservationBatch::from_untrusted_parts(
        corrupt,
        valid.reports().to_vec(),
        valid.integrity(),
    )
}

#[derive(Default)]
struct CanonicalCoreMutation {
    protocol: Option<worth_ui::facade::observation_report::UiHostProtocolAgreement>,
    host_session: Option<u64>,
    binding: Option<UiSurfaceBindingGeneration>,
    presentation_epoch: Option<UiHostPresentationEpoch>,
    sequences: Option<UiHostObservationSequenceRange>,
    byte_count: Option<usize>,
}

fn core_with(
    source: UiHostObservationCanonicalCore,
    mutation: CanonicalCoreMutation,
) -> UiHostObservationCanonicalCore {
    UiHostObservationCanonicalCore::from_untrusted(UiHostObservationCanonicalCoreInput {
        protocol: mutation.protocol.unwrap_or(source.protocol()),
        host_session: mutation.host_session.unwrap_or(source.host_session()),
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis::new(
            source.frame(),
            mutation.binding.unwrap_or(source.binding()),
            mutation
                .presentation_epoch
                .unwrap_or(source.presentation().epoch()),
        ),
        sequences: mutation.sequences.unwrap_or(source.sequences()),
        report_count: source.report_count(),
        byte_count: mutation.byte_count.unwrap_or(source.byte_count()),
        loss: source.loss(),
    })
}

fn reseal(
    core: UiHostObservationCanonicalCore,
    reports: Vec<UiHostObservationReport>,
) -> UiHostObservationBatch {
    let integrity = UiHostObservationIntegrity::derive(core, &reports);
    UiHostObservationBatch::from_untrusted_parts(core, reports, integrity)
}

fn assert_denial(
    world: &mut super::mounted_application_lifecycle::published_mounted_world::PublishedObservationWorld,
    raw: UiHostObservationBatch,
    expected: UiHostObservationReportDenial,
) {
    assert_eq!(
        world.session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Denied(expected)
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 0);
}

fn range(first: u64, last: u64) -> UiHostObservationSequenceRange {
    UiHostObservationSequenceRange::new(
        UiHostObservationSequence::new(first),
        UiHostObservationSequence::new(last),
    )
}

fn compatible_noncurrent_protocol() -> worth_ui::facade::observation_report::UiHostProtocolAgreement
{
    let contract = UiHostProtocolContract::new(
        UiHostProtocolIdentity::worth_ui(),
        UiHostProtocolVersion::new(1),
        UiMountedFrameSchemaVersion::new(1),
        UiMountedPresentationSchemaVersion::new(1),
        UiHostProtocolContract::current().observation(),
        UiHostMeasurementSchemaVersion::new(1),
    );
    match contract.negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(denial) => {
            panic!("declared compatible predecessor protocol was denied: {denial:?}")
        }
    }
}

fn old_observation_contract() -> UiHostProtocolContract {
    UiHostProtocolContract::new(
        UiHostProtocolIdentity::worth_ui(),
        UiHostProtocolVersion::new(2),
        UiMountedFrameSchemaVersion::new(2),
        UiMountedPresentationSchemaVersion::new(2),
        UiHostObservationSchemaVersion::new(5),
        UiHostMeasurementSchemaVersion::new(2),
    )
}
