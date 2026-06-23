use super::super::inventory_lane::WorthGraphReadAccessInventoryRow;
use super::super::phase_six_closeout::{
    WorthGraphReadAccessInventoryRowContext, WorthGraphReadAccessInventoryRowIdentity,
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};
use super::read_family_target::WorthGraphReadReadFamilyTarget;
use super::requirement_vocabulary::WorthGraphReadRequirementVocabulary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCandidate {
    inventory_row_context: WorthGraphReadAccessInventoryRowContext,
    read_family_target: WorthGraphReadReadFamilyTarget,
    touched_authority_input: String,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
    milestone_seven_lowering_target: String,
}

impl WorthGraphReadDeclarationCandidate {
    pub fn for_inventory_row(
        row: &WorthGraphReadAccessInventoryRow,
    ) -> WorthGraphReadDeclarationCandidateBuilder {
        WorthGraphReadDeclarationCandidateBuilder {
            inventory_row_context: Some(WorthGraphReadAccessInventoryRowContext::from_row(row)),
            ..WorthGraphReadDeclarationCandidateBuilder::default()
        }
    }

    pub fn inventory_row_identity(&self) -> &WorthGraphReadAccessInventoryRowIdentity {
        self.inventory_row_context.identity()
    }

    pub fn inventory_row_context(&self) -> &WorthGraphReadAccessInventoryRowContext {
        &self.inventory_row_context
    }

    pub const fn read_family_target(&self) -> WorthGraphReadReadFamilyTarget {
        self.read_family_target
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn requirement_vocabulary(&self) -> &WorthGraphReadRequirementVocabulary {
        &self.requirement_vocabulary
    }

    pub fn milestone_seven_lowering_target(&self) -> &str {
        &self.milestone_seven_lowering_target
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCandidateBuilder {
    inventory_row_context: Option<WorthGraphReadAccessInventoryRowContext>,
    read_family_target: Option<WorthGraphReadReadFamilyTarget>,
    touched_authority_input: Option<String>,
    requirement_vocabulary: Option<WorthGraphReadRequirementVocabulary>,
    milestone_seven_lowering_target: Option<String>,
}

impl WorthGraphReadDeclarationCandidateBuilder {
    pub const fn read_family_target(
        mut self,
        read_family_target: WorthGraphReadReadFamilyTarget,
    ) -> Self {
        self.read_family_target = Some(read_family_target);
        self
    }

    pub fn touched_authority_input(mut self, touched_authority_input: impl Into<String>) -> Self {
        self.touched_authority_input = Some(touched_authority_input.into());
        self
    }

    pub fn requirement_vocabulary(
        mut self,
        requirement_vocabulary: WorthGraphReadRequirementVocabulary,
    ) -> Self {
        self.requirement_vocabulary = Some(requirement_vocabulary);
        self
    }

    pub fn milestone_seven_lowering_target(
        mut self,
        milestone_seven_lowering_target: impl Into<String>,
    ) -> Self {
        self.milestone_seven_lowering_target = Some(milestone_seven_lowering_target.into());
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthGraphReadDeclarationCandidate, WorthGraphReadAccessPhaseSixError> {
        Ok(WorthGraphReadDeclarationCandidate {
            inventory_row_context: self.inventory_row_context.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowIdentity)
            })?,
            read_family_target: self.read_family_target.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingReadFamilyTarget)
            })?,
            touched_authority_input: require_non_empty(
                self.touched_authority_input,
                WorthGraphReadAccessPhaseSixErrorKind::MissingTouchedAuthorityInput,
            )?,
            requirement_vocabulary: self.requirement_vocabulary.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingRequirementVocabulary)
            })?,
            milestone_seven_lowering_target: require_non_empty(
                self.milestone_seven_lowering_target,
                WorthGraphReadAccessPhaseSixErrorKind::MissingMilestoneSevenLoweringTarget,
            )?,
        })
    }
}

fn require_non_empty(
    value: Option<String>,
    kind: WorthGraphReadAccessPhaseSixErrorKind,
) -> Result<String, WorthGraphReadAccessPhaseSixError> {
    let value = value.ok_or_else(|| error(kind))?;
    if value.is_empty() {
        return Err(error(kind));
    }
    Ok(value)
}

const fn error(kind: WorthGraphReadAccessPhaseSixErrorKind) -> WorthGraphReadAccessPhaseSixError {
    WorthGraphReadAccessPhaseSixError::new(kind)
}
