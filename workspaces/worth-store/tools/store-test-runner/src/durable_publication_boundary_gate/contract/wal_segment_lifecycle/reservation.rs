use super::super::wal_source_syntax::ParsedRustSource;

pub(super) fn inspect(reservation: &str, member_planning: &str) -> Result<(), String> {
    inspect_reservation(reservation)?;
    inspect_member_planning(member_planning)
}

fn inspect_reservation(source: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(source, "WAL group reservation owner")?;
    let reserve = source.function("reserve_group")?;
    reserve.require_exact("call:plan_group", 2)?;
    reserve.require_exact("assign:in_flight=true", 2)?;
    reserve.require_at_least("path:CreateSegment", 2)?;
    reserve.require_in_order(&[
        "method:last_lsn_end",
        "call:plan_group",
        "method:release_after_no_effect",
        "method:segment_inventory_limit",
        "call:plan_group",
    ])
}

fn inspect_member_planning(source: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(source, "WAL group member-planning owner")?;
    let planning = source.function("plan_group")?;
    planning.require_exact("path:AppendExistingSegment", 1)?;
    planning.require_in_order(&[
        "method:next",
        "path:AppendExistingSegment",
        "call:plan_member",
        "method:mutation",
        "method:resulting_frontier",
        "method:last_lsn_end",
        "call:nonempty",
        "call:ReservedPhysicalWalGroupMembers",
    ])
}
