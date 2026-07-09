use worth_query::facade::consumer_kit::{
    query_consumer_residue_audit, query_owned_consumer_residue_root_authority,
};

fn main() {
    let authority = query_owned_consumer_residue_root_authority();
    let _audit = query_consumer_residue_audit("downstream")
        .required_query_owned_implementation_root("src", &authority);
}
