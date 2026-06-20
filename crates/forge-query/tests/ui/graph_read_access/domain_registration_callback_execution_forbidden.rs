use forge_query::facade::runtime::ForgeQueryGraphReadOperationRegistration;

fn main() {
    let registration = ForgeQueryGraphReadOperationRegistration::domain(
        "worth.geometry.visible_face_neighborhood",
        1,
        "worth.geometry",
    );
    registration.execute_callback(|| Vec::<String>::new());
}

