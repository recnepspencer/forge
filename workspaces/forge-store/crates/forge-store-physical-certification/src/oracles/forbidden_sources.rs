use super::{OracleDenial, PhysicalProofOracleVerdict};

pub fn test_support_oracle_verdict_attempt() -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    Err(OracleDenial::TestSupportOracleDenied)
}

pub fn log_only_oracle_attempt() -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    Err(OracleDenial::LogOnlyEvidenceDenied)
}

pub fn expected_error_text_oracle_attempt() -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    Err(OracleDenial::ExpectedErrorTextDenied)
}

pub fn same_run_self_comparison_oracle_attempt() -> Result<PhysicalProofOracleVerdict, OracleDenial>
{
    Err(OracleDenial::SameRunSelfComparisonDenied)
}

pub fn fixture_label_oracle_attempt() -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    Err(OracleDenial::FixtureLabelOracleDenied)
}
