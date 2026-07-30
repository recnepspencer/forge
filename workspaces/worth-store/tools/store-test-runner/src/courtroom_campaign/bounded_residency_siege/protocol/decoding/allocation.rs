use super::super::{
    BoundedResidencyAllocationBoundaryObservation, BoundedResidencyAllocationDimensionObservation,
    BoundedResidencyAllocationObservation, BoundedResidencyAllocationTraceObservation,
    BoundedResidencyScopeObservation,
};

const DIMENSION_NAMES: [&str; 19] = [
    "total-bytes",
    "resident-bytes",
    "metadata-bytes",
    "frame-entries",
    "pinned-frames",
    "pin-leases",
    "dirty-frames",
    "dirty-replacement-bytes",
    "operation-bytes",
    "scope-foreground-read",
    "scope-foreground-write",
    "scope-recovery",
    "scope-scrub",
    "scope-maintenance",
    "scope-verification",
    "scope-blob",
    "speculative-read-ahead",
    "speculative-prefetch",
    "speculative-write-behind",
];

pub(in super::super) fn parse(
    lines: &[String],
) -> Result<BoundedResidencyAllocationObservation, String> {
    Ok(BoundedResidencyAllocationObservation {
        scopes: parse_scopes(lines)?,
        dimensions: parse_dimensions(lines)?,
        trace: parse_trace(lines)?,
    })
}

fn parse_trace(lines: &[String]) -> Result<BoundedResidencyAllocationTraceObservation, String> {
    let header = unique_marker(lines, "BOUNDED_RESIDENCY_ALLOCATION_TRACE ", 6)?;
    let event_count = number(header[3], "allocation boundary event count")?;
    let events = lines
        .iter()
        .filter(|line| line.starts_with("BOUNDED_RESIDENCY_ALLOCATION_EVENT "))
        .map(|line| parse_boundary(line))
        .collect::<Result<Vec<_>, _>>()?;
    if usize::try_from(event_count).ok() != Some(events.len()) {
        return Err(format!(
            "Courtroom C allocation trace declared {event_count} events but emitted {}",
            events.len()
        ));
    }
    Ok(BoundedResidencyAllocationTraceObservation {
        store: super::fixed_hex(header[1], "allocation trace Store identity")?,
        pool_incarnation: number(header[2], "allocation pool incarnation")?,
        event_count,
        process: number(header[4], "allocation trace process")?,
        attributed_actualizations: number(header[5], "allocation attributed actualizations")?,
        events: events.into_boxed_slice(),
    })
}

fn parse_boundary(line: &str) -> Result<BoundedResidencyAllocationBoundaryObservation, String> {
    let value = line.split_whitespace().collect::<Vec<_>>();
    if value.len() != 9 {
        return Err(format!(
            "malformed Courtroom C allocation boundary marker `{line}`"
        ));
    }
    Ok(BoundedResidencyAllocationBoundaryObservation {
        sequence: number(value[1], "allocation boundary sequence")?,
        kind: boundary_kind(value[2])?,
        dimension: dimension_name(value[3])?,
        scope: optional_scope(value[4])?,
        requested_units: number(value[5], "allocation requested units")?,
        actual_units: number(value[6], "allocation actual units")?,
        process: number(value[7], "allocation event process")?,
        physical_operation: optional_operation(value[8])?,
    })
}

fn parse_scopes(lines: &[String]) -> Result<BoundedResidencyScopeObservation, String> {
    let value = unique_marker(lines, "BOUNDED_RESIDENCY_SCOPES ", 10)?;
    Ok(BoundedResidencyScopeObservation {
        admitted_scopes: number(value[1], "admitted operation scopes")?,
        exact_scope_denials: number(value[2], "exact scope denials")?,
        global_envelope_denied: boolean(value[3], "global envelope denial")?,
        global_denial_requested: number(value[4], "global denial requested bytes")?,
        global_denial_current: number(value[5], "global denial current bytes")?,
        global_denial_limit: number(value[6], "global denial limit bytes")?,
        peak_operation_bytes: number(value[7], "peak operation bytes")?,
        terminal_operation_bytes: number(value[8], "terminal operation bytes")?,
        all_effect_free: boolean(value[9], "scope denials effect posture")?,
    })
}

