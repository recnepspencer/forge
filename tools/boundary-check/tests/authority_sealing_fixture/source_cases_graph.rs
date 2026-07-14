//! Specimens for authoritative compiled-source inventory (lib path, cfg, #[path]).

/// Harmless decoy root that must not be the only scanned surface.
pub fn decoy_lib_rs() -> &'static str {
    r#"
// Decoy conventional root — real library target is elsewhere.
pub fn seed() {}
"#
}

/// Real library target with AuthorityMarker ceremony.
pub fn hostile_ordinary_api_lib() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
"#
}

/// Legal concrete ceremony at a custom library path.
pub fn legal_ordinary_api_lib() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}

pub fn admit(_authority: AuthorityWitnessPlaceholder<EntryAdmission>) {}
"#
}

/// Cfg-exclusive modules: safe body first, hostile second (declaration order).
pub fn hostile_cfg_modules_safe_then_hostile() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[cfg(any())]
pub mod gate {
    pub fn admit() {}
}

#[cfg(all())]
pub mod gate {
    use super::AuthorityMarker;
    pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
}
"#
}

/// Cfg-exclusive modules: hostile body first, safe second.
pub fn hostile_cfg_modules_hostile_then_safe() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[cfg(all())]
pub mod gate {
    use super::AuthorityMarker;
    pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
}

#[cfg(any())]
pub mod gate {
    pub fn admit() {}
}
"#
}

/// Root that loads an out-of-line module via #[path] to a hostile file.
pub fn path_attr_root() -> &'static str {
    r#"
#[path = "real_gate.rs"]
pub mod gate;
"#
}

/// Conventional decoy next to a #[path] target (must not be the inspected body alone).
pub fn path_attr_decoy_gate_rs() -> &'static str {
    r#"
pub fn admit() {}
"#
}

/// Real #[path] target with AuthorityMarker ceremony.
pub fn path_attr_real_gate_rs() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
"#
}

/// Nested inline module with direct #[path] (virtual-directory resolution).
pub fn nested_inline_path_attr_root() -> &'static str {
    r#"
pub mod outer {
    #[path = "hostile_gate.rs"]
    pub mod gate;
}
"#
}

/// Nested out-of-line parent with #[path] child.
pub fn nested_outline_path_parent_rs() -> &'static str {
    r#"
#[path = "hostile_gate.rs"]
pub mod gate;
"#
}

pub fn nested_outline_path_root() -> &'static str {
    r#"
pub mod outer;
"#
}

/// Root using cfg_attr to select hostile path.
pub fn cfg_attr_path_root() -> &'static str {
    r#"
#[cfg_attr(all(), path = "hostile_gate.rs")]
pub mod gate;
"#
}

/// Nested cfg_attr path under inline module.
pub fn nested_cfg_attr_path_root() -> &'static str {
    r#"
pub mod outer {
    #[cfg_attr(all(), path = "hostile_gate.rs")]
    pub mod gate;
}
"#
}

/// Hostile path body (shared by nested path tests).
pub fn hostile_gate_path_body() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
"#
}

/// Conventional decoy for path tests.
pub fn safe_gate_decoy() -> &'static str {
    r#"
pub fn admit() {}
"#
}
