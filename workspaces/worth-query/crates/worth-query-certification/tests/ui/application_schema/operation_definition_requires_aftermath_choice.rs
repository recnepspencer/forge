use worth_query_decl::facade::worth_query_operation;

struct Schema;
struct Input;
worth_query_operation!(Operation(Input) in Schema);

fn main() {
    let _ = Operation::reference()
        .definition()
        .no_external_effect()
        .finish();
}
