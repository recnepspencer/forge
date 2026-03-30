#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
#[derive(Debug, Clone, Copy)]
pub struct RuntimeInstant(std::time::Instant);

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
impl RuntimeInstant {
    pub fn now() -> Self {
        Self(std::time::Instant::now())
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuntimeInstant {
    millis: f64,
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
impl RuntimeInstant {
    pub fn now() -> Self {
        Self {
            millis: monotonic_now_millis(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        duration_from_millis(monotonic_now_millis() - self.millis)
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn monotonic_now_millis() -> f64 {
    use wasm_bindgen::JsValue;

    let global = js_sys::global();

    if let Ok(performance) = js_sys::Reflect::get(&global, &JsValue::from_str("performance")) {
        if let Some(millis) = call_zero_arg_number(&performance, "now") {
            return millis;
        }
    }

    if let Ok(date) = js_sys::Reflect::get(&global, &JsValue::from_str("Date")) {
        if let Some(millis) = call_zero_arg_number(&date, "now") {
            return millis;
        }
    }

    0.0
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn call_zero_arg_number(target: &wasm_bindgen::JsValue, name: &str) -> Option<f64> {
    use wasm_bindgen::{JsCast, JsValue};

    let property = js_sys::Reflect::get(target, &JsValue::from_str(name)).ok()?;
    let function = property.dyn_ref::<js_sys::Function>()?;
    function.call0(target).ok()?.as_f64()
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn duration_from_millis(millis: f64) -> std::time::Duration {
    if millis <= 0.0 {
        return std::time::Duration::ZERO;
    }

    let nanos = (millis * 1_000_000.0).round();
    if nanos.is_finite() && nanos > 0.0 {
        std::time::Duration::from_nanos(nanos.min(u64::MAX as f64) as u64)
    } else {
        std::time::Duration::ZERO
    }
}
