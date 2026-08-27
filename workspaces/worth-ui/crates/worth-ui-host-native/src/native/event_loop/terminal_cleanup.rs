pub(super) fn terminal_cleanup_complete(
    client_closed: bool,
    client_resources_complete: bool,
    readiness_closed: bool,
    census: &super::super::UiNativeResourceCensus,
) -> bool {
    client_closed && client_resources_complete && readiness_closed && census.is_zero()
}
