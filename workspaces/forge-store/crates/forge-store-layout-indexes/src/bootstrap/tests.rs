#[test]
fn bootstrap_path_is_fixed_to_store_magic_and_version() {
    let fixed = super::S8BootstrapOnlyAccessPath::s8_fixed();

    assert_eq!(fixed.magic(), forge_store_physical_format::PhysicalFormatMagic::s1_store());
    assert_eq!(
        fixed.physical_format_version(),
        forge_store_physical_format::PhysicalFormatVersion::s1_initial()
    );
}
