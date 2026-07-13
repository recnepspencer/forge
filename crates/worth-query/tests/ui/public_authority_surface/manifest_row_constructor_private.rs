use worth_query::facade::consumer_kit::{
    WorthQueryPublicAuthorityOwner, WorthQueryPublicAuthoritySurfaceClass,
    WorthQueryPublicAuthoritySurfaceRow,
};

fn main() {
    let _ = WorthQueryPublicAuthoritySurfaceRow::new(
        "consumer-minted-authority-surface",
        "consumer.rs",
        "mint",
        None,
        None,
        "consumer authority",
        WorthQueryPublicAuthorityOwner::Identity,
        WorthQueryPublicAuthoritySurfaceClass::OrdinaryDeclarativeApi,
        WorthQueryPublicAuthoritySurfaceClass::SealedPhaseApi,
        "consumer-selected replacement",
    );
}
