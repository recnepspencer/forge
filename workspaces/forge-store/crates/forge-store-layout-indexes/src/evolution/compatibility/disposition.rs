#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySurfaceDisposition {
    ConsumedAsProductionAuthority,
    ConsumedAsInputOnly,
    WrappedBehindFacade,
    SupersededAndForbidden,
    CertificationOnly,
    TerminalOnly,
    DeprecatedDebt,
    ForbiddenAsAuthority,
}
