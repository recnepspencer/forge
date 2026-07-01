use crate::declaration::UiDeclarationIdentity;
use crate::graph::UiRepeatedInstanceBasisDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphInstantiationDenial {
    RuntimeBasisTargetsUnknownDeclaration {
        declaration_identity: UiDeclarationIdentity,
    },
    DuplicateRuntimeBasisAdmission {
        declaration_identity: UiDeclarationIdentity,
    },
    ContradictoryRuntimeBasisAdmission {
        declaration_identity: UiDeclarationIdentity,
    },
    RuntimeBasisDenied {
        declaration_identity: UiDeclarationIdentity,
        denial: UiRepeatedInstanceBasisDenial,
    },
}
