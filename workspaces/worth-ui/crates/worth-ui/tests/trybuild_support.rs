pub fn new_test_cases() -> trybuild::TestCases {
    static SET_RUSTFLAGS: std::sync::Once = std::sync::Once::new();

    SET_RUSTFLAGS.call_once(|| {
        std::env::set_var("RUSTFLAGS", "-Awarnings");
    });

    trybuild::TestCases::new()
}

