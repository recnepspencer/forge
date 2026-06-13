use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::built_recipe::{
    BuiltBooleanCleanFailCatalogRecipe, BuiltBooleanDeniedCatalogRecipe,
    BuiltBooleanOperandPairRecipe,
};
use super::catalog::WorkloadCatalog;
use super::error::WorkloadCatalogError;
use super::recipe_kind::{WorkloadCatalogRecipeKind, WorkloadCatalogSupportPosture};
use super::support_receipt::{
    WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportDecision,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogBooleanOperandPairRecipe {
    kind: WorkloadCatalogRecipeKind,
    declaration: String,
}

impl WorkloadCatalogBooleanOperandPairRecipe {
    pub(crate) fn new(kind: WorkloadCatalogRecipeKind) -> Self {
        Self {
            kind,
            declaration: kind.default_declaration().to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn inspect_support(&self) -> Result<WorkloadCatalogSupportReceipt, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision()?)
    }

    pub fn build(self) -> Result<BuiltBooleanOperandPairRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support = WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision()?)?;
        require_admitted_pair_support(&support)?;

        match self.kind {
            WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair
            | WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => {
                return Err(WorkloadCatalogError::UnsupportedRecipe {
                    recipe: self.kind,
                    reason: self.product_lane_reason(),
                });
            }
            _ => {}
        }

        let left = self.left_member_recipe().build()?;
        let right = self.right_member_recipe().build()?;
        let operand_pair_identity =
            operand_pair_identity(self.kind, left.declaration(), right.declaration());

        Ok(BuiltBooleanOperandPairRecipe::new(
            self.kind,
            declaration,
            support,
            operand_pair_identity,
            left,
            right,
        ))
    }

    pub fn build_clean_fail(
        self,
    ) -> Result<BuiltBooleanCleanFailCatalogRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support = WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision()?)?;
        require_admitted_pair_support(&support)?;

        if self.kind != WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe: self.kind,
                reason:
                    "only dirty boolean operand-pair recipes can build through the clean-fail lane"
                        .to_string(),
            });
        }

        let dirty_operand = self.left_member_recipe().build_clean_fail()?;
        let clean_operand = self.right_member_recipe().build()?;
        let operand_pair_identity = operand_pair_identity(
            self.kind,
            dirty_operand.declaration(),
            clean_operand.declaration(),
        );

        Ok(BuiltBooleanCleanFailCatalogRecipe::new(
            self.kind,
            declaration,
            support,
            operand_pair_identity,
            dirty_operand,
            clean_operand,
        ))
    }

    pub fn build_denial(self) -> Result<BuiltBooleanDeniedCatalogRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support = WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision()?)?;
        require_admitted_pair_support(&support)?;

        if self.kind != WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe: self.kind,
                reason:
                    "only open or unbounded boolean operand-pair recipes can build through the denial lane"
                        .to_string(),
            });
        }

        let left = self.left_member_recipe().build()?;
        let right = self.right_member_recipe().build()?;
        let operand_pair_identity =
            operand_pair_identity(self.kind, left.declaration(), right.declaration());

        Ok(BuiltBooleanDeniedCatalogRecipe::new(
            self.kind,
            declaration,
            support,
            operand_pair_identity,
            "open or unbounded operand members remain real workload evidence, but milestone 7.0 only admits this pair on the denial lane until bounded planar boolean execution hardens".to_string(),
            left,
            right,
        ))
    }

    pub fn kind(&self) -> WorkloadCatalogRecipeKind {
        self.kind
    }

    fn declaration_receipt(
        &self,
    ) -> Result<WorkloadCatalogDeclarationReceipt, WorkloadCatalogError> {
        if self.declaration.trim().is_empty() {
            return Err(WorkloadCatalogError::MissingDeclaration);
        }
        WorkloadCatalogDeclarationReceipt::new(self.kind, &self.declaration)
    }

    fn support_decision(&self) -> Result<WorkloadCatalogSupportDecision, WorkloadCatalogError> {
        Ok(match self.kind {
            WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair
            | WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair
            | WorkloadCatalogRecipeKind::BooleanThinFeaturePair
            | WorkloadCatalogRecipeKind::BooleanHighValenceContactPair => {
                ensure_member_support(self.left_member_recipe())?;
                ensure_member_support(self.right_member_recipe())?;
                WorkloadCatalogSupportDecision::admitted(format!(
                    "{} is admitted as a real workload-backed boolean operand pair",
                    self.kind.human_name()
                ))
            }
            WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair => {
                ensure_clean_fail_member_support(self.left_member_recipe())?;
                ensure_member_support(self.right_member_recipe())?;
                WorkloadCatalogSupportDecision::admitted(
                    "dirty boolean operand pair is admitted only on the topology-backed clean-fail lane; it must not fabricate an admitted workload pair".to_string(),
                )
            }
            WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => {
                ensure_member_support(self.left_member_recipe())?;
                ensure_member_support(self.right_member_recipe())?;
                WorkloadCatalogSupportDecision::admitted(
                    "open or unbounded boolean operand pair is admitted only on the denial lane; open members remain real workloads but cannot claim an admitted boolean operand pair".to_string(),
                )
            }
            _ => {
                return Err(WorkloadCatalogError::UnsupportedRecipe {
                    recipe: self.kind,
                    reason: "recipe kind is not a boolean operand-pair family".to_string(),
                })
            }
        })
    }

    fn left_member_recipe(&self) -> super::catalog::WorkloadCatalogRecipe {
        match self.kind {
            WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair => {
                WorkloadCatalog::single_face_loop()
            }
            WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair => {
                WorkloadCatalog::coplanar_overlap_storm().with_retained_replay_artifacts()
            }
            WorkloadCatalogRecipeKind::BooleanThinFeaturePair => {
                WorkloadCatalog::thin_feature_wall()
            }
            WorkloadCatalogRecipeKind::BooleanHighValenceContactPair => {
                WorkloadCatalog::high_valence_vertex()
            }
            WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair => {
                WorkloadCatalog::dirty_self_intersecting_loop()
            }
            WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => {
                WorkloadCatalog::open_sheet()
            }
            _ => unreachable!("non-pair recipe kind cannot select a left operand member"),
        }
        .declared(format!("{} left operand", self.declaration))
    }

    fn right_member_recipe(&self) -> super::catalog::WorkloadCatalogRecipe {
        match self.kind {
            WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair => {
                WorkloadCatalog::single_face_loop()
            }
            WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair => {
                WorkloadCatalog::coplanar_overlap_storm().with_retained_replay_artifacts()
            }
            WorkloadCatalogRecipeKind::BooleanThinFeaturePair => {
                WorkloadCatalog::single_face_loop()
            }
            WorkloadCatalogRecipeKind::BooleanHighValenceContactPair => {
                WorkloadCatalog::single_face_loop()
            }
            WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair => {
                WorkloadCatalog::single_face_loop()
            }
            WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => {
                WorkloadCatalog::single_face_loop()
            }
            _ => unreachable!("non-pair recipe kind cannot select a right operand member"),
        }
        .declared(format!("{} right operand", self.declaration))
    }

    fn product_lane_reason(&self) -> String {
        match self.kind {
            WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair => {
                "dirty boolean operand pair is only admitted through the topology-backed clean-fail lane".to_string()
            }
            WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => {
                "open or unbounded boolean operand pair is only admitted through the denial lane".to_string()
            }
            _ => format!("{} must build through the admitted workload pair lane", self.kind.human_name()),
        }
    }
}

