use forge_query::facade::{
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicApiNamingRow,
};

fn main() {
    let _row = ForgeQueryRuntimePublicApiNamingRow {
        concept: "live-view".to_string(),
        preferred_name: "surface".to_string(),
        alternate_names: Vec::new(),
        boundary_crossing: true,
        naming_digest: "unchecked".to_string(),
    };

    let _contract = ForgeQueryRuntimePublicApiNamingContract {
        rows: Vec::new(),
        preferred_entrypoint_count: 0,
        alternate_name_count: 0,
        boundary_crossing_name_count: 0,
        contract_digest: "unchecked".to_string(),
    };
}
