mod compile_fail_contracts {
    include!("../public_facade_contracts/compile_fail_contracts.rs");
}

mod public_api {
    use crate as topology;

    include!("../public_facade_contracts/contracts/public_api.rs");
}
