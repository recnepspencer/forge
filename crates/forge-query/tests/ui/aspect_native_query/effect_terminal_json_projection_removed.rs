use forge_query::facade::{ForgeQueryEffectDelivery, ForgeQueryEffectPayload};

fn payload_projection(payload: ForgeQueryEffectPayload) {
    let _ = payload.terminal_json_projection();
}

fn delivery_projection(delivery: ForgeQueryEffectDelivery) {
    let _ = delivery.terminal_json_payload_projection();
}

fn main() {}
