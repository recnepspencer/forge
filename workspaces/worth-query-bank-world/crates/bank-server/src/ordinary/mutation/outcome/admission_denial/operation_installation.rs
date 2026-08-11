use worth_query_host::facade::domain::WorthQueryApplicationOperationInstallationDenialKind as QueryKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankOperationInstallationDenialKind {
    OperationNotInstalled,
    OperationMeaningChanged,
    MissingAbilityPolicy,
    MissingProgram,
    MissingDecisionFactBudget,
    MissingProjectionWorkBudget,
    ConflictingAuthorizationContract,
    InvalidMutationPreconditionContract,
    CanonicalEntryBudgetExceeded,
    CanonicalEncodedByteBudgetExceeded,
    CanonicalDigestSlotRejected,
    InvalidGraphObligationContract,
    AftermathInstallationDenied,
    AmbiguousExternalEffectContract,
    AmbiguousAftermathContract,
    ForeignRuntime,
    StaleGeneration,
    SchemaMeaningChanged,
    PackageIdentityChanged,
    AuthorityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BankOperationInstallationDenial {
    kind: BankOperationInstallationDenialKind,
}

impl BankOperationInstallationDenial {
    pub const fn kind(self) -> BankOperationInstallationDenialKind {
        self.kind
    }

    pub const fn code(self) -> &'static str {
        use BankOperationInstallationDenialKind as Bank;
        match self.kind {
            Bank::OperationNotInstalled => "operation-not-installed",
            Bank::OperationMeaningChanged => "operation-meaning-changed",
            Bank::MissingAbilityPolicy => "missing-ability-policy",
            Bank::MissingProgram => "missing-program",
            Bank::MissingDecisionFactBudget => "missing-decision-fact-budget",
            Bank::MissingProjectionWorkBudget => "missing-projection-work-budget",
            Bank::ConflictingAuthorizationContract => "conflicting-authorization-contract",
            Bank::InvalidMutationPreconditionContract => "invalid-mutation-precondition-contract",
            Bank::CanonicalEntryBudgetExceeded => "canonical-entry-budget-exceeded",
            Bank::CanonicalEncodedByteBudgetExceeded => "canonical-byte-budget-exceeded",
            Bank::CanonicalDigestSlotRejected => "canonical-digest-slot-rejected",
            Bank::InvalidGraphObligationContract => "invalid-graph-obligation-contract",
            Bank::AftermathInstallationDenied => "aftermath-installation-denied",
            Bank::AmbiguousExternalEffectContract => "ambiguous-external-effect-contract",
            Bank::AmbiguousAftermathContract => "ambiguous-aftermath-contract",
            Bank::ForeignRuntime => "foreign-runtime",
            Bank::StaleGeneration => "stale-generation",
            Bank::SchemaMeaningChanged => "schema-meaning-changed",
            Bank::PackageIdentityChanged => "package-identity-changed",
            Bank::AuthorityMismatch => "authority-mismatch",
        }
    }

    pub(crate) const fn from_query(kind: QueryKind) -> Self {
        use BankOperationInstallationDenialKind as Bank;
        let kind = match kind {
            QueryKind::OperationNotInstalled => Bank::OperationNotInstalled,
            QueryKind::OperationMeaningChanged => Bank::OperationMeaningChanged,
            QueryKind::MissingAbilityPolicy => Bank::MissingAbilityPolicy,
            QueryKind::MissingProgram => Bank::MissingProgram,
            QueryKind::MissingDecisionFactBudget => Bank::MissingDecisionFactBudget,
            QueryKind::MissingProjectionWorkBudget => Bank::MissingProjectionWorkBudget,
            QueryKind::ConflictingAuthorizationContract => Bank::ConflictingAuthorizationContract,
            QueryKind::InvalidMutationPreconditionContract => {
                Bank::InvalidMutationPreconditionContract
            }
            QueryKind::CanonicalEntryBudgetExceeded => Bank::CanonicalEntryBudgetExceeded,
            QueryKind::CanonicalEncodedByteBudgetExceeded => {
                Bank::CanonicalEncodedByteBudgetExceeded
            }
            QueryKind::CanonicalDigestSlotRejected => Bank::CanonicalDigestSlotRejected,
            QueryKind::InvalidGraphObligationContract => Bank::InvalidGraphObligationContract,
            QueryKind::AftermathInstallationDenied => Bank::AftermathInstallationDenied,
            QueryKind::AmbiguousExternalEffectContract => Bank::AmbiguousExternalEffectContract,
            QueryKind::AmbiguousAftermathContract => Bank::AmbiguousAftermathContract,
            QueryKind::ForeignRuntime => Bank::ForeignRuntime,
            QueryKind::StaleGeneration => Bank::StaleGeneration,
            QueryKind::SchemaMeaningChanged => Bank::SchemaMeaningChanged,
            QueryKind::PackageIdentityChanged => Bank::PackageIdentityChanged,
            QueryKind::AuthorityMismatch => Bank::AuthorityMismatch,
        };
        Self { kind }
    }
}

impl std::fmt::Display for BankOperationInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}
