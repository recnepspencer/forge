use worth_query::WorthQueryPublicBridgePublishedProjectionReader;

fn leaks_binding(reader: &WorthQueryPublicBridgePublishedProjectionReader<'_>) {
    let _ = reader.published_binding();
    let _ = reader.materialization_by_name("view");
    let _ = reader.rows();
}

fn main() {}
