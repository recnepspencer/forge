use worth_ui::facade::{
    WorthUiActivatedProjectionRebindPlan, WorthUiAdmittedProjectionPlan, WorthUiHeaderMenuPlan,
    WorthUiPreservedProjectionRebindPlan, WorthUiProjectionDependencyAdmissionDenial,
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionDependencyValidationProof, WorthUiProjectionEquivalenceBasis,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanAdmissionDenial, WorthUiProjectionPlanContract,
    WorthUiProjectionPlanProof, WorthUiProjectionRebindBatchReceipt,
    WorthUiProjectionRebindBatchAggregationDenial, WorthUiProjectionRebindCounters,
    WorthUiProjectionRebindPlan, WorthUiProjectionRebindPlanDenial,
    WorthUiProjectionRebindRowReceipt, WorthUiProjectionRebindStatus,
    WorthUiValidatedProjectionDependencyContract,
};

fn accepts_projection_contract<P: WorthUiProjectionPlanContract>() {}

fn main() {
    let _: Option<WorthUiAdmittedProjectionPlan<WorthUiHeaderMenuPlan>> = None;
    let _: Option<WorthUiProjectionDependencyAdmissionDenial> = None;
    let _: Option<WorthUiProjectionDependencyDeclaration> = None;
    let _: Option<WorthUiProjectionDependencySet> = None;
    let _: Option<WorthUiProjectionDependencyValidationProof> = None;
    let _: Option<WorthUiProjectionEquivalenceBasis> = None;
    let _: Option<WorthUiProjectionIdentity> = None;
    let _: Option<WorthUiProjectionPlanAdmissionDenial> = None;
    let _: Option<WorthUiProjectionPlanProof> = None;
    let _: Option<WorthUiActivatedProjectionRebindPlan<WorthUiHeaderMenuPlan>> = None;
    let _: Option<WorthUiPreservedProjectionRebindPlan<WorthUiHeaderMenuPlan>> = None;
    let _: Option<WorthUiProjectionRebindBatchAggregationDenial> = None;
    let _: Option<WorthUiProjectionRebindBatchReceipt> = None;
    let _: Option<WorthUiProjectionRebindCounters> = None;
    let _: Option<WorthUiProjectionRebindPlan<WorthUiHeaderMenuPlan>> = None;
    let _: Option<WorthUiProjectionRebindPlanDenial> = None;
    let _: Option<WorthUiProjectionRebindRowReceipt> = None;
    let _: Option<WorthUiValidatedProjectionDependencyContract> = None;
    let _ = WorthUiProjectionFamily::HeaderMenu;
    let _ = WorthUiProjectionEquivalenceBasisKind::ProjectionDigest;
    let _ = WorthUiProjectionRebindStatus::EquivalentAfterActivation;
    accepts_projection_contract::<WorthUiHeaderMenuPlan>();
}
