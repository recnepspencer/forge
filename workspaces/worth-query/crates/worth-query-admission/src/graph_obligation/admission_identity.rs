use worth_foundational::facade::{
    CanonicalDigestDerivationDenial, CanonicalDigestId, CanonicalDigestWorkBudget,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use crate::canonical_identity_derivation::WorthQueryCanonicalIdentityBasis;

use super::{WorthQueryGraphWorkIntentKind, WorthQueryRequiredGraphWork};

const GRAPH_WORK_PLAN_IDENTITY_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(8, 64 * 1024) {
        Some(budget) => budget,
        None => panic!("fixed graph-work admission identity budget is valid"),
    };

pub(super) fn derive_graph_work_plan_identity(
    required: &WorthQueryRequiredGraphWork,
    support_identity: &str,
    planning_identity: Option<&CanonicalDigestId>,
) -> Result<(CanonicalDigestId, WorthQueryCanonicalWorkEvidence), CanonicalDigestDerivationDenial> {
    let mut basis = WorthQueryCanonicalIdentityBasis::new(
        "worth-query.admitted-graph-work",
        "worth-query-admitted-graph-work-v1",
        GRAPH_WORK_PLAN_IDENTITY_BUDGET,
    );
    basis.digest("obligations", *required.identity().digest())?;
    basis.text("intent", intent_name(required.intent().kind()))?;
    basis.text("support", support_identity)?;
    if let Some(planning_identity) = planning_identity {
        basis.digest("graph-read-requirements", *planning_identity)?;
    }
    basis.derive()
}

const fn intent_name(intent: WorthQueryGraphWorkIntentKind) -> &'static str {
    match intent {
        WorthQueryGraphWorkIntentKind::ApplicationQueryRead => "application-query-read",
        WorthQueryGraphWorkIntentKind::ApplicationOperationRead => "application-operation-read",
        WorthQueryGraphWorkIntentKind::ApplicationOperationMutation => {
            "application-operation-mutation"
        }
    }
}
