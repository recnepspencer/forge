use worth_query::facade::{
    WorthQueryRuntimePublicApiNamingContract, WorthQueryRuntimePublicApiNamingRow,
};

fn main() {
    let _row = WorthQueryRuntimePublicApiNamingRow {
        concept: "live-view".to_string(),
        preferred_name: "surface".to_string(),
        alternate_names: Vec::new(),
        boundary_crossing: true,
        naming_digest: "unchecked".to_string(),
    };

    let _contract = WorthQueryRuntimePublicApiNamingContract {
        rows: Vec::new(),
        preferred_entrypoint_count: 0,
        alternate_name_count: 0,
        boundary_crossing_name_count: 0,
        contract_digest: "unchecked".to_string(),
    };
}