fn ensure_member_support(
    recipe: super::catalog::WorkloadCatalogRecipe,
) -> Result<(), WorkloadCatalogError> {
    let support = recipe.inspect_support()?;
    if support.posture() == WorkloadCatalogSupportPosture::Admitted {
        Ok(())
    } else {
        Err(WorkloadCatalogError::UnsupportedRecipe {
            recipe: support.recipe(),
            reason: support.human_reason().to_string(),
        })
    }
}

fn ensure_clean_fail_member_support(
    recipe: super::catalog::WorkloadCatalogRecipe,
) -> Result<(), WorkloadCatalogError> {
    recipe.inspect_clean_fail_support().map(|_| ())
}

fn require_admitted_pair_support(
    support: &WorkloadCatalogSupportReceipt,
) -> Result<(), WorkloadCatalogError> {
    if support.posture() == WorkloadCatalogSupportPosture::Admitted {
        Ok(())
    } else {
        Err(WorkloadCatalogError::UnsupportedRecipe {
            recipe: support.recipe(),
            reason: support.human_reason().to_string(),
        })
    }
}

fn operand_pair_identity(
    recipe: WorkloadCatalogRecipeKind,
    left: &WorkloadCatalogDeclarationReceipt,
    right: &WorkloadCatalogDeclarationReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "workload-catalog-boolean-operand-pair".to_string(),
            format!("recipe:{}", recipe.query_key()),
            format!("left:{}", left.query_declaration_digest()),
            format!("right:{}", right.query_declaration_digest()),
        ],
    )
}
