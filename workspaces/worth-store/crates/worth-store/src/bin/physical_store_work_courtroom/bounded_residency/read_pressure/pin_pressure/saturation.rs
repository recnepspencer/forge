use worth_store::physical_runtime::{
    PhysicalOperationAllocationScope, PhysicalRecordChunkBasis, PhysicalRecordChunkView,
    PhysicalRecordId, PhysicalRecordReader, PhysicalResidencyDimension,
    PhysicalResidencyRetryPosture, RecordReadObservation, RecordReadSession,
    RecordStreamFailureKind, ServingPhysicalRuntime,
};

use super::super::super::configuration::BoundedResidencyConfiguration;

const SATURATING_VIEWS: u32 = 6;
const SATURATING_IDENTITIES: u32 = 4;

pub(in crate::bounded_residency) struct PinSaturationEvidence {
    pub(in crate::bounded_residency) views: u32,
    pub(in crate::bounded_residency) unique_frame_identities: u32,
    pub(in crate::bounded_residency) zero_copy_events: u64,
    pub(in crate::bounded_residency) peak_pinned_frames: u32,
    pub(in crate::bounded_residency) peak_pin_leases: u32,
    pub(in crate::bounded_residency) dimension: PhysicalResidencyDimension,
    pub(in crate::bounded_residency) scope: PhysicalOperationAllocationScope,
    pub(in crate::bounded_residency) requested: u64,
    pub(in crate::bounded_residency) admitted: u64,
    pub(in crate::bounded_residency) limit: u64,
    pub(in crate::bounded_residency) retry_posture: PhysicalResidencyRetryPosture,
    pub(in crate::bounded_residency) effect_may_have_started: bool,
    pub(in crate::bounded_residency) basis_matched: bool,
}

struct PinScenario {
    first_cold: RecordReadSession,
    first_hot: RecordReadSession,
    second_cold: RecordReadSession,
    second_hot: RecordReadSession,
    third: RecordReadSession,
    fourth: RecordReadSession,
    denied: RecordReadSession,
    expected_records: [PhysicalRecordId; SATURATING_VIEWS as usize],
}

#[derive(Clone, Copy)]
struct OpenRequest {
    record: PhysicalRecordId,
    ordinal: usize,
    label: &'static str,
}

struct PressureObservation {
    peak_pinned_frames: u32,
    peak_pin_leases: u32,
    dimension: PhysicalResidencyDimension,
    scope: PhysicalOperationAllocationScope,
    requested: u64,
    admitted: u64,
    limit: u64,
    retry_posture: PhysicalResidencyRetryPosture,
    effect_may_have_started: bool,
    basis_matched: bool,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<PinSaturationEvidence, String> {
    let mut scenario = open_scenario(serving, records, configuration)?;
    let pressure = observe_saturation(serving, &mut scenario, configuration)?;
    let zero_copy_events = require_zero_copy(&scenario)?;
    release_and_verify(serving, scenario)?;
    Ok(pressure.into_evidence(zero_copy_events))
}

fn open_scenario(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<PinScenario, String> {
    let first = records
        .len()
        .checked_sub(SATURATING_IDENTITIES as usize)
        .ok_or_else(|| "bounded-residency world lacks four extent records".to_owned())?;
    let expected_records = [
        records[first],
        records[first],
        records[first + 1],
        records[first + 1],
        records[first + 2],
        records[first + 3],
    ];
    let reader = serving.records();
    Ok(PinScenario {
        first_cold: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[0],
                ordinal: first,
                label: "first cold",
            },
        )?,
        first_hot: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[1],
                ordinal: first,
                label: "first hot",
            },
        )?,
        second_cold: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[2],
                ordinal: first + 1,
                label: "second cold",
            },
        )?,
        second_hot: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[3],
                ordinal: first + 1,
                label: "second hot",
            },
        )?,
        third: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[4],
                ordinal: first + 2,
                label: "third",
            },
        )?,
        fourth: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[5],
                ordinal: first + 3,
                label: "fourth",
            },
        )?,
        denied: open(
            &reader,
            configuration,
            OpenRequest {
                record: expected_records[0],
                ordinal: first,
                label: "denied seventh",
            },
        )?,
        expected_records,
    })
}

fn observe_saturation(
    serving: &ServingPhysicalRuntime,
    scenario: &mut PinScenario,
    configuration: BoundedResidencyConfiguration,
) -> Result<PressureObservation, String> {
    let first_cold = first_view(&mut scenario.first_cold, "first cold")?;
    let first_hot = first_view(&mut scenario.first_hot, "first hot")?;
    let second_cold = first_view(&mut scenario.second_cold, "second cold")?;
    let second_hot = first_view(&mut scenario.second_hot, "second hot")?;
    let third = first_view(&mut scenario.third, "third")?;
    let fourth = first_view(&mut scenario.fourth, "fourth")?;
    let bases = [
        first_cold.basis(),
        first_hot.basis(),
        second_cold.basis(),
        second_hot.basis(),
        third.basis(),
        fourth.basis(),
    ];
    require_real_view_shape(serving, &bases, scenario.expected_records)?;
    require_nonempty_first_chunks([
        &first_cold,
        &first_hot,
        &second_cold,
        &second_hot,
        &third,
        &fourth,
    ])?;
    let failure = match scenario.denied.next_chunk() {
        Err(failure) => failure,
        Ok(_) => return Err("bounded-residency seventh public view succeeded".to_owned()),
    };
    if failure.kind() != RecordStreamFailureKind::PhysicalPressure
        || failure.completed_range() != (0..0)
    {
        return Err(format!(
            "bounded-residency seventh view returned imprecise failure {failure:?}"
        ));
    }
    let pressure = failure
        .pressure()
        .ok_or_else(|| "bounded-residency seventh view omitted pressure evidence".to_owned())?;
    let counters = serving.residency_observation().counters();
    if counters.pinned_frames() != configuration.pinned_frames()
        || counters.pin_leases() != configuration.pin_leases()
        || counters.frame_entries() > configuration.resident_frames()
    {
        return Err(format!(
            "bounded-residency public views missed their exact live ceilings: {counters:?}"
        ));
    }
    Ok(PressureObservation {
        peak_pinned_frames: counters.peak_pinned_frames(),
        peak_pin_leases: counters.peak_pin_leases(),
        dimension: pressure.dimension(),
        scope: pressure.scope(),
        requested: pressure.requested(),
        admitted: pressure.admitted(),
        limit: pressure.limit(),
        retry_posture: pressure.retry_posture(),
        effect_may_have_started: pressure.effect_may_have_started(),
        basis_matched: pressure.basis().store_identity() == serving.store_identity()
            && pressure.basis().record() == Some(scenario.expected_records[0])
            && pressure.basis().frame_coordinate() == Some(bases[0].frame_coordinate()),
    })
}

