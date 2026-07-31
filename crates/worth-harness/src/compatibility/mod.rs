use serde::{Deserialize, Serialize};

use crate::capture::RecordSchemaVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityStatus {
    Compatible,
    UnsupportedVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityPolicy {
    Exact,
    BackwardCompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub expected: RecordSchemaVersion,
    pub actual: RecordSchemaVersion,
    pub status: CompatibilityStatus,
}

pub fn check_record_schema(
    expected: RecordSchemaVersion,
    actual: RecordSchemaVersion,
) -> CompatibilityReport {
    check_record_schema_with_policy(expected, actual, CompatibilityPolicy::Exact)
}

pub fn check_record_schema_with_policy(
    expected: RecordSchemaVersion,
    actual: RecordSchemaVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityReport {
    let compatible = expected == actual
        || (matches!(policy, CompatibilityPolicy::BackwardCompatible) && actual <= expected);
    let status = if compatible {
        CompatibilityStatus::Compatible
    } else {
        CompatibilityStatus::UnsupportedVersion
    };
    CompatibilityReport {
        expected,
        actual,
        status,
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::RecordSchemaVersion;

    use super::{
        check_record_schema, check_record_schema_with_policy, CompatibilityPolicy,
        CompatibilityStatus,
    };

    #[test]
    fn compatibility_report_is_compatible_for_matching_versions() {
        let report = check_record_schema(RecordSchemaVersion::V1, RecordSchemaVersion::V1);
        assert_eq!(report.status, CompatibilityStatus::Compatible);
    }

    #[test]
    fn compatibility_policy_can_be_checked_explicitly() {
        let report = check_record_schema_with_policy(
            RecordSchemaVersion::V1,
            RecordSchemaVersion::V1,
            CompatibilityPolicy::BackwardCompatible,
        );
        assert_eq!(report.status, CompatibilityStatus::Compatible);
    }
}
