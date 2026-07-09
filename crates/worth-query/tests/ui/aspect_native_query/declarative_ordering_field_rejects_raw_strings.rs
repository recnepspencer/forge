use worth_query::facade::DeclarativeOrderingField;

fn main() {
    let _ = DeclarativeOrderingField::ascending("identity", "id");
    let _ = DeclarativeOrderingField::descending("identity", "id");
}
