use super::{assert_contract_allowed, assert_contract_denied, value_row};

const NAMED_ALIAS_SOURCE: &str = r#"
mod named_owner {
    pub struct Named { value: u8 }
    pub fn issue_named() -> Named { Named { value: 1 } }
}
use named_owner as named_alias;
pub use named_alias::{issue_named, Named};
"#;
const NAMED_ALIAS_WITNESS: &str =
    "pub(crate) fn named() -> worth_proof::Named { worth_proof::issue_named() }";

const GLOB_ALIAS_SOURCE: &str = r#"
mod glob_owner {
    pub struct Globbed { value: u8 }
    pub fn issue_globbed() -> Globbed { Globbed { value: 2 } }
}
use glob_owner as glob_alias;
pub use glob_alias::*;
"#;
const GLOB_ALIAS_WITNESS: &str =
    "pub(crate) fn globbed() -> worth_proof::Globbed { worth_proof::issue_globbed() }";

const TYPE_ALIAS_SOURCE: &str = r#"
mod type_owner {
    pub struct Aliased { value: u8 }
    pub fn issue_aliased() -> Aliased { Aliased { value: 3 } }
}
use type_owner as type_alias_owner;
pub type PublicAlias = type_alias_owner::Aliased;
pub use type_owner::issue_aliased;
"#;
const TYPE_ALIAS_WITNESS: &str =
    "pub(crate) fn aliased() -> worth_proof::PublicAlias { worth_proof::issue_aliased() }";

const USE_CRATE_ALIAS_SOURCE: &str = r#"
mod owner {
    pub struct Named { value: u8 }
    pub struct Aliased { value: u8 }
    pub fn issue_named() -> Named { Named { value: 1 } }
    pub fn issue_aliased() -> Aliased { Aliased { value: 2 } }
}
use crate as root_alias;
pub use root_alias::owner::{issue_aliased, issue_named, Named};
pub type PublicAlias = root_alias::owner::Aliased;
"#;

const EXTERN_SELF_ALIAS_SOURCE: &str = r#"
mod owner {
    pub struct Named { value: u8 }
    pub struct Aliased { value: u8 }
    pub fn issue_named() -> Named { Named { value: 1 } }
    pub fn issue_aliased() -> Aliased { Aliased { value: 2 } }
}
extern crate self as root_alias;
pub use root_alias::owner::{issue_aliased, issue_named, Named};
pub type PublicAlias = root_alias::owner::Aliased;
"#;

const SELF_ALIAS_WITNESSES: &str = r#"
pub(crate) fn named() -> worth_proof::Named { worth_proof::issue_named() }
pub(crate) fn aliased() -> worth_proof::PublicAlias { worth_proof::issue_aliased() }
"#;

const EXTERNAL_EXTERN_ALIAS_SOURCE: &str = r#"
pub mod local { pub struct PhantomData { value: u8 } }
extern crate core as root_alias;
pub type PublicAlias = root_alias::marker::PhantomData<u8>;
"#;

const EXTERN_ALIAS_COLLISION_SOURCE: &str = r#"
pub mod owner {
    pub struct Sealed { value: u8 }
    pub fn issue() -> Sealed { Sealed { value: 1 } }
}
extern crate self as root_alias;
extern crate core as root_alias;
pub use root_alias::owner::{issue, Sealed};
"#;

const SELF_ALIAS_CYCLE_SOURCE: &str = r#"
pub mod owner {
    pub struct Sealed { value: u8 }
    pub fn issue() -> Sealed { Sealed { value: 1 } }
}
use second as first;
use first as second;
pub use first::owner::{issue, Sealed};
"#;

const SEALED_WITNESS: &str = "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}";

#[test]
fn named_reexport_resolves_through_transitive_public_glob() {
    assert_contract_allowed(
        "named-through-glob",
        r#"
mod owner {
    pub struct Sealed { value: u8 }
    pub fn issue() -> Sealed { Sealed { value: 1 } }
}
mod facade { pub use crate::owner::*; }
pub use facade::Sealed;
pub use owner::issue;
"#,
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        &value_row().replace(
            "definition_path = \"Sealed\"",
            "definition_path = \"owner::Sealed\"",
        ),
        "",
    );
}

#[test]
fn private_module_alias_resolves_named_reexport() {
    assert_contract_allowed(
        "private-module-alias-named",
        NAMED_ALIAS_SOURCE,
        NAMED_ALIAS_WITNESS,
        &exact_value_row("named_owner::Named", "named", "Named"),
        "",
    );
}

#[test]
fn private_module_alias_resolves_glob_reexport() {
    assert_contract_allowed(
        "private-module-alias-glob",
        GLOB_ALIAS_SOURCE,
        GLOB_ALIAS_WITNESS,
        &exact_value_row("glob_owner::Globbed", "globbed", "Globbed"),
        "",
    );
}

