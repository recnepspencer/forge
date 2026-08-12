//! Installed aftermath postcondition field.

use worth_query_declaration::facade::application_aftermath::DeclaredAftermathPostcondition;

/// Postcondition retained as a field of an installed mechanism contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstalledAftermathPostcondition {
    ExactPriorTruth,
    InvariantRestored { invariant: String },
    BusinessPostcondition { identity: String },
}

impl InstalledAftermathPostcondition {
    pub(crate) fn from_declared(declared: &DeclaredAftermathPostcondition) -> Self {
        match declared {
            DeclaredAftermathPostcondition::ExactPriorTruth => Self::ExactPriorTruth,
            DeclaredAftermathPostcondition::InvariantRestored { invariant } => {
                Self::InvariantRestored {
                    invariant: invariant.clone(),
                }
            }
            DeclaredAftermathPostcondition::BusinessPostcondition { identity } => {
                Self::BusinessPostcondition {
                    identity: identity.clone(),
                }
            }
        }
    }
}
