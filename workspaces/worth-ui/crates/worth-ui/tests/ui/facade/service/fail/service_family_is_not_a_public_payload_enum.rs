//! `UiRuntimeServiceFamily` is runtime classification vocabulary. It must not
//! reach product code as a payload enum that application logic could switch on
//! to implement family behaviour.

fn main() {
    let _ = worth_ui::facade::service::UiRuntimeServiceFamily::Portal;
}
