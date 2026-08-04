use worth_foundational::facade::CanonicalDigestDerivationDenial;

#[derive(Debug)]
pub(crate) enum WorthQueryGraphObligationInstallationDenial {
    Canonical(CanonicalDigestDerivationDenial),
    InvalidContract,
}

impl From<CanonicalDigestDerivationDenial> for WorthQueryGraphObligationInstallationDenial {
    fn from(denial: CanonicalDigestDerivationDenial) -> Self {
        Self::Canonical(denial)
    }
}
