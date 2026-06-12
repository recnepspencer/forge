/// Diagnostic marker for native support inferred from the current host.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AmbientHostCheck {
    kind: AmbientHostCheckKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum AmbientHostCheckKind {
    CurrentHost,
}

impl AmbientHostCheck {
    pub fn current_host_for_diagnostics() -> Self {
        Self {
            kind: AmbientHostCheckKind::CurrentHost,
        }
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self.kind {
            AmbientHostCheckKind::CurrentHost => "current_host",
        }
    }
}
