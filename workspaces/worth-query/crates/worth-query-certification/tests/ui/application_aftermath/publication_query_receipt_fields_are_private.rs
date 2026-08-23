use worth_query_host::facade::publication::application_aftermath::WorthQueryPublishedCanonicalWork;
use worth_query_host::facade::publication::domain_computation::{
    WorthQueryApplicationQueryPublicationReceipt, WorthQueryPublishedApplicationDisclosure,
    WorthQueryPublishedApplicationQueryTerminalRelease,
};

fn forge_query_receipt(
    result_count: usize,
    ordinary_work_units: usize,
    disclosure: WorthQueryPublishedApplicationDisclosure,
    publication_work: WorthQueryPublishedCanonicalWork,
    terminal_release: WorthQueryPublishedApplicationQueryTerminalRelease,
) -> WorthQueryApplicationQueryPublicationReceipt {
    WorthQueryApplicationQueryPublicationReceipt {
        result_count,
        ordinary_work_units,
        disclosure,
        publication_work,
        terminal_release,
    }
}

fn main() {}
