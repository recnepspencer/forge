use super::{ConflictOverlapCategory, ConflictParticipantAuthority};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConflictRoutingVocabularyError {
    EmptyDigest(&'static str),
    EmptyParticipantSet(ConflictOverlapCategory),
    MissingAspectClass,
    MissingLocalityIdentity(ConflictOverlapCategory),
    MissingPriorProof(ConflictOverlapCategory),
    WrongInternalAuthorityType {
        required: &'static str,
        found: &'static str,
    },
    WrongParticipantAuthority {
        category: ConflictOverlapCategory,
        expected: ConflictParticipantAuthority,
        found: ConflictParticipantAuthority,
    },
    WrongPriorProof(&'static str),
}

impl ConflictRoutingVocabularyError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::EmptyDigest(label) => format!("{label} requires a non-empty digest"),
            Self::EmptyParticipantSet(category) => {
                format!(
                    "{} overlap requires at least one participant",
                    category.as_str()
                )
            }
            Self::MissingAspectClass => "aspect overlap requires an aspect class".to_string(),
            Self::MissingLocalityIdentity(category) => format!(
                "{} overlap requires a typed locality identity",
                category.as_str()
            ),
            Self::MissingPriorProof(category) => format!(
                "{} overlap requires typed replay/undo or transaction proof",
                category.as_str()
            ),
            Self::WrongInternalAuthorityType { required, found } => {
                format!("internal conflict authority requires {required}, found {found}")
            }
            Self::WrongParticipantAuthority {
                category,
                expected,
                found,
            } => format!(
                "{} overlap requires {:?} participants, found {:?}",
                category.as_str(),
                expected,
                found
            ),
            Self::WrongPriorProof(reason) => reason.to_string(),
        }
    }
}
