use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::runtime::WorthUiSemanticSliceId;

use super::{
    WorthUiExpressionArity, WorthUiExpressionCostPosture, WorthUiExpressionDependencyContract,
    WorthUiExpressionDiagnosticsPosture, WorthUiExpressionInputKind, WorthUiExpressionOperatorId,
    WorthUiExpressionOutputKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExpressionOperatorDescriptor {
    operator_id: WorthUiExpressionOperatorId,
    input_kinds: Vec<WorthUiExpressionInputKind>,
    output_kind: WorthUiExpressionOutputKind,
    arity: WorthUiExpressionArity,
    dependency_contract: WorthUiExpressionDependencyContract,
    cost_posture: WorthUiExpressionCostPosture,
    diagnostics_posture: WorthUiExpressionDiagnosticsPosture,
    semantic_slice: WorthUiSemanticSliceId,
    bounded: bool,
    pure: bool,
    descriptor_digest: u64,
}

impl WorthUiExpressionOperatorDescriptor {
    pub(crate) fn new(
        operator_id: WorthUiExpressionOperatorId,
        input_kinds: Vec<WorthUiExpressionInputKind>,
        output_kind: WorthUiExpressionOutputKind,
        arity: WorthUiExpressionArity,
        dependency_contract: WorthUiExpressionDependencyContract,
        cost_posture: WorthUiExpressionCostPosture,
        diagnostics_posture: WorthUiExpressionDiagnosticsPosture,
        semantic_slice: WorthUiSemanticSliceId,
    ) -> Self {
        let digest = descriptor_digest(operator_id, output_kind, arity, dependency_contract);
        Self {
            operator_id,
            input_kinds,
            output_kind,
            arity,
            dependency_contract,
            cost_posture,
            diagnostics_posture,
            semantic_slice,
            bounded: true,
            pure: true,
            descriptor_digest: digest,
        }
    }

    pub fn operator_id(&self) -> WorthUiExpressionOperatorId {
        self.operator_id
    }

    pub fn input_kinds(&self) -> &[WorthUiExpressionInputKind] {
        &self.input_kinds
    }

    pub fn output_kind(&self) -> WorthUiExpressionOutputKind {
        self.output_kind
    }

    pub fn arity(&self) -> WorthUiExpressionArity {
        self.arity
    }

    pub fn dependency_contract(&self) -> WorthUiExpressionDependencyContract {
        self.dependency_contract
    }

    pub fn cost_posture(&self) -> WorthUiExpressionCostPosture {
        self.cost_posture
    }

    pub fn diagnostics_posture(&self) -> WorthUiExpressionDiagnosticsPosture {
        self.diagnostics_posture
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        self.semantic_slice
    }

    pub fn is_bounded(&self) -> bool {
        self.bounded
    }

    pub fn is_pure(&self) -> bool {
        self.pure
    }

    pub fn descriptor_digest(&self) -> u64 {
        self.descriptor_digest
    }
}

fn descriptor_digest(
    operator_id: WorthUiExpressionOperatorId,
    output_kind: WorthUiExpressionOutputKind,
    arity: WorthUiExpressionArity,
    dependency_contract: WorthUiExpressionDependencyContract,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    operator_id.as_str().hash(&mut hasher);
    output_kind.token().hash(&mut hasher);
    arity.token().hash(&mut hasher);
    dependency_contract.token().hash(&mut hasher);
    "bounded".hash(&mut hasher);
    "pure".hash(&mut hasher);
    hasher.finish()
}
