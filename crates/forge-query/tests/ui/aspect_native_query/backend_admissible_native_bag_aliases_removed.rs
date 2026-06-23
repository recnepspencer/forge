use forge_query::facade::ForgeQueryBackendAdmissibleMutation;

fn main() {}

fn removed_backend_admissible_native_bag_aliases(mutation: &ForgeQueryBackendAdmissibleMutation) {
    let _ = mutation.aspect_values();
    let _ = mutation.asserted_aspect_values();
    let _ = mutation.touched_aspects();
}
