/// Diagnostic marker for native descriptors that try to redefine shell meaning.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeShellAuthorityClaim {
    kind: NativeShellAuthorityClaimKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NativeShellAuthorityClaimKind {
    RedefinesShellSemantics,
}

impl NativeShellAuthorityClaim {
    pub fn redefines_shell_semantics_for_diagnostics() -> Self {
        Self {
            kind: NativeShellAuthorityClaimKind::RedefinesShellSemantics,
        }
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self.kind {
            NativeShellAuthorityClaimKind::RedefinesShellSemantics => "redefines_shell_semantics",
        }
    }
}
