use worth_query::facade::consumer_kit::{
    WorthQueryGraphObligationAdoptionManifest,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationExecutionProof, WorthQueryGraphObligationInMemoryProof,
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};

fn main() {
    let _ = WorthQueryGraphObligationAdoptionManifest::new(
        "worth-kernel",
        todo::<WorthQueryGraphObligationConsumerRegistrationDeclaration>(),
        todo::<WorthQueryGraphObligationSelectorCoverageDeclaration>(),
        todo::<WorthQueryGraphObligationSupportPin>(),
        "support-matrix",
        todo::<WorthQueryGraphObligationResidueManifest>(),
        todo::<WorthQueryGraphObligationLocalCeremonyAudit>(),
        todo::<WorthQueryGraphObligationInMemoryProof>(),
        None::<&WorthQueryGraphObligationExecutionProof>,
    );
}

fn todo<T>() -> &'static T {
    todo!()
}
