use worth_ui::facade::{CommandRegistry, ComponentRegistry};

fn main() {
    let _ = core::mem::size_of::<CommandRegistry>();
    let _ = core::mem::size_of::<ComponentRegistry>();
}
