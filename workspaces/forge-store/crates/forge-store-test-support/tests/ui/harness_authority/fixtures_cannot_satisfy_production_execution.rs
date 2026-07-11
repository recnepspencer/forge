use forge_store_test_support::production_facade;

fn main() {
    let fixtures = production_facade::s8_layout_access::s8_layout_fixtures();
    let _ = production_facade::s8_layout_access::execute_s8_layout_scenario(fixtures);
}
