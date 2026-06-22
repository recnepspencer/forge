use forge_query::facade::ForgeQueryEntity;

fn main() {
    let entity: ForgeQueryEntity = unreachable!();
    let _ = entity.terminal_json_row_projection();

    let entity: ForgeQueryEntity = unreachable!();
    let _ = entity.into_terminal_json_row_projection();
}
