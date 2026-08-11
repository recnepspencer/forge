use worth_query_host::facade::installed::provider_session::WorthQueryProviderCompareAndCommitOutcome;

fn cannot_author_provider_commit() {
    let _ = WorthQueryProviderCompareAndCommitOutcome::Committed {
        provider_receipt: String::from("caller-authored-receipt"),
    };
}

fn main() {}