fn require_zero_copy(scenario: &PinScenario) -> Result<u64, String> {
    let (copy_events, copied_bytes) =
        scenario
            .observations()
            .into_iter()
            .fold((0_u64, 0_u64), |(events, bytes), observation| {
                (
                    events.saturating_add(observation.explicit_copy_count()),
                    bytes.saturating_add(observation.copied_bytes()),
                )
            });
    if copy_events != 0 || copied_bytes != 0 {
        return Err("bounded-residency public views performed explicit payload copies".to_owned());
    }
    Ok(copy_events)
}

fn release_and_verify(
    serving: &ServingPhysicalRuntime,
    scenario: PinScenario,
) -> Result<(), String> {
    drop(scenario);
    let released = serving.residency_observation().counters();
    if released.pinned_frames() != 0 || released.pin_leases() != 0 {
        return Err("bounded-residency public views leaked pin authority".to_owned());
    }
    Ok(())
}

impl PinScenario {
    fn observations(&self) -> [RecordReadObservation; 7] {
        [
            self.first_cold.observation(),
            self.first_hot.observation(),
            self.second_cold.observation(),
            self.second_hot.observation(),
            self.third.observation(),
            self.fourth.observation(),
            self.denied.observation(),
        ]
    }
}

impl PressureObservation {
    fn into_evidence(self, zero_copy_events: u64) -> PinSaturationEvidence {
        PinSaturationEvidence {
            views: SATURATING_VIEWS,
            unique_frame_identities: SATURATING_IDENTITIES,
            zero_copy_events,
            peak_pinned_frames: self.peak_pinned_frames,
            peak_pin_leases: self.peak_pin_leases,
            dimension: self.dimension,
            scope: self.scope,
            requested: self.requested,
            admitted: self.admitted,
            limit: self.limit,
            retry_posture: self.retry_posture,
            effect_may_have_started: self.effect_may_have_started,
            basis_matched: self.basis_matched,
        }
    }
}

fn open(
    reader: &PhysicalRecordReader,
    configuration: BoundedResidencyConfiguration,
    request: OpenRequest,
) -> Result<RecordReadSession, String> {
    reader
        .open(
            request.record,
            super::super::read_limits(configuration, request.ordinal)?,
        )
        .map_err(|failure| {
            format!(
                "bounded-residency {} view open failed: {failure:?}",
                request.label
            )
        })
}

fn first_view<'session>(
    session: &'session mut RecordReadSession,
    label: &str,
) -> Result<PhysicalRecordChunkView<'session>, String> {
    session
        .next_chunk()
        .map_err(|failure| format!("bounded-residency {label} view failed: {failure:?}"))?
        .ok_or_else(|| format!("bounded-residency {label} view found no payload"))
}

fn require_nonempty_first_chunks(
    views: [&PhysicalRecordChunkView<'_>; SATURATING_VIEWS as usize],
) -> Result<(), String> {
    if views
        .iter()
        .any(|view| view.bytes().is_empty() || view.logical_range().start != 0)
    {
        return Err("bounded-residency pin view was empty or not the first chunk".to_owned());
    }
    Ok(())
}

fn require_real_view_shape(
    serving: &ServingPhysicalRuntime,
    bases: &[PhysicalRecordChunkBasis; SATURATING_VIEWS as usize],
    records: [PhysicalRecordId; SATURATING_VIEWS as usize],
) -> Result<(), String> {
    if bases.iter().zip(records).any(|(basis, record)| {
        basis.store_identity() != serving.store_identity()
            || basis.store_generation() != serving.residency_observation().store_generation()
            || basis.record() != record
    }) {
        return Err("bounded-residency view basis was foreign".to_owned());
    }
    let unique = bases
        .iter()
        .enumerate()
        .filter(|(index, basis)| {
            !bases[..*index]
                .iter()
                .any(|prior| prior.frame_coordinate() == basis.frame_coordinate())
        })
        .count();
    if unique != SATURATING_IDENTITIES as usize
        || bases[0].frame_coordinate() != bases[1].frame_coordinate()
        || bases[2].frame_coordinate() != bases[3].frame_coordinate()
    {
        return Err("bounded-residency views did not represent six leases over four frames".into());
    }
    Ok(())
}
