use forge_query::ForgeQueryPublicBridgePublishedProjectionReader;

fn leaks_binding(reader: &ForgeQueryPublicBridgePublishedProjectionReader<'_>) {
    let _ = reader.published_binding();
    let _ = reader.materialization_by_name("view");
    let _ = reader.rows();
}

fn main() {}
