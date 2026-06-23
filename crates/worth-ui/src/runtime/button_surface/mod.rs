mod size;

use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiInlineContentItem, WorthUiInlineContentReceipt, WorthUiResolvedBoxStyle,
    WorthUiResolvedIcon, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::component_content::{resolve_inline_content, WorthUiInlineContentDefaults};
use super::component_visual::{
    authored_prop_number_or_text, authored_prop_text, resolve_component_box_style,
    resolve_component_box_style_with_prefix, WorthUiComponentStyleDefaults,
};
pub use size::WorthUiButtonSize;

const BUTTON_COMPONENT_ID: &str = "worth.component.button";

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiButtonFrameReceipt {
    surface_id: String,
    component_id: String,
    content: WorthUiInlineContentReceipt,
    style: WorthUiResolvedBoxStyle,
    pressed_style: WorthUiResolvedBoxStyle,
    variant: WorthUiButtonVariant,
    size: WorthUiButtonSize,
    hover_fill: String,
    pressed_fill: String,
    width_points: f32,
    height_points: f32,
    padding_x_points: f32,
    padding_y_points: f32,
    container_align: WorthUiButtonContainerAlign,
    container_padding_points: f32,
    dependency_fact: WorthUiRuntimeFactId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiButtonVariant {
    Primary,
    Secondary,
    Quiet,
    Danger,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiButtonContainerAlign {
    Start,
    Center,
    End,
    Fill,
}

impl WorthUiRuntimeHost {
    pub fn resolve_button_frame(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiButtonFrameReceipt, WorthUiButtonFrameDenial> {
        let component_id = self
            .inspect_active_authored_surface_component_id(surface_id)
            .or_else(|| {
                self.inspect_active_surface_descriptor(surface_id)
                    .map(|surface| surface.component_id().as_str())
            })
            .ok_or_else(|| WorthUiButtonFrameDenial::MissingSurface {
                surface_id: surface_id.as_str().to_owned(),
            })?;
        if component_id != BUTTON_COMPONENT_ID {
            return Err(WorthUiButtonFrameDenial::ComponentMismatch {
                surface_id: surface_id.as_str().to_owned(),
                expected_component_id: BUTTON_COMPONENT_ID.to_owned(),
                actual_component_id: component_id.to_owned(),
            });
        }

        let variant =
            parse_variant(authored_prop_text(self, surface_id, "variant").unwrap_or("primary"));
        let size = parse_size(authored_prop_text(self, surface_id, "size").unwrap_or("medium"));
        let style = resolve_component_box_style(
            self,
            surface_id,
            WorthUiComponentStyleDefaults {
                background_color: default_fill(variant),
                foreground_color: "#f7f1e8",
                icon_color: "#f7f1e8",
                border_color: "#ffffff18",
                border_width_points: 1.0,
                border_radius_points: 7.0,
            },
        );
        let pressed_fill = authored_prop_text(self, surface_id, "pressed_background_color")
            .map(str::to_owned)
            .unwrap_or_else(|| style.background_color().to_owned());
        let pressed_style = resolve_component_box_style_with_prefix(
            self,
            surface_id,
            "pressed",
            WorthUiComponentStyleDefaults {
                background_color: &pressed_fill,
                foreground_color: style.foreground_color(),
                icon_color: style.icon_color(),
                border_color: style.border_color(),
                border_width_points: style.border_width_points(),
                border_radius_points: style.border_radius_points(),
            },
        );
        let content = resolve_inline_content(
            self,
            surface_id,
            WorthUiInlineContentDefaults {
                text: "Submit",
                text_color: style.foreground_color(),
                icon_color: style.icon_color(),
                text_size_points: size.text_size(),
                icon_size_points: size.icon_size(),
                icon_stroke_width_points: size.icon_stroke_width(),
                gap_points: size.content_gap(),
            },
        );
        let align = parse_align(
            authored_prop_text(self, surface_id, "container_align").unwrap_or("center"),
        );
        let hover_fill = authored_prop_text(self, surface_id, "hover_background_color")
            .map(str::to_owned)
            .unwrap_or_else(|| style.background_color().to_owned());
        Ok(WorthUiButtonFrameReceipt {
            surface_id: surface_id.as_str().to_owned(),
            component_id: BUTTON_COMPONENT_ID.to_owned(),
            content,
            style,
            pressed_style,
            variant,
            size,
            hover_fill,
            pressed_fill,
            width_points: authored_prop_number_or_text(self, surface_id, "width")
                .as_deref()
                .and_then(|value| value.parse::<f32>().ok())
                .unwrap_or_else(|| size.default_width()),
            height_points: size.default_height(),
            padding_x_points: size.padding_x(),
            padding_y_points: size.padding_y(),
            container_align: align,
            container_padding_points: authored_prop_number_or_text(
                self,
                surface_id,
                "container_padding",
            )
            .as_deref()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(24.0),
            dependency_fact: WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiButtonFrameDenial {
    MissingSurface {
        surface_id: String,
    },
    ComponentMismatch {
        surface_id: String,
        expected_component_id: String,
        actual_component_id: String,
    },
}

impl WorthUiButtonFrameReceipt {
    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn label(&self) -> &str {
        self.content
            .items()
            .iter()
            .find_map(|item| match item {
                WorthUiInlineContentItem::Text(text) => Some(text.text()),
                WorthUiInlineContentItem::Icon(_) => None,
            })
            .unwrap_or("")
    }

    pub fn icon_id(&self) -> Option<&str> {
        self.icon().map(WorthUiResolvedIcon::icon_id)
    }

    pub fn icon_source_key(&self) -> Option<&str> {
        self.icon().map(WorthUiResolvedIcon::source_key)
    }

    pub fn icon(&self) -> Option<&WorthUiResolvedIcon> {
        self.content.items().iter().find_map(|item| match item {
            WorthUiInlineContentItem::Icon(icon) => Some(icon.icon()),
            WorthUiInlineContentItem::Text(_) => None,
        })
    }

    pub fn content(&self) -> &WorthUiInlineContentReceipt {
        &self.content
    }

    pub fn style(&self) -> &WorthUiResolvedBoxStyle {
        &self.style
    }

    pub fn pressed_style(&self) -> &WorthUiResolvedBoxStyle {
        &self.pressed_style
    }

    pub fn variant(&self) -> WorthUiButtonVariant {
        self.variant
    }

    pub fn size(&self) -> WorthUiButtonSize {
        self.size
    }

    pub fn fill(&self) -> &str {
        self.style.background_color()
    }

    pub fn text(&self) -> &str {
        self.style.foreground_color()
    }

    pub fn hover_fill(&self) -> &str {
        &self.hover_fill
    }

    pub fn pressed_fill(&self) -> &str {
        &self.pressed_fill
    }

    pub fn width_points(&self) -> f32 {
        self.width_points
    }

    pub fn height_points(&self) -> f32 {
        self.height_points
    }

    pub fn padding_x_points(&self) -> f32 {
        self.padding_x_points
    }

    pub fn padding_y_points(&self) -> f32 {
        self.padding_y_points
    }

    pub fn container_align(&self) -> WorthUiButtonContainerAlign {
        self.container_align
    }

    pub fn container_padding_points(&self) -> f32 {
        self.container_padding_points
    }

    pub fn dependency_fact(&self) -> &WorthUiRuntimeFactId {
        &self.dependency_fact
    }
}

fn parse_variant(value: &str) -> WorthUiButtonVariant {
    match value {
        "secondary" => WorthUiButtonVariant::Secondary,
        "quiet" => WorthUiButtonVariant::Quiet,
        "danger" => WorthUiButtonVariant::Danger,
        _ => WorthUiButtonVariant::Primary,
    }
}

fn parse_size(value: &str) -> WorthUiButtonSize {
    match value {
        "small" => WorthUiButtonSize::Small,
        "large" => WorthUiButtonSize::Large,
        _ => WorthUiButtonSize::Medium,
    }
}

fn parse_align(value: &str) -> WorthUiButtonContainerAlign {
    match value {
        "start" => WorthUiButtonContainerAlign::Start,
        "end" => WorthUiButtonContainerAlign::End,
        "fill" => WorthUiButtonContainerAlign::Fill,
        _ => WorthUiButtonContainerAlign::Center,
    }
}

fn default_fill(variant: WorthUiButtonVariant) -> &'static str {
    match variant {
        WorthUiButtonVariant::Primary => "#2f7de1",
        WorthUiButtonVariant::Secondary => "#2f3338",
        WorthUiButtonVariant::Quiet => "#1f2023",
        WorthUiButtonVariant::Danger => "#b84a4a",
    }
}
