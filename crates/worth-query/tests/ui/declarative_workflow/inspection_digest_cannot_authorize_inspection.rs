use worth_query::facade::inspection::inspect;

fn cannot_inspect_digest(digest: &str) {
    let _declaration = inspect(digest);
}

fn main() {}
