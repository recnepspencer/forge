use egui::{Frame, RichText, Stroke, Ui};

use crate::runtime::ValidationWorkbenchSnapshot;

use super::{ValidationDynamicPageRequest, ValidationResolvedPage, ValidationWorkspaceState};

pub(crate) fn render_slot(
    ui: &mut Ui,
    page: &ValidationResolvedPage,
    slot_name: &str,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
) {
    match page.authoring_page_name(state) {
        "OverviewPage" => render_overview_slot(ui, slot_name, snapshot, state),
        "ProductsPage" => render_products_slot(ui, slot_name, state),
        "OrdersPage" => render_orders_slot(ui, slot_name, state),
        "CustomersPage" => render_customers_slot(ui, slot_name),
        "ProductDetailPage" => render_product_detail_slot(ui, page, slot_name, state),
        "OrderDetailPage" => render_order_detail_slot(ui, page, slot_name, state),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_overview_slot(
    ui: &mut Ui,
    slot_name: &str,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
) {
    match slot_name {
        "summary" => section(ui, "Launch proof", |ui| {
            ui.label(format!("Artifact digest: {}", snapshot.artifact_digest()));
            ui.label(format!("Plan digest: {}", snapshot.active_plan_digest()));
            ui.label(format!("Static pages: {}", snapshot.page_count()));
            ui.label(format!(
                "Dynamic templates: {}",
                snapshot.dynamic_page_count()
            ));
        }),
        "evidence" => section(ui, "Workspace shell", |ui| {
            ui.label("The shell stays mounted while page content swaps inside the shared host.");
            if ui.button("Open command palette overlay").clicked() {
                state.toggle_command_palette();
            }
            ui.separator();
            ui.label("Topbar, rail, page host, inspector, status, overlays, and toasts are all workspace-owned surfaces.");
        }),
        "status" => section(ui, "Status surfaces", |ui| {
            ui.label("Reload, diagnostics, and runtime evidence stay platform-owned and can be projected here later.");
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_products_slot(ui: &mut Ui, slot_name: &str, state: &mut ValidationWorkspaceState) {
    match slot_name {
        "toolbar" => section(ui, "Products toolbar", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Products").strong());
                ui.separator();
                ui.label("Search");
                ui.label("Saved views");
                ui.label("Bulk actions");
            });
        }),
        "filters" => section(ui, "Filters", |ui| {
            ui.label("Sales channel");
            ui.label("Status");
            ui.label("Inventory");
            ui.label("Vendor");
            ui.separator();
            ui.small("This slot is a scroll-owning sidebar in the authored layout.");
        }),
        "collection" => section(ui, "Product collection", |ui| {
            product_row(ui, "P-1001", "Travel Pack", "$128", state);
            product_row(ui, "P-1002", "Field Journal", "$22", state);
            product_row(ui, "P-1003", "Canvas Tote", "$44", state);
            product_row(ui, "P-1004", "Trail Cap", "$29", state);
            product_row(ui, "P-1005", "Utility Pouch", "$18", state);
        }),
        "activity" => section(ui, "Activity", |ui| {
            ui.label("Price changed on Travel Pack");
            ui.label("Inventory synced for Canvas Tote");
            ui.label("Field Journal published to online store");
            ui.label("Bulk archive denied for discontinued bundle");
        }),
        "inspector" => section(ui, "Inspector", |ui| {
            ui.label("Selected product");
            ui.monospace("P-1001");
            ui.separator();
            ui.label("Status: Draft");
            ui.label("Inventory: 42");
            if ui.button("Open product detail").clicked() {
                let request = ValidationDynamicPageRequest::product_detail("P-1001")
                    .expect("sample product detail request should be valid");
                state
                    .navigation_mut()
                    .open_dynamic_page(request)
                    .expect("sample product detail request should open");
                state.push_toast("Opened product detail for P-1001");
            }
        }),
        "status" => section(ui, "Status summary", |ui| {
            ui.label("5 products visible");
            ui.label("1 pending publish action");
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_orders_slot(ui: &mut Ui, slot_name: &str, state: &mut ValidationWorkspaceState) {
    match slot_name {
        "filters" => section(ui, "Queue filters", |ui| {
            ui.label("Fulfillment");
            ui.label("Payment review");
            ui.label("Fraud risk");
        }),
        "queue" => section(ui, "Order queue", |ui| {
            order_row(ui, "O-4902", "Payment review", state);
            order_row(ui, "O-4908", "Packed", state);
            order_row(ui, "O-4914", "Awaiting carrier pickup", state);
            order_row(ui, "O-4920", "Refund requested", state);
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_customers_slot(ui: &mut Ui, slot_name: &str) {
    match slot_name {
        "segments" => section(ui, "Segments", |ui| {
            ui.label("High-value repeat buyers");
            ui.label("Wholesale storefronts");
            ui.label("Recently churned subscribers");
            ui.label("Regional event customers");
        }),
        "profile" => section(ui, "Customer profile", |ui| {
            ui.label("Name: Alex Rivera");
            ui.label("Lifetime spend: $4,892");
            ui.label("Orders: 17");
            ui.label("Tags: wholesale, early-access");
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_product_detail_slot(
    ui: &mut Ui,
    page: &ValidationResolvedPage,
    slot_name: &str,
    state: &mut ValidationWorkspaceState,
) {
    let product_id = page.parameter_value().unwrap_or("unknown-product");
    match slot_name {
        "header" => section(ui, "Product detail", |ui| {
            ui.label(RichText::new(product_id).monospace());
            if let Some(badge) = page.parameter_badge() {
                ui.small(badge);
            }
        }),
        "details" => section(ui, "Editable fields", |ui| {
            ui.label("Title");
            ui.label("Description");
            ui.label("Primary image");
            ui.label("Price");
            ui.label("Inventory");
            ui.separator();
            ui.small("This region owns scroll for long product forms.");
        }),
        "actions" => section(ui, "Actions", |ui| {
            if ui.button("Return to Products").clicked() {
                if let Some(landing_page) = page.landing_page() {
                    state.navigation_mut().select_static_page(landing_page);
                }
            }
            if ui.button("Close this detail page").clicked() {
                if let Some(handle) = page.handle() {
                    state.navigation_mut().close_dynamic_page(handle);
                    state.push_toast(format!("Closed {product_id}"));
                }
            }
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn render_order_detail_slot(
    ui: &mut Ui,
    page: &ValidationResolvedPage,
    slot_name: &str,
    state: &mut ValidationWorkspaceState,
) {
    let order_id = page.parameter_value().unwrap_or("unknown-order");
    match slot_name {
        "header" => section(ui, "Order detail", |ui| {
            ui.label(RichText::new(order_id).monospace());
            ui.label("Customer: Alex Rivera");
        }),
        "timeline" => section(ui, "Timeline", |ui| {
            ui.label("Order placed");
            ui.label("Payment review triggered");
            ui.label("Warehouse hold released");
            ui.label("Carrier label purchased");
            ui.separator();
            if ui.button("Back to Orders").clicked() {
                if let Some(landing_page) = page.landing_page() {
                    state.navigation_mut().select_static_page(landing_page);
                }
            }
        }),
        _ => fallback_slot(ui, slot_name),
    }
}

fn product_row(
    ui: &mut Ui,
    product_id: &str,
    label: &str,
    price: &str,
    state: &mut ValidationWorkspaceState,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(product_id).monospace());
        ui.label(label);
        ui.label(price);
        if ui.button("Open detail").clicked() {
            let request = ValidationDynamicPageRequest::product_detail(product_id)
                .expect("product row should produce a valid typed request");
            state
                .navigation_mut()
                .open_dynamic_page(request)
                .expect("product row request should open");
            state.push_toast(format!("Opened product detail for {product_id}"));
        }
    });
}

fn order_row(ui: &mut Ui, order_id: &str, status: &str, state: &mut ValidationWorkspaceState) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(order_id).monospace());
        ui.label(status);
        if ui.button("Inspect").clicked() {
            let request = ValidationDynamicPageRequest::order_detail(order_id)
                .expect("order row should produce a valid typed request");
            state
                .navigation_mut()
                .open_dynamic_page(request)
                .expect("order row request should open");
            state.push_toast(format!("Opened order detail for {order_id}"));
        }
    });
}

fn section(ui: &mut Ui, title: &str, add_body: impl FnOnce(&mut Ui)) {
    Frame::group(ui.style())
        .stroke(Stroke::new(
            1.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong());
            ui.add_space(8.0);
            add_body(ui);
        });
}

fn fallback_slot(ui: &mut Ui, slot_name: &str) {
    section(ui, "Unbound slot", |ui| {
        ui.label(format!(
            "No sample renderer is registered for slot '{slot_name}'."
        ));
    });
}
