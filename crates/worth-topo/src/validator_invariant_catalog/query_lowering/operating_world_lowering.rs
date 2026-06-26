use forge_query::facade::ForgeQueryGraphObligationOperatingWorldSelector;

pub(super) fn authoritative_operating_world_selector(
) -> ForgeQueryGraphObligationOperatingWorldSelector {
    ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority()
}
