use worth_query::facade::runtime::WorthQueryGraphReadOperationRegistration;

fn main() {
    let registration = WorthQueryGraphReadOperationRegistration::domain(
        "worth.geometry.visible_face_neighborhood",
        1,
        "worth.geometry",
    );
    registration.execute_callback(|| Vec::<String>::new());
}

