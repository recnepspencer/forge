use crate::workload_composition::{
    BuiltBooleanOperandPairRecipe, BuiltWorkloadCatalogRecipe, PlanarBooleanDeclarationReceipt,
    PlanarBooleanOperandPairConstructionReceipt, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

use super::error::PlanarBooleanCommonPlaneReductionRequestError;
use super::identity::request_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneReductionRequest {
    pair: BuiltBooleanOperandPairRecipe,
    request_identity: String,
    construction_receipt: PlanarBooleanOperandPairConstructionReceipt,
    declaration_receipt: Option<PlanarBooleanDeclarationReceipt>,
}

impl PlanarBooleanCommonPlaneReductionRequest {
    pub fn from_operand_pair_recipe(
        pair: BuiltBooleanOperandPairRecipe,
    ) -> Result<Self, PlanarBooleanCommonPlaneReductionRequestError> {
        let construction = pair.construction_receipt();
        Self::from_built_pair_construction_and_declaration(pair, construction, None)
    }

    pub fn from_declaration_receipt_and_operand_pair(
        declaration_receipt: PlanarBooleanDeclarationReceipt,
        pair: BuiltBooleanOperandPairRecipe,
    ) -> Result<Self, PlanarBooleanCommonPlaneReductionRequestError> {
        if declaration_receipt.operand_pair_identity().as_str() != pair.operand_pair_identity() {
            return Err(
                PlanarBooleanCommonPlaneReductionRequestError::DeclarationOperandPairIdentityMismatch {
                    expected_operand_pair_identity: pair.operand_pair_identity().to_string(),
                    actual_operand_pair_identity: declaration_receipt
                        .operand_pair_identity()
                        .as_str()
                        .to_string(),
                },
            );
        }

        let construction = pair.construction_receipt();
        Self::from_built_pair_construction_and_declaration(
            pair,
            construction,
            Some(declaration_receipt),
        )
    }

    pub fn from_built_pair_and_construction(
        pair: BuiltBooleanOperandPairRecipe,
        construction: PlanarBooleanOperandPairConstructionReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneReductionRequestError> {
        Self::from_built_pair_construction_and_declaration(pair, construction, None)
    }

    fn from_built_pair_construction_and_declaration(
        pair: BuiltBooleanOperandPairRecipe,
        construction: PlanarBooleanOperandPairConstructionReceipt,
        declaration_receipt: Option<PlanarBooleanDeclarationReceipt>,
    ) -> Result<Self, PlanarBooleanCommonPlaneReductionRequestError> {
        if pair.operand_pair_identity() != construction.operand_pair_identity() {
            return Err(
                PlanarBooleanCommonPlaneReductionRequestError::OperandPairIdentityMismatch {
                    expected_operand_pair_identity: pair.operand_pair_identity().to_string(),
                    actual_operand_pair_identity: construction.operand_pair_identity().to_string(),
                },
            );
        }

        Ok(Self {
            request_identity: request_identity(&pair, &construction, declaration_receipt.as_ref()),
            pair,
            construction_receipt: construction,
            declaration_receipt,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.pair.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.pair.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.pair.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn construction_receipt(&self) -> &PlanarBooleanOperandPairConstructionReceipt {
        &self.construction_receipt
    }

    pub fn declaration_receipt(&self) -> Option<&PlanarBooleanDeclarationReceipt> {
        self.declaration_receipt.as_ref()
    }

    pub fn left(&self) -> &BuiltWorkloadCatalogRecipe {
        self.pair.left()
    }

    pub fn right(&self) -> &BuiltWorkloadCatalogRecipe {
        self.pair.right()
    }

    pub fn operand_pair_recipe(&self) -> &BuiltBooleanOperandPairRecipe {
        &self.pair
    }
}
