use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanDeclarationReceipt,
    PlanarBooleanOperandPairConstructionReceipt,
};

pub(super) fn request_identity(
    pair: &BuiltBooleanOperandPairRecipe,
    construction: &PlanarBooleanOperandPairConstructionReceipt,
    declaration: Option<&PlanarBooleanDeclarationReceipt>,
) -> String {
    let declaration_boundary = declaration
        .map(|receipt| format!("declaration:{}", receipt.query_declaration_digest()))
        .unwrap_or_else(|| "declaration:none".to_string());
    let readiness_boundary = declaration
        .map(|receipt| format!("readiness:{}", receipt.readiness_basis_digest()))
        .unwrap_or_else(|| "readiness:none".to_string());

    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-reduction-request".to_string(),
            format!("recipe:{}", pair.recipe().query_key()),
            format!(
                "catalog-declaration:{}",
                pair.declaration().query_declaration_digest()
            ),
            format!("catalog-support:{}", pair.support().query_support_digest()),
            format!("pair:{}", pair.operand_pair_identity()),
            format!("construction:{}", construction.construction_digest()),
            declaration_boundary,
            readiness_boundary,
            format!(
                "left:{}",
                pair.left()
                    .workload()
                    .response()
                    .identity()
                    .receipt_identity()
            ),
            format!(
                "right:{}",
                pair.right()
                    .workload()
                    .response()
                    .identity()
                    .receipt_identity()
            ),
        ],
    )
}
