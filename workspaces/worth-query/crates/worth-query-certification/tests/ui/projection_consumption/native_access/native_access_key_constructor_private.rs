use worth_query::facade::domain::WorthQueryNativeAccessKey;
use worth_query::facade::foundation::ProjectionFactFieldPath;

fn forge(path: ProjectionFactFieldPath) -> WorthQueryNativeAccessKey {
    WorthQueryNativeAccessKey::new(path)
}

fn main() {}
