pub(super) fn terminal_cleanup_complete(
    client_closed: bool,
    readiness_closed: bool,
    census: &super::super::UiNativeResourceCensus,
) -> bool {
    client_closed && readiness_closed && census.is_zero()
}
