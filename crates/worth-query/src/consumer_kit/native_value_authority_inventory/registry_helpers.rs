use super::{
    WorthQueryNativeValueAuthorityClass as Class, WorthQueryNativeValueAuthorityRow as Row,
    WorthQueryNativeValueDisposition as Disposition,
};

pub(super) const fn coarse(
    symbol: &'static str,
    path: &'static str,
    exports: &'static [&'static str],
    consumers: &'static [&'static str],
    owner: &'static str,
) -> Row {
    Row::new(
        symbol,
        path,
        exports,
        consumers,
        Class::CoarseSemanticVocabulary,
        Disposition::ReplaceWithFoundationalValue,
        owner,
    )
}

pub(super) const fn contract_projection(
    symbol: &'static str,
    path: &'static str,
    exports: &'static [&'static str],
    consumers: &'static [&'static str],
    owner: &'static str,
) -> Row {
    Row::new(
        symbol,
        path,
        exports,
        consumers,
        Class::ContractCapabilityProjection,
        Disposition::DeriveFromFoundationalContract,
        owner,
    )
}

pub(super) const fn proof(
    symbol: &'static str,
    path: &'static str,
    exports: &'static [&'static str],
    consumers: &'static [&'static str],
    owner: &'static str,
) -> Row {
    Row::new(
        symbol,
        path,
        exports,
        consumers,
        Class::ProofBearingCarrier,
        Disposition::PreserveWithProof,
        owner,
    )
}

pub(super) const fn unvalidated(
    symbol: &'static str,
    path: &'static str,
    exports: &'static [&'static str],
    consumers: &'static [&'static str],
    owner: &'static str,
) -> Row {
    Row::new(
        symbol,
        path,
        exports,
        consumers,
        Class::UnvalidatedNativeCarrier,
        Disposition::SealBehindContractValidation,
        owner,
    )
}

pub(super) const fn misleading(
    symbol: &'static str,
    path: &'static str,
    exports: &'static [&'static str],
    consumers: &'static [&'static str],
) -> Row {
    Row::new(
        symbol,
        path,
        exports,
        consumers,
        Class::MisleadingCarrier,
        Disposition::RealizeOrRename,
        "phase-28-native-result-consumption",
    )
}
