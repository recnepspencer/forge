use forge_query::facade::AuthorizedProjectionFieldPath;

fn main() {
    let _ = AuthorizedProjectionFieldPath::from_terminal_ingress("profile.display_name");
    let _ = AuthorizedProjectionFieldPath::from_parts("profile", "display_name");
}
