use super::certification_bundle::ForgeServerCertificationBundle;

pub fn assert_equal_on(
    left: &ForgeServerCertificationBundle,
    right: &ForgeServerCertificationBundle,
    fields: &[&str],
) {
    for field in fields {
        match *field {
            "request_context_digest" => {
                assert_eq!(
                    left.request_context_digest(),
                    right.request_context_digest()
                )
            }
            "response_digest" => assert_eq!(left.response_digest(), right.response_digest()),
            "provenance_digest" => {
                assert_eq!(left.provenance_digest(), right.provenance_digest())
            }
            "failure_digest" => assert_eq!(left.failure_digest(), right.failure_digest()),
            "counter_snapshot" => assert_eq!(left.counter_snapshot(), right.counter_snapshot()),
            other => panic!("unknown certification field {other}"),
        }
    }
}

pub fn assert_not_equal_on(
    left: &ForgeServerCertificationBundle,
    right: &ForgeServerCertificationBundle,
    fields: &[&str],
) {
    for field in fields {
        match *field {
            "request_context_digest" => {
                assert_ne!(
                    left.request_context_digest(),
                    right.request_context_digest()
                )
            }
            "response_digest" => assert_ne!(left.response_digest(), right.response_digest()),
            "provenance_digest" => {
                assert_ne!(left.provenance_digest(), right.provenance_digest())
            }
            "failure_digest" => assert_ne!(left.failure_digest(), right.failure_digest()),
            "counter_snapshot" => assert_ne!(left.counter_snapshot(), right.counter_snapshot()),
            other => panic!("unknown certification field {other}"),
        }
    }
}
