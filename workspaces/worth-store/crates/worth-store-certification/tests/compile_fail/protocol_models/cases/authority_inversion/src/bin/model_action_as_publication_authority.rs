use worth_store_formal_models::ImportPublicationAction;
use worth_store_operations::complete_import_publication;

fn main() {
    let copied = ImportPublicationAction::PublicationDurable;
    let _ = complete_import_publication(copied, copied);
}
