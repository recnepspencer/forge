use crate::commands::ShellCommandRegistry;

pub const VALIDATION_SAMPLE_MODULE_PATH: &str = "validation/main.wui";
pub const VALIDATION_SAMPLE_SOURCE: &str = r#"
app ShopifyAdminApp {
    theme ShopifyAdminTheme
    workspace AdminWorkspace
}

workspace AdminWorkspace {
    shell {
        topbar AdminTopbar
        rail AdminPrimaryRail
        page_host AdminPageHost
        inspector AdminInspectorDock
        status AdminStatusBar
        overlays [CommandPaletteOverlay, GlobalSearchOverlay]
        toasts AdminToastCenter
    }

    pages [OverviewPage, ProductsPage, OrdersPage, CustomersPage]
    dynamic_pages [
        ProductDetailPage(product_id: ProductId),
        OrderDetailPage(order_id: OrderId)
    ]
}

page OverviewPage {
    title "Overview"
    runtime OverviewRuntime
    layout OverviewLayout
    content OverviewContent
}

page ProductsPage {
    title "Products"
    runtime ProductsRuntime
    layout ProductsLayout
    content ProductsContent
}

page OrdersPage {
    title "Orders"
    runtime OrdersRuntime
    layout OrdersLayout
    content OrdersContent
}

page CustomersPage {
    title "Customers"
    runtime CustomersRuntime
    layout CustomersLayout
    content CustomersContent
}

page ProductDetailPage(product_id: ProductId) {
    title "Product"
    runtime ProductDetailRuntime
    layout ProductDetailLayout
    content ProductDetailContent
}

page OrderDetailPage(order_id: OrderId) {
    title "Order"
    runtime OrderDetailRuntime
    layout OrderDetailLayout
    content OrderDetailContent
}

runtime OverviewRuntime {}
layout OverviewLayout {
    column {
        row height fit {
            slot summary
        }

        row height fill scroll_owner {
            slot evidence
        }

        row height fit {
            slot status
        }
    }
}
content OverviewContent {}

runtime ProductsRuntime {}
layout ProductsLayout {
    column {
        row height fit {
            slot toolbar
        }

        row height fill {
            column width clamp(min: rail.md, preferred: share(2), max: rail.xl) scroll_owner {
                slot filters
            }

            column width fill {
                row height ratio(3, 5) scroll_owner {
                    slot collection
                }

                row height share(2) scroll_owner {
                    slot activity
                }
            }

            column width clamp(min: inspector.md, preferred: share(2), max: inspector.xl) resizable restore {
                slot inspector
            }
        }

        row height fit {
            slot status
        }
    }
}
content ProductsContent {}

runtime OrdersRuntime {}
layout OrdersLayout {
    column {
        row height fit {
            slot filters
        }

        row height fill {
            slot queue
        }
    }
}
content OrdersContent {}

runtime CustomersRuntime {}
layout CustomersLayout {
    row {
        column width share(2) scroll_owner {
            slot segments
        }

        column width fill {
            slot profile
        }
    }
}
content CustomersContent {}

runtime ProductDetailRuntime {}
layout ProductDetailLayout {
    column {
        row height fit {
            slot header
        }

        row height fill scroll_owner {
            slot details
        }

        row height fit {
            slot actions
        }
    }
}
content ProductDetailContent {}

runtime OrderDetailRuntime {}
layout OrderDetailLayout {
    column {
        row height fit {
            slot header
        }

        row height fill scroll_owner {
            slot timeline
        }
    }
}
content OrderDetailContent {}

