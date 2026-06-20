use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationAdoptionManifest,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationInMemoryProof,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};

fn main() {
    let _ = ForgeQueryGraphObligationAdoptionManifest::new(
        "worth-kernel",
        todo::<ForgeQueryGraphObligationConsumerRegistrationDeclaration>(),
        todo::<ForgeQueryGraphObligationSelectorCoverageDeclaration>(),
        todo::<ForgeQueryGraphObligationSupportPin>(),
        "support-matrix",
        todo::<ForgeQueryGraphObligationResidueManifest>(),
        todo::<ForgeQueryGraphObligationLocalCeremonyAudit>(),
        todo::<ForgeQueryGraphObligationInMemoryProof>(),
        None::<&ForgeQueryGraphObligationExecutionProof>,
    );
}

fn todo<T>() -> &'static T {
    todo!()
}
