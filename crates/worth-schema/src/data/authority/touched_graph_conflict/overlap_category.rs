use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ConflictOverlapCategory {
    Entity,
    Relation,
    Aspect,
    Locality,
    Evidence,
    Validator,
    ReplayUndo,
    Transaction,
}

impl ConflictOverlapCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Relation => "relation",
            Self::Aspect => "aspect",
            Self::Locality => "locality",
            Self::Evidence => "evidence",
            Self::Validator => "validator",
            Self::ReplayUndo => "replay-undo",
            Self::Transaction => "transaction",
        }
    }
}
