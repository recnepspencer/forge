use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationEntityIdentity, WorthQueryApplicationQueryAccessContext,
    WorthQueryAuthenticatedPrincipal,
};

struct Schema;
struct Principal;
struct PrincipalIdentity;
struct Account;
struct Institution;

fn require_account_scope(
    _: WorthQueryApplicationQueryAccessContext<'_, Schema, Principal, PrincipalIdentity, Account>,
) {
}

fn institution_scope_cannot_substitute_for_account_scope(
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    institution: &WorthQueryApplicationEntityIdentity<Schema, Institution>,
) {
    require_account_scope(WorthQueryApplicationQueryAccessContext::new(
        principal,
        institution,
    ));
}

fn main() {}
