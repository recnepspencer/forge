use worth_ui::facade::declaration::CommandId;

fn main() {
    let _ = CommandId { id: fake_id() };
}

fn fake_id<T>() -> T {
    loop {}
}
