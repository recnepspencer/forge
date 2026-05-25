use forge_query::facade::ForgeQueryRawDeclarationInput;

fn main() {
    let _ = std::any::type_name::<ForgeQueryRawDeclarationInput<(), ()>>();
}
