fn mutate_sibling(
    mounted: &mut crate::mounting::WorthUiMountedSessionState,
) {
    mounted.identity.advance_frame();
}
