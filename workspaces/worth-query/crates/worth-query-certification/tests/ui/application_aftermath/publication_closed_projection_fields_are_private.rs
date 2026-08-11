use worth_query_host::facade::publication::application_aftermath::{
    WorthQueryPublishedApplicationAftermath, WorthQueryPublishedApplicationCommitBoundaryEvidence,
    WorthQueryPublishedCanonicalWork, WorthQueryPublishedExternalEffectPosture,
};
use worth_query_host::facade::publication::domain_computation::{
    WorthQueryApplicationCommitPublicationReceipt, WorthQueryApplicationQueryPublicationReceipt,
    WorthQueryPublishedApplicationDisclosure, WorthQueryPublishedApplicationDisclosureIdentity,
    WorthQueryPublishedApplicationDisclosurePosture,
    WorthQueryPublishedApplicationQueryReleasePosture,
    WorthQueryPublishedApplicationQueryResultBufferRelease,
    WorthQueryPublishedApplicationQueryTerminalRelease,
};

fn forge_commit_receipt(
    aftermath: WorthQueryPublishedApplicationAftermath,
    boundary_evidence: WorthQueryPublishedApplicationCommitBoundaryEvidence,
) -> WorthQueryApplicationCommitPublicationReceipt {
    WorthQueryApplicationCommitPublicationReceipt {
        aftermath,
        boundary_evidence,
    }
}

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

fn forge_terminal_release() -> WorthQueryPublishedApplicationQueryTerminalRelease {
    WorthQueryPublishedApplicationQueryTerminalRelease {
        application_basis: WorthQueryPublishedApplicationQueryReleasePosture::Released,
        graph_read_basis: WorthQueryPublishedApplicationQueryReleasePosture::Released,
        result_buffer: WorthQueryPublishedApplicationQueryResultBufferRelease::Released {
            limit_bytes: 1024,
            peak_bytes: 64,
        },
        released_graph_capacity_reservation_count: 1,
    }
}

fn forge_disclosure(
    identity: WorthQueryPublishedApplicationDisclosureIdentity,
    posture: WorthQueryPublishedApplicationDisclosurePosture,
) -> WorthQueryPublishedApplicationDisclosure {
    WorthQueryPublishedApplicationDisclosure {
        identity,
        posture,
        disclosure_decision_count: 4,
        disclosed_value_count: 2,
        omitted_value_count: 2,
        authorization_decision_fact_count: 7,
    }
}

fn forge_aftermath(
    external_effect: WorthQueryPublishedExternalEffectPosture,
) -> WorthQueryPublishedApplicationAftermath {
    WorthQueryPublishedApplicationAftermath {
        posture: None,
        external_effect,
    }
}

fn main() {}
