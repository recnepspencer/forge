use worth_query::facade::domain::WorthQueryNativeAccessKey;
use worth_query::facade::foundation::ProjectionFactFieldPath;

fn forge_key(path: ProjectionFactFieldPath) -> WorthQueryNativeAccessKey {
    path.into()
}

fn main() {}
