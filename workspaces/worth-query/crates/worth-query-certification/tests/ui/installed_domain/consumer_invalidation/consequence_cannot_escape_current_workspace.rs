use worth_query::facade::{domain, runtime};

fn escape_current_guard<'a>(
    admitted: &'a domain::WorthQueryAdmittedConsumerInvalidation<'a>,
    workspace: &'a runtime::WorthQueryWorkspace,
) -> domain::WorthQueryConsumerConsequence<'static, ()> {
    admitted
        .attach_consumer_authored_consequence(
            workspace,
            domain::WorthQueryConsumerInvalidationDisposition::LocalPatch,
            (),
        )
        .ok()
        .unwrap()
}

fn main() {}
