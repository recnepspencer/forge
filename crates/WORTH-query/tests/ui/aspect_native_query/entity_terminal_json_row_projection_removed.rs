use worth_query::facade::WorthQueryEntity;

fn main() {
    let entity: WorthQueryEntity = unreachable!();
    let _ = entity.terminal_json_row_projection();

    let entity: WorthQueryEntity = unreachable!();
    let _ = entity.into_terminal_json_row_projection();
}
