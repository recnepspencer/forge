use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryAuthenticatedExternalPrincipal,
    primary_graph::{WorthQueryApplicationEntityIdentity, WorthQueryApplicationQueryAccessContext},
};

struct Schema;
struct Account;

fn external_authentication_is_not_query_access(
    external: &WorthQueryAuthenticatedExternalPrincipal<Schema>,
    account: &WorthQueryApplicationEntityIdentity<Schema, Account>,
) {
    let _ = WorthQueryApplicationQueryAccessContext::new(external, account);
}

fn main() {}
