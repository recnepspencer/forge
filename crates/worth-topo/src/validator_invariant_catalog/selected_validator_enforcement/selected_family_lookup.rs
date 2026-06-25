use forge_query::facade::{ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportStatus};

use crate::validation::loop_wiring_rule;
use crate::validator_invariant_catalog::selected_validator_enforcement::WorthTopologySelectedValidatorEnforcementDenial;
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologySelectedLegalityObligationRow;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedLegalityObligationPlan,
    WorthTopologyValidatorFamilyIdentity,
};

pub(in crate::validator_invariant_catalog) fn selected_loop_wiring_obligation<'a>(
    selected_plan: &'a WorthTopologySelectedLegalityObligationPlan,
) -> Result<&'a WorthTopologySelectedLegalityObligationRow, WorthTopologyLegalityCatalogError> {
    let loop_wiring_identity =
        WorthTopologyValidatorFamilyIdentity::from_registered_rule(loop_wiring_rule());
    let Some(row) = selected_plan
        .selected_obligation_rows()
        .iter()
        .find(|row| row.worth_family_identity_digest() == loop_wiring_identity.identity_digest())
    else {
        return Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
            WorthTopologySelectedValidatorEnforcementDenial::missing_selected_family(
                "loop_wiring",
                loop_wiring_identity.identity_digest(),
            ),
        ));
    };
    if row.query_obligation_kind() != ForgeQueryGraphObligationKind::SchemaContractValidator {
        return Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
            WorthTopologySelectedValidatorEnforcementDenial::unexpected_obligation_kind(
                "loop_wiring",
                row.row_digest(),
                row.query_obligation_kind(),
                ForgeQueryGraphObligationKind::SchemaContractValidator,
            ),
        ));
    }
    if row.support_status() != ForgeQueryGraphObligationSupportStatus::Supported {
        return Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
            WorthTopologySelectedValidatorEnforcementDenial::unsupported_obligation(
                "loop_wiring",
                row.row_digest(),
                row.support_status(),
            ),
        ));
    }
    if row.execution_budget_digest().is_empty() || row.support_posture_digest().is_empty() {
        return Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
            WorthTopologySelectedValidatorEnforcementDenial::missing_support_or_budget_proof(
                "loop_wiring",
                row.row_digest(),
            ),
        ));
    }
    Ok(row)
}
