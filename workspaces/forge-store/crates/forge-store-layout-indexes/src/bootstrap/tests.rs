#[test]
fn bootstrap_path_is_fixed_to_store_magic_and_version() {
    let fixed = super::BootstrapOnlyAccessPath::fixed_bootstrap_access_path();

    assert_eq!(
        fixed.magic(),
        forge_store_physical_format::PhysicalFormatMagic::store_format_magic()
    );
    assert_eq!(
        fixed.physical_format_version(),
        forge_store_physical_format::PhysicalFormatVersion::initial_format_version()
    );
}
