use forge_query::facade::ForgeQueryAspect;

fn main() {
    let aspect = ForgeQueryAspect::new("title.value", "title.value");
    let _: &str = aspect.external_projection_path();
}
