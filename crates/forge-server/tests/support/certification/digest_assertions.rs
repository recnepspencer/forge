use super::certification_bundle::{ForgeServerCertificationBundle, ForgeServerCertificationField};

pub fn assert_equal_on(
    left: &ForgeServerCertificationBundle,
    right: &ForgeServerCertificationBundle,
    fields: &[ForgeServerCertificationField],
) {
    for field in fields {
        match comparable_field(left, right, field) {
            ComparableField::String(left, right) => {
                assert_eq!(left, right, "field {}", field.label())
            }
            ComparableField::OptionalString(left, right) => {
                assert_eq!(left, right, "field {}", field.label())
            }
            ComparableField::CounterSnapshot(left, right) => {
                assert_eq!(left, right, "field {}", field.label())
            }
        }
    }
}

pub fn assert_not_equal_on(
    left: &ForgeServerCertificationBundle,
    right: &ForgeServerCertificationBundle,
    fields: &[ForgeServerCertificationField],
) {
    for field in fields {
        match comparable_field(left, right, field) {
            ComparableField::String(left, right) => {
                assert_ne!(left, right, "field {}", field.label())
            }
            ComparableField::OptionalString(left, right) => {
                assert_ne!(left, right, "field {}", field.label())
            }
            ComparableField::CounterSnapshot(left, right) => {
                assert_ne!(left, right, "field {}", field.label())
            }
        }
    }
}

enum ComparableField<'a> {
    String(&'a str, &'a str),
    OptionalString(Option<&'a str>, Option<&'a str>),
    CounterSnapshot(
        &'a std::collections::BTreeMap<String, u64>,
        &'a std::collections::BTreeMap<String, u64>,
    ),
}

fn comparable_field<'a>(
    left: &'a ForgeServerCertificationBundle,
    right: &'a ForgeServerCertificationBundle,
    field: &ForgeServerCertificationField,
) -> ComparableField<'a> {
    match field {
        ForgeServerCertificationField::RequestContextDigest => ComparableField::String(
            left.request_context_digest(),
            right.request_context_digest(),
        ),
        ForgeServerCertificationField::ResponseDigest => {
            ComparableField::String(left.response_digest(), right.response_digest())
        }
        ForgeServerCertificationField::ProvenanceDigest => {
            ComparableField::String(left.provenance_digest(), right.provenance_digest())
        }
        ForgeServerCertificationField::FailureDigest => {
            ComparableField::OptionalString(left.failure_digest(), right.failure_digest())
        }
        ForgeServerCertificationField::CounterSnapshot => {
            ComparableField::CounterSnapshot(left.counter_snapshot(), right.counter_snapshot())
        }
        ForgeServerCertificationField::Output(output) => {
            let left_output = left.output_digest(*output);
            let right_output = right.output_digest(*output);
            ComparableField::OptionalString(left_output, right_output)
        }
    }
}