#[test]
fn private_module_alias_resolves_public_type_alias_rhs() {
    assert_contract_allowed(
        "private-module-alias-type-rhs",
        TYPE_ALIAS_SOURCE,
        TYPE_ALIAS_WITNESS,
        &exact_value_row("type_owner::Aliased", "aliased", "PublicAlias"),
        "",
    );
}

#[test]
fn use_crate_alias_resolves_named_and_type_alias_surfaces() {
    assert_contract_allowed(
        "use-crate-self-alias",
        USE_CRATE_ALIAS_SOURCE,
        SELF_ALIAS_WITNESSES,
        &self_alias_rows(),
        "",
    );
}

#[test]
fn extern_crate_self_alias_resolves_named_and_type_alias_surfaces() {
    assert_contract_allowed(
        "extern-crate-self-alias",
        EXTERN_SELF_ALIAS_SOURCE,
        SELF_ALIAS_WITNESSES,
        &self_alias_rows(),
        "",
    );
}

#[test]
fn external_extern_crate_alias_cannot_capture_a_local_definition() {
    assert_contract_denied(
        "external-extern-crate-alias",
        EXTERNAL_EXTERN_ALIAS_SOURCE,
        "pub(crate) fn phantom()->worth_proof::PublicAlias{core::marker::PhantomData}",
        &exact_value_row("local::PhantomData", "phantom", "PublicAlias"),
        "",
    );
}

#[test]
fn extern_crate_alias_collision_fails_closed() {
    assert_contract_denied(
        "extern-crate-alias-collision",
        EXTERN_ALIAS_COLLISION_SOURCE,
        SEALED_WITNESS,
        &exact_value_row("owner::Sealed", "sealed", "Sealed"),
        "",
    );
}

#[test]
fn self_alias_cycle_fails_closed() {
    assert_contract_denied(
        "self-alias-cycle",
        SELF_ALIAS_CYCLE_SOURCE,
        SEALED_WITNESS,
        &exact_value_row("owner::Sealed", "sealed", "Sealed"),
        "",
    );
}

#[test]
fn external_module_alias_cannot_capture_a_local_same_named_definition() {
    assert_contract_denied(
        "external-module-alias-collision",
        r#"
pub mod local { pub struct PhantomData { value: u8 } }
use core as alias;
pub type PublicAlias = alias::marker::PhantomData<u8>;
"#,
        "pub(crate) fn phantom()->worth_proof::PublicAlias{core::marker::PhantomData}",
        &exact_value_row("local::PhantomData", "phantom", "PublicAlias"),
        "",
    );
}

#[test]
fn cyclic_module_aliases_without_a_definition_fail_closed() {
    assert_contract_denied(
        "cyclic-module-alias-no-definition",
        r#"
mod first { use crate::second as alias; pub use alias::Sealed; }
mod second { use crate::first as alias; pub use alias::Sealed; }
pub use first::Sealed;
"#,
        "",
        &exact_value_row("first::Sealed", "sealed", "Sealed"),
        "",
    );
}

#[test]
fn unrelated_glob_cannot_capture_a_same_named_definition() {
    assert_contract_denied(
        "unrelated-glob-collision",
        r#"
mod hidden { pub struct Sealed { value: u8 } }
mod owner { pub struct Sealed { pub value: u8 } }
mod facade { pub use crate::owner::*; }
pub use facade::Sealed;
"#,
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::Sealed{value:1}}",
        &value_row().replace(
            "definition_path = \"Sealed\"",
            "definition_path = \"hidden::Sealed\"",
        ),
        "",
    );
}

#[test]
fn cyclic_globs_without_the_named_definition_fail_closed() {
    assert_contract_denied(
        "cyclic-glob-no-definition",
        r#"
mod first { pub use crate::second::*; }
mod second { pub use crate::first::*; }
"#,
        "",
        &value_row().replace(
            "definition_path = \"Sealed\"",
            "definition_path = \"first::Sealed\"",
        ),
        "",
    );
}

fn exact_value_row(definition: &str, function: &str, public_name: &str) -> String {
    value_row()
        .replace(
            "definition_path = \"Sealed\"",
            &format!("definition_path = \"{definition}\""),
        )
        .replace(
            "function = \"sealed\"",
            &format!("function = \"{function}\""),
        )
        .replace(
            "public_type_path = \"::worth_proof::Sealed\"",
            &format!("public_type_path = \"::worth_proof::{public_name}\""),
        )
}

fn self_alias_rows() -> String {
    [
        exact_value_row("owner::Named", "named", "Named"),
        exact_value_row("owner::Aliased", "aliased", "PublicAlias"),
    ]
    .concat()
}
