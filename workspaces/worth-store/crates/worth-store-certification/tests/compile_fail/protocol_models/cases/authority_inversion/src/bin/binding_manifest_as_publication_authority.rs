use worth_store_formal_models::current_protocol_binding_manifest;
use worth_store_operations::complete_import_publication;

fn main() {
    let copied = current_protocol_binding_manifest();
    let _ = complete_import_publication(copied, copied);
}
