#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEvolutionKind {
    Unchanged,
    Additive,
    Widening,
    Narrowing,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectEvolutionVerdict {
    kind: AspectEvolutionKind,
    reason: &'static str,
}

impl AspectEvolutionVerdict {
    pub fn new(kind: AspectEvolutionKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub fn kind(&self) -> AspectEvolutionKind {
        self.kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
