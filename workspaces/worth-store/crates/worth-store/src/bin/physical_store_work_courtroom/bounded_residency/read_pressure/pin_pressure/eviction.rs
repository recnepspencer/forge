use worth_store::physical_runtime::{
    PhysicalRecordChunkBasis, PhysicalRecordChunkView, PhysicalRecordId, PhysicalRecordReader,
    RecordReadSession, ServingPhysicalRuntime,
};

use super::super::super::configuration::BoundedResidencyConfiguration;

// Leave one of the configured four pinned-identity slots available for the
// incoming loading owner; otherwise admission denies before victim selection.
const PROTECTED_IDENTITIES: usize = 3;

pub(in crate::bounded_residency) struct PinnedEvictionEvidence {
    pub(in crate::bounded_residency) forced_evictions: u64,
    pub(in crate::bounded_residency) pinned_frames_before: u32,
    pub(in crate::bounded_residency) pinned_frames_after: u32,
    pub(in crate::bounded_residency) pin_leases_before: u32,
    pub(in crate::bounded_residency) pin_leases_after: u32,
    pub(in crate::bounded_residency) bases_preserved: bool,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<PinnedEvictionEvidence, String> {
    let first_protected = records
        .len()
        .checked_sub(PROTECTED_IDENTITIES)
        .ok_or_else(|| "bounded-residency world lacks three protected records".to_owned())?;
    let protected = &records[first_protected..];
    serving
        .certification_physical_residency()
        .drain_unpinned_clean_frames();
    prewarm(serving.records(), protected, first_protected, configuration)?;

    let mut sessions =
        open_protected(serving.records(), protected, first_protected, configuration)?;
    let views = pin_first_chunks(&mut sessions)?;
    let bases = views
        .iter()
        .map(PhysicalRecordChunkView::basis)
        .collect::<Vec<_>>();
    let prefixes = views
        .iter()
        .map(protected_prefix)
        .collect::<Result<Vec<_>, _>>()?;
    require_protected_bases(serving, protected, &bases)?;
    let before = serving.residency_observation().counters();
    if before.pinned_frames() != PROTECTED_IDENTITIES as u32
        || before.pin_leases() != PROTECTED_IDENTITIES as u32
    {
        return Err(format!(
            "pinned-eviction siege did not hold three exact identities: {before:?}"
        ));
    }

    force_eviction(
        serving,
        records,
        first_protected,
        configuration,
        before.evictions(),
    )?;
    let after = serving.residency_observation().counters();
    let forced_evictions = after
        .evictions()
        .checked_sub(before.evictions())
        .ok_or_else(|| "pinned-eviction counter regressed".to_owned())?;
    let bases_preserved = views
        .iter()
        .zip(&bases)
        .zip(&prefixes)
        .all(|((view, basis), prefix)| {
            view.basis() == *basis && view.bytes().get(..prefix.len()) == Some(prefix.as_slice())
        });
    let authority_preserved = after.pinned_frames() == before.pinned_frames()
        && after.pin_leases() == before.pin_leases();
    if forced_evictions == 0 || !bases_preserved || !authority_preserved {
        return Err(format!(
            "forced eviction did not preserve pinned authority: before={before:?}, after={after:?}"
        ));
    }

    let evidence = PinnedEvictionEvidence {
        forced_evictions,
        pinned_frames_before: before.pinned_frames(),
        pinned_frames_after: after.pinned_frames(),
        pin_leases_before: before.pin_leases(),
        pin_leases_after: after.pin_leases(),
        bases_preserved,
    };
    drop(views);
    drop(sessions);
    let released = serving.residency_observation().counters();
    if released.pinned_frames() != 0 || released.pin_leases() != 0 {
        return Err("pinned-eviction siege leaked protected leases".to_owned());
    }
    Ok(evidence)
}

fn prewarm(
    reader: PhysicalRecordReader,
    records: &[PhysicalRecordId],
    first_ordinal: usize,
    configuration: BoundedResidencyConfiguration,
) -> Result<(), String> {
    for (offset, record) in records.iter().copied().enumerate() {
        let mut session = open(
            &reader,
            record,
            first_ordinal + offset,
            configuration,
            "prewarm",
        )?;
        session
            .next_chunk()
            .map_err(|failure| format!("pinned-eviction prewarm failed: {failure:?}"))?
            .ok_or_else(|| "pinned-eviction prewarm found no payload".to_owned())?;
    }
    Ok(())
}

fn open_protected(
    reader: PhysicalRecordReader,
    records: &[PhysicalRecordId],
    first_ordinal: usize,
    configuration: BoundedResidencyConfiguration,
) -> Result<Vec<RecordReadSession>, String> {
    records
        .iter()
        .copied()
        .enumerate()
        .map(|(offset, record)| {
            open(
                &reader,
                record,
                first_ordinal + offset,
                configuration,
                "repin",
            )
        })
        .collect()
}

fn pin_first_chunks<'sessions>(
    sessions: &'sessions mut [RecordReadSession],
) -> Result<Vec<PhysicalRecordChunkView<'sessions>>, String> {
    sessions
        .iter_mut()
        .map(|session| {
            session
                .next_chunk()
                .map_err(|failure| format!("pinned-eviction repin failed: {failure:?}"))?
                .ok_or_else(|| "pinned-eviction repin found no payload".to_owned())
        })
        .collect()
}

fn protected_prefix(view: &PhysicalRecordChunkView<'_>) -> Result<[u8; 8], String> {
    view.bytes()
        .get(..8)
        .ok_or_else(|| "pinned-eviction protected view omitted its payload prefix".to_owned())?
        .try_into()
        .map_err(|_| "pinned-eviction protected prefix had the wrong width".to_owned())
}

fn require_protected_bases(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    bases: &[PhysicalRecordChunkBasis],
) -> Result<(), String> {
    if bases.len() != PROTECTED_IDENTITIES
        || bases.iter().zip(records).any(|(basis, record)| {
            basis.store_identity() != serving.store_identity()
                || basis.store_generation() != serving.residency_observation().store_generation()
                || basis.record() != *record
        })
        || bases.iter().enumerate().any(|(index, basis)| {
            bases[..index]
                .iter()
                .any(|prior| prior.frame_coordinate() == basis.frame_coordinate())
        })
    {
        return Err(
            "pinned-eviction siege did not bind three distinct current Store frames".into(),
        );
    }
    Ok(())
}

fn force_eviction(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    protected_start: usize,
    configuration: BoundedResidencyConfiguration,
    evictions_before: u64,
) -> Result<(), String> {
    let reader = serving.records();
    for (ordinal, record) in records[..protected_start].iter().copied().enumerate() {
        let mut session = open(&reader, record, ordinal, configuration, "pressure")?;
        session
            .next_chunk()
            .map_err(|failure| format!("pinned-eviction pressure read failed: {failure:?}"))?
            .ok_or_else(|| "pinned-eviction pressure read found no payload".to_owned())?;
        drop(session);
        if serving.residency_observation().counters().evictions() > evictions_before {
            return Ok(());
        }
    }
    Err("pinned-eviction siege exhausted cold records without forcing eviction".to_owned())
}

fn open(
    reader: &PhysicalRecordReader,
    record: PhysicalRecordId,
    ordinal: usize,
    configuration: BoundedResidencyConfiguration,
    label: &str,
) -> Result<RecordReadSession, String> {
    reader
        .open(record, super::super::read_limits(configuration, ordinal)?)
        .map_err(|failure| format!("pinned-eviction {label} open failed: {failure:?}"))
}
