#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationStaticPageId {
    Overview,
    Products,
    Orders,
    Customers,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationDynamicPageKind {
    ProductDetail,
    OrderDetail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationStaticPageDescriptor {
    id: ValidationStaticPageId,
    title: &'static str,
    summary: &'static str,
    authoring_page_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationDynamicPageTemplateDescriptor {
    kind: ValidationDynamicPageKind,
    title: &'static str,
    parameter_name: &'static str,
    landing_page: ValidationStaticPageId,
    authoring_page_name: &'static str,
}

const STATIC_PAGES: [ValidationStaticPageDescriptor; 4] = [
    ValidationStaticPageDescriptor::new(
        ValidationStaticPageId::Overview,
        "Overview",
        "Native shell overview, launch proof, and workspace summary.",
        "OverviewPage",
    ),
    ValidationStaticPageDescriptor::new(
        ValidationStaticPageId::Products,
        "Products",
        "Typed page host with repeated product-detail launch targets.",
        "ProductsPage",
    ),
    ValidationStaticPageDescriptor::new(
        ValidationStaticPageId::Orders,
        "Orders",
        "Independent status-heavy page proving shell continuity across layouts.",
        "OrdersPage",
    ),
    ValidationStaticPageDescriptor::new(
        ValidationStaticPageId::Customers,
        "Customers",
        "Alternate density-heavy content inside the same persistent workspace shell.",
        "CustomersPage",
    ),
];

const DYNAMIC_PAGE_TEMPLATES: [ValidationDynamicPageTemplateDescriptor; 2] = [
    ValidationDynamicPageTemplateDescriptor::new(
        ValidationDynamicPageKind::ProductDetail,
        "Product",
        "product_id",
        ValidationStaticPageId::Products,
        "ProductDetailPage",
    ),
    ValidationDynamicPageTemplateDescriptor::new(
        ValidationDynamicPageKind::OrderDetail,
        "Order",
        "order_id",
        ValidationStaticPageId::Orders,
        "OrderDetailPage",
    ),
];

impl ValidationStaticPageDescriptor {
    pub const fn new(
        id: ValidationStaticPageId,
        title: &'static str,
        summary: &'static str,
        authoring_page_name: &'static str,
    ) -> Self {
        Self {
            id,
            title,
            summary,
            authoring_page_name,
        }
    }

    pub fn id(self) -> ValidationStaticPageId {
        self.id
    }

    pub fn title(self) -> &'static str {
        self.title
    }

    pub fn summary(self) -> &'static str {
        self.summary
    }

    pub fn authoring_page_name(self) -> &'static str {
        self.authoring_page_name
    }
}

impl ValidationDynamicPageTemplateDescriptor {
    pub const fn new(
        kind: ValidationDynamicPageKind,
        title: &'static str,
        parameter_name: &'static str,
        landing_page: ValidationStaticPageId,
        authoring_page_name: &'static str,
    ) -> Self {
        Self {
            kind,
            title,
            parameter_name,
            landing_page,
            authoring_page_name,
        }
    }

    pub fn title(self) -> &'static str {
        self.title
    }

    pub fn parameter_name(self) -> &'static str {
        self.parameter_name
    }

    pub fn landing_page(self) -> ValidationStaticPageId {
        self.landing_page
    }

    pub fn authoring_page_name(self) -> &'static str {
        self.authoring_page_name
    }
}

pub fn static_pages() -> &'static [ValidationStaticPageDescriptor] {
    &STATIC_PAGES
}

pub fn static_page(id: ValidationStaticPageId) -> ValidationStaticPageDescriptor {
    STATIC_PAGES
        .iter()
        .copied()
        .find(|page| page.id() == id)
        .expect("static page catalog should contain every static page id")
}

pub fn dynamic_page_template(
    kind: ValidationDynamicPageKind,
) -> ValidationDynamicPageTemplateDescriptor {
    DYNAMIC_PAGE_TEMPLATES
        .iter()
        .copied()
        .find(|template| template.kind == kind)
        .expect("dynamic page catalog should contain every dynamic page kind")
}