appearance ShopifyAdminTheme {}
"#;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringPage {
    title: &'static str,
    runtime: &'static str,
    layout: &'static str,
    content: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationDynamicPage {
    title: &'static str,
    parameter_name: &'static str,
    parameter_type: &'static str,
    runtime: &'static str,
    layout: &'static str,
    content: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationWorkspaceShellSample {
    topbar: &'static str,
    rail: &'static str,
    page_host: &'static str,
    inspector: &'static str,
    status: &'static str,
    overlays: &'static [&'static str],
    toasts: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringSample {
    app_name: &'static str,
    workspace_name: &'static str,
    workspace_title: &'static str,
    theme_name: &'static str,
    shell: ValidationWorkspaceShellSample,
    pages: &'static [ValidationAuthoringPage],
    dynamic_pages: &'static [ValidationDynamicPage],
    commands: ShellCommandRegistry,
}

pub const VALIDATION_AUTHORING_SAMPLE: ValidationAuthoringSample = ValidationAuthoringSample {
    app_name: "ShopifyAdminApp",
    workspace_name: "AdminWorkspace",
    workspace_title: "Shopify Admin",
    theme_name: "ShopifyAdminTheme",
    shell: ValidationWorkspaceShellSample {
        topbar: "AdminTopbar",
        rail: "AdminPrimaryRail",
        page_host: "AdminPageHost",
        inspector: "AdminInspectorDock",
        status: "AdminStatusBar",
        overlays: &["CommandPaletteOverlay", "GlobalSearchOverlay"],
        toasts: "AdminToastCenter",
    },
    pages: &[
        ValidationAuthoringPage {
            title: "OverviewPage",
            runtime: "OverviewRuntime",
            layout: "OverviewLayout",
            content: "OverviewContent",
        },
        ValidationAuthoringPage {
            title: "ProductsPage",
            runtime: "ProductsRuntime",
            layout: "ProductsLayout",
            content: "ProductsContent",
        },
        ValidationAuthoringPage {
            title: "OrdersPage",
            runtime: "OrdersRuntime",
            layout: "OrdersLayout",
            content: "OrdersContent",
        },
        ValidationAuthoringPage {
            title: "CustomersPage",
            runtime: "CustomersRuntime",
            layout: "CustomersLayout",
            content: "CustomersContent",
        },
    ],
    dynamic_pages: &[
        ValidationDynamicPage {
            title: "ProductDetailPage",
            parameter_name: "product_id",
            parameter_type: "ProductId",
            runtime: "ProductDetailRuntime",
            layout: "ProductDetailLayout",
            content: "ProductDetailContent",
        },
        ValidationDynamicPage {
            title: "OrderDetailPage",
            parameter_name: "order_id",
            parameter_type: "OrderId",
            runtime: "OrderDetailRuntime",
            layout: "OrderDetailLayout",
            content: "OrderDetailContent",
        },
    ],
    commands: ShellCommandRegistry::DEFAULT,
};

impl ValidationAuthoringSample {
    pub fn app_name(self) -> &'static str {
        self.app_name
    }

    pub fn workspace_name(self) -> &'static str {
        self.workspace_name
    }

    pub fn workspace_title(self) -> &'static str {
        self.workspace_title
    }

    pub fn theme_name(self) -> &'static str {
        self.theme_name
    }

    pub fn shell(self) -> ValidationWorkspaceShellSample {
        self.shell
    }

    pub fn source_text(self) -> &'static str {
        VALIDATION_SAMPLE_SOURCE
    }

    pub fn pages(self) -> &'static [ValidationAuthoringPage] {
        self.pages
    }

    pub fn dynamic_pages(self) -> &'static [ValidationDynamicPage] {
        self.dynamic_pages
    }

    pub fn commands(self) -> ShellCommandRegistry {
        self.commands
    }
}

impl ValidationWorkspaceShellSample {
    pub fn topbar(self) -> &'static str {
        self.topbar
    }

    pub fn rail(self) -> &'static str {
        self.rail
    }

    pub fn page_host(self) -> &'static str {
        self.page_host
    }

    pub fn inspector(self) -> &'static str {
        self.inspector
    }

    pub fn status(self) -> &'static str {
        self.status
    }

    pub fn overlays(self) -> &'static [&'static str] {
        self.overlays
    }

    pub fn toasts(self) -> &'static str {
        self.toasts
    }
}

impl ValidationAuthoringPage {
    pub fn title(self) -> &'static str {
        self.title
    }

    pub fn runtime(self) -> &'static str {
        self.runtime
    }

    pub fn layout(self) -> &'static str {
        self.layout
    }

    pub fn content(self) -> &'static str {
        self.content
    }
}

impl ValidationDynamicPage {
    pub fn title(self) -> &'static str {
        self.title
    }

    pub fn parameter_name(self) -> &'static str {
        self.parameter_name
    }

    pub fn parameter_type(self) -> &'static str {
        self.parameter_type
    }

    pub fn runtime(self) -> &'static str {
        self.runtime
    }

    pub fn layout(self) -> &'static str {
        self.layout
    }

    pub fn content(self) -> &'static str {
        self.content
    }
}
