use worth_query::facade::consumer_kit::{
    audit_public_authority_surface_symbols, worth_query_public_authority_surface_rows,
};

fn main() {
    let rows = worth_query_public_authority_surface_rows();
    let observed = rows.iter().map(|row| row.symbol()).collect::<Vec<_>>();
    let audit = audit_public_authority_surface_symbols(&observed);

    assert!(audit.is_complete());
    assert_eq!(audit.classified_surface_count(), rows.len());
}
