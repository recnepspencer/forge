#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphObligationDispatchError {
    EmptyRuleNamespace,
    EmptyRuleName,
    EmptyRuleVersion,
    EmptyTouchDescriptorDigest,
    EmptyOperatingWorldDigest,
    EmptyVerdictContext,
    EmptyEnvelope,
}

impl std::fmt::Display for ForgeQueryGraphObligationDispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyRuleNamespace => "graph obligation rule namespace must not be empty",
            Self::EmptyRuleName => "graph obligation rule name must not be empty",
            Self::EmptyRuleVersion => "graph obligation rule version must not be empty",
            Self::EmptyTouchDescriptorDigest => "touch descriptor digest must not be empty",
            Self::EmptyOperatingWorldDigest => "operating world digest must not be empty",
            Self::EmptyVerdictContext => "graph obligation verdict context must not be empty",
            Self::EmptyEnvelope => {
                "graph obligation dispatch envelope must record at least one row"
            }
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ForgeQueryGraphObligationDispatchError {}
