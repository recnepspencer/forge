use std::collections::BTreeMap;

use super::super::super::protocol::BoundedResidencyAllocationBoundaryObservation;

type AdmissionIdentity = (&'static str, Option<&'static str>, u64);

pub(super) fn verify(
    events: &[BoundedResidencyAllocationBoundaryObservation],
) -> Result<(), String> {
    let mut available_admissions = BTreeMap::<AdmissionIdentity, u64>::new();
    for event in events {
        match event.kind {
            "admission" if is_materialized(event.dimension) => {
                let identity = (event.dimension, event.scope, event.actual_units);
                *available_admissions.entry(identity).or_default() += 1;
            }
            "actualization" => {
                consume_prior_admission(&mut available_admissions, event)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn consume_prior_admission(
    available_admissions: &mut BTreeMap<AdmissionIdentity, u64>,
    actualization: &BoundedResidencyAllocationBoundaryObservation,
) -> Result<(), String> {
    let identity = (
        actualization.dimension,
        actualization.scope,
        admitted_units(actualization),
    );
    let Some(available) = available_admissions.get_mut(&identity) else {
        return Err(format!(
            "Courtroom C allocation `{}` materialized before matching admission",
            actualization.dimension
        ));
    };
    *available -= 1;
    if *available == 0 {
        available_admissions.remove(&identity);
    }
    Ok(())
}

fn admitted_units(event: &BoundedResidencyAllocationBoundaryObservation) -> u64 {
    if event.dimension == "metadata-bytes" {
        event.actual_units
    } else {
        event.requested_units
    }
}

fn is_materialized(dimension: &str) -> bool {
    matches!(
        dimension,
        "metadata-bytes" | "resident-bytes" | "dirty-replacement-bytes"
    )
}
