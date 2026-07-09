use worth_query::facade::WorthQueryRawDeclarationInput;

fn main() {
    let _ = std::any::type_name::<WorthQueryRawDeclarationInput<(), ()>>();
}