fn parse_dimensions(
    lines: &[String],
) -> Result<[BoundedResidencyAllocationDimensionObservation; 19], String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with("BOUNDED_RESIDENCY_ALLOCATION "))
        .collect::<Vec<_>>();
    if matching.len() != DIMENSION_NAMES.len() {
        return Err(format!(
            "expected {} allocation-dimension markers, found {}",
            DIMENSION_NAMES.len(),
            matching.len()
        ));
    }
    matching
        .into_iter()
        .map(|line| parse_dimension(line))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| "Courtroom C allocation dimension width changed".to_owned())
}

fn parse_dimension(line: &str) -> Result<BoundedResidencyAllocationDimensionObservation, String> {
    let value = line.split_whitespace().collect::<Vec<_>>();
    if value.len() != 14 {
        return Err(format!("malformed Courtroom C allocation marker `{line}`"));
    }
    Ok(BoundedResidencyAllocationDimensionObservation {
        name: dimension_name(value[1])?,
        attempts: number(value[2], "allocation attempts")?,
        admissions: number(value[3], "allocation admissions")?,
        releases: number(value[4], "allocation releases")?,
        denials: number(value[5], "allocation denials")?,
        allocator_failures: number(value[6], "allocator failures")?,
        admitted_units: number(value[7], "admitted units")?,
        released_units: number(value[8], "released units")?,
        denied_units: number(value[9], "denied units")?,
        active_units: number(value[10], "active units")?,
        current_units: number(value[11], "current units")?,
        peak_units: number(value[12], "peak units")?,
        limit_units: number(value[13], "limit units")?,
    })
}

fn dimension_name(encoded: &str) -> Result<&'static str, String> {
    DIMENSION_NAMES
        .into_iter()
        .find(|name| *name == encoded)
        .ok_or_else(|| format!("unknown allocation dimension `{encoded}`"))
}

fn boundary_kind(encoded: &str) -> Result<&'static str, String> {
    const KINDS: [&str; 5] = [
        "admission",
        "release",
        "denial",
        "allocator-failure",
        "actualization",
    ];
    KINDS
        .into_iter()
        .find(|kind| *kind == encoded)
        .ok_or_else(|| format!("unknown allocation boundary kind `{encoded}`"))
}

fn optional_scope(encoded: &str) -> Result<Option<&'static str>, String> {
    const SCOPES: [&str; 7] = [
        "foreground-read",
        "foreground-write",
        "recovery",
        "scrub",
        "maintenance",
        "verification",
        "blob",
    ];
    if encoded == "none" {
        return Ok(None);
    }
    SCOPES
        .into_iter()
        .find(|scope| *scope == encoded)
        .map(Some)
        .ok_or_else(|| format!("unknown allocation scope `{encoded}`"))
}

fn optional_operation(encoded: &str) -> Result<Option<u64>, String> {
    if encoded == "none" {
        return Ok(None);
    }
    let operation = number(encoded, "allocation physical operation")?;
    if operation == 0 {
        return Err("allocation physical operation cannot be zero".to_owned());
    }
    Ok(Some(operation))
}

fn unique_marker<'lines>(
    lines: &'lines [String],
    prefix: &str,
    count: usize,
) -> Result<Vec<&'lines str>, String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    let [line] = matching.as_slice() else {
        return Err(format!(
            "expected one `{prefix}` marker, found {}",
            matching.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != count {
        return Err(format!("malformed Courtroom C marker `{line}`"));
    }
    Ok(fields)
}

fn boolean(encoded: &str, label: &str) -> Result<bool, String> {
    match encoded {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{label} must be `true` or `false`")),
    }
}

fn number<T: std::str::FromStr>(encoded: &str, label: &str) -> Result<T, String> {
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}
