use worth_store::physical_runtime::{
    PhysicalWorkIdentity, PhysicalWorkOperationFamily, PhysicalWorkSignalFamily,
    ServingPhysicalRuntime,
};

pub(super) fn require_exact(
    serving: &ServingPhysicalRuntime,
    work: PhysicalWorkIdentity,
    family: PhysicalWorkSignalFamily,
    aspect_key: &str,
) -> Result<(), String> {
    let record = serving
        .physical_work_observer()
        .causal()
        .records()
        .iter()
        .copied()
        .find(|record| record.identity() == work)
        .ok_or_else(|| "speculative work omitted its causal settlement record".to_owned())?;
    if record.operation() != expected_operation(family) || record.signal_family() != family {
        return Err(format!(
            "speculative work selected the wrong operation or Signal family: {record:?}"
        ));
    }
    let binding = serving
        .physical_signal_aspect_binding_observations()
        .into_vec()
        .into_iter()
        .find(|binding| binding.digest() == record.signal_binding())
        .ok_or_else(|| "speculative work selected an unobserved Signal binding".to_owned())?;
    if !binding.families().contains(family)
        || binding.identity().aspect_key().as_str() != aspect_key
    {
        return Err(format!(
            "speculative work selected the wrong Foundational basis: {binding:?}"
        ));
    }
    Ok(())
}

fn expected_operation(family: PhysicalWorkSignalFamily) -> PhysicalWorkOperationFamily {
    match family {
        PhysicalWorkSignalFamily::ReadFault => PhysicalWorkOperationFamily::ArtifactRangeRead,
        PhysicalWorkSignalFamily::ExactWriteback => PhysicalWorkOperationFamily::ArtifactRangeWrite,
        _ => unreachable!("bounded speculation uses only read-fault and exact-writeback families"),
    }
}
