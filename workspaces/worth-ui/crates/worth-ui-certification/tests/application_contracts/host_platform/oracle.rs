#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OracleRect {
    pub identity: u16,
    pub bounds: [u16; 4],
    pub rgba: [u8; 4],
    pub order: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OracleRectChange {
    pub identity: u16,
    pub previous: OracleRect,
    pub successor: OracleRect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct OracleDelta {
    pub changes: Vec<OracleRectChange>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct OracleExpectation {
    pub owner_delta_count: usize,
    pub discovery_count: usize,
    pub damage: Vec<[u16; 4]>,
    pub ordered_identities: Vec<u16>,
    pub vacated_replay_count: usize,
    pub baseline_clear: [u8; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OracleDenial {
    OwnerDeltaDropped,
    HostDiscoveryUsed,
    DamageWidened,
    PaintOrderChanged,
    VacatedReplayOmitted,
    BaselineClearChanged,
}

pub(super) fn expectation(baseline: &[OracleRect], delta: &OracleDelta) -> OracleExpectation {
    let mut successor = baseline.to_vec();
    let mut damage = Vec::with_capacity(delta.changes.len() * 2);
    for change in &delta.changes {
        let slot = successor
            .iter_mut()
            .find(|row| row.identity == change.identity)
            .expect("world delta identity belongs to the baseline");
        assert_eq!(*slot, change.previous);
        *slot = change.successor;
        damage.extend([change.previous.bounds, change.successor.bounds]);
    }
    successor.sort_by_key(|row| (row.order, row.identity));
    OracleExpectation {
        owner_delta_count: delta.changes.len(),
        discovery_count: 0,
        damage,
        ordered_identities: successor.iter().map(|row| row.identity).collect(),
        vacated_replay_count: delta.changes.len(),
        baseline_clear: [0, 0, 0, 0],
    }
}

pub(super) fn adjudicate(
    expected: &OracleExpectation,
    candidate: &OracleExpectation,
) -> Result<(), OracleDenial> {
    if candidate.owner_delta_count != expected.owner_delta_count {
        return Err(OracleDenial::OwnerDeltaDropped);
    }
    if candidate.discovery_count != 0 {
        return Err(OracleDenial::HostDiscoveryUsed);
    }
    if candidate.damage != expected.damage {
        return Err(OracleDenial::DamageWidened);
    }
    if candidate.ordered_identities != expected.ordered_identities {
        return Err(OracleDenial::PaintOrderChanged);
    }
    if candidate.vacated_replay_count != expected.vacated_replay_count {
        return Err(OracleDenial::VacatedReplayOmitted);
    }
    if candidate.baseline_clear != [0, 0, 0, 0] {
        return Err(OracleDenial::BaselineClearChanged);
    }
    Ok(())
}

pub(super) fn ordered_pixel(rows: &[OracleRect], point: [u16; 2]) -> [u8; 4] {
    let mut ordered = rows.to_vec();
    ordered.sort_by_key(|row| (row.order, row.identity));
    ordered
        .into_iter()
        .filter(|row| contains(row.bounds, point))
        .map(|row| row.rgba)
        .next_back()
        .unwrap_or([0, 0, 0, 0])
}

fn contains(bounds: [u16; 4], point: [u16; 2]) -> bool {
    point[0] >= bounds[0]
        && point[1] >= bounds[1]
        && point[0] < bounds[0] + bounds[2]
        && point[1] < bounds[1] + bounds[3]
}
