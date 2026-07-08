use super::super::{
    chunk_plan::ChunkMigrationReadInterlockPlan, FutureBlobMigrationNonClaimReport,
    MovablePhysicalRef, PhysicalChunkStabilityPlaceholder, TierMovementReadInterlockPlan,
    TierMovementStabilityDenial,
};

pub fn admit_chunk_migration_interlock(
    placeholder: PhysicalChunkStabilityPlaceholder,
) -> Result<ChunkMigrationReadInterlockPlan, TierMovementStabilityDenial> {
    let movable = MovablePhysicalRef::future_chunk_from_placeholder(placeholder);
    let tier_plan = TierMovementReadInterlockPlan::admit(movable)?;
    let counters = placeholder.counters().with_stability_admission();
    Ok(ChunkMigrationReadInterlockPlan::from_admitted_transition(
        placeholder,
        tier_plan,
        FutureBlobMigrationNonClaimReport::s5_stability_only(),
        counters,
    ))
}