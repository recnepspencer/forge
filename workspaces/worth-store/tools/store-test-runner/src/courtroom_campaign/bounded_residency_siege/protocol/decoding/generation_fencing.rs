use super::{fields, number};
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
    BoundedResidencyGenerationFencingObservation,
};

pub(super) fn parse(
    lines: &[String],
) -> Result<BoundedResidencyGenerationFencingObservation, String> {
    Ok(BoundedResidencyGenerationFencingObservation {
        read: parse_case(lines, "BOUNDED_RESIDENCY_GENERATION_READ ")?,
        dirty: parse_case(lines, "BOUNDED_RESIDENCY_GENERATION_DIRTY ")?,
        writeback: parse_case(lines, "BOUNDED_RESIDENCY_GENERATION_WRITEBACK ")?,
    })
}

fn parse_case(
    lines: &[String],
    marker: &str,
) -> Result<BoundedResidencyGenerationFenceCase, String> {
    let value = fields(lines, marker, 18)?;
    Ok(BoundedResidencyGenerationFenceCase {
        current_generation: number(value[1], "generation-fence current generation")?,
        stale_generation: number(value[2], "generation-fence stale generation")?,
        denial: parse_denial(value[3])?,
        effects: BoundedResidencyGenerationFenceEffects {
            allocation_admissions: number(value[4], "generation-fence allocation admissions")?,
            allocation_releases: number(value[5], "generation-fence allocation releases")?,
            allocation_other: number(value[6], "generation-fence other allocation events")?,
            residency_hits: number(value[7], "generation-fence residency hits")?,
            residency_faults: number(value[8], "generation-fence residency faults")?,
            source_loads: number(value[9], "generation-fence source loads")?,
            dirty_transitions: number(value[10], "generation-fence dirty transitions")?,
            writeback_attempts: number(value[11], "generation-fence writeback attempts")?,
            work_declarations: number(value[12], "generation-fence work declarations")?,
            signal_requests: number(value[13], "generation-fence Signal requests")?,
            scheduler_admissions: number(value[14], "generation-fence scheduler admissions")?,
            media_attempts: number(value[15], "generation-fence media attempts")?,
        },
        mutation_invocations: number(value[16], "generation-fence mutation invocations")?,
        cleanup: parse_cleanup(value[17])?,
    })
}

fn parse_denial(encoded: &str) -> Result<BoundedResidencyGenerationDenial, String> {
    match encoded {
        "stale-generation" => Ok(BoundedResidencyGenerationDenial::StaleGeneration),
        "stale-or-foreign-frame" => Ok(BoundedResidencyGenerationDenial::StaleOrForeignFrame),
        _ => Err(format!(
            "unknown bounded-residency generation denial `{encoded}`"
        )),
    }
}

fn parse_cleanup(encoded: &str) -> Result<BoundedResidencyGenerationCleanup, String> {
    match encoded {
        "none" => Ok(BoundedResidencyGenerationCleanup::None),
        "lease-released" => Ok(BoundedResidencyGenerationCleanup::LeaseReleased),
        "dirty-returned" => Ok(BoundedResidencyGenerationCleanup::DirtyReturned),
        _ => Err(format!(
            "unknown bounded-residency generation cleanup `{encoded}`"
        )),
    }
}

#[cfg(test)]
mod tests;
