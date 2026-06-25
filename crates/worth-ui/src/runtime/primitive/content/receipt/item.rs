use super::icon_paint_command::WorthUiPrimitiveContentIconPaintCommand;
use super::image_asset_receipt::WorthUiPrimitiveImageAssetReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentItemKind {
    Text,
    Icon,
    Image,
    Spacer,
    Badge,
    Divider,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentIconRenderPosture {
    NativeVector,
    SymbolFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitiveContentItem {
    Text(WorthUiPrimitiveTextContentItem),
    Icon(WorthUiPrimitiveIconContentItem),
    Image(WorthUiPrimitiveImageContentItem),
    Spacer(WorthUiPrimitiveSpacerContentItem),
    Badge(WorthUiPrimitiveBadgeContentItem),
    Divider(WorthUiPrimitiveDividerContentItem),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveTextContentItem {
    text: String,
    size_token: String,
    size_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveIconContentItem {
    icon_id: String,
    source_kind: String,
    provider: String,
    source_key: String,
    paint_command: WorthUiPrimitiveContentIconPaintCommand,
    render_posture: WorthUiPrimitiveContentIconRenderPosture,
    size_token: String,
    size_points: f32,
    stroke_token: String,
    stroke_width_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveImageContentItem {
    asset: WorthUiPrimitiveImageAssetReceipt,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveSpacerContentItem {
    size_token: String,
    size_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveBadgeContentItem {
    text: String,
    size_token: String,
    size_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDividerContentItem {
    thickness_token: String,
    thickness_points: f32,
}

impl WorthUiPrimitiveContentItem {
    pub(crate) fn text(
        text: impl Into<String>,
        size_token: impl Into<String>,
        size_points: f32,
    ) -> Self {
        let size_points = size_points.max(1.0);
        Self::Text(WorthUiPrimitiveTextContentItem {
            text: text.into(),
            size_token: size_token.into(),
            size_points,
            baseline_points: size_points * 0.78,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn icon(
        icon_id: impl Into<String>,
        source_kind: impl Into<String>,
        provider: impl Into<String>,
        source_key: impl Into<String>,
        render_posture: WorthUiPrimitiveContentIconRenderPosture,
        size_token: impl Into<String>,
        size_points: f32,
        stroke_token: impl Into<String>,
        stroke_width_points: f32,
    ) -> Self {
        let source_key = source_key.into();
        let size_points = size_points.max(1.0);
        Self::Icon(WorthUiPrimitiveIconContentItem {
            icon_id: icon_id.into(),
            source_kind: source_kind.into(),
            provider: provider.into(),
            paint_command: WorthUiPrimitiveContentIconPaintCommand::from_source_key(&source_key),
            source_key,
            render_posture,
            size_token: size_token.into(),
            size_points,
            stroke_token: stroke_token.into(),
            stroke_width_points,
            baseline_points: size_points * 0.5,
        })
    }

    pub(crate) fn image(asset: WorthUiPrimitiveImageAssetReceipt) -> Self {
        let width_points = asset.intrinsic_width_points();
        let height_points = asset.intrinsic_height_points();
        Self::Image(WorthUiPrimitiveImageContentItem {
            asset,
            width_points,
            height_points,
            baseline_points: height_points * 0.8,
        })
    }

    pub(crate) fn spacer(size_token: impl Into<String>, size_points: f32) -> Self {
        Self::Spacer(WorthUiPrimitiveSpacerContentItem {
            size_token: size_token.into(),
            size_points,
        })
    }

    pub(crate) fn badge(
        text: impl Into<String>,
        size_token: impl Into<String>,
        size_points: f32,
    ) -> Self {
        let size_points = size_points.max(1.0);
        Self::Badge(WorthUiPrimitiveBadgeContentItem {
            text: text.into(),
            size_token: size_token.into(),
            size_points,
            baseline_points: size_points * 0.72,
        })
    }

    pub(crate) fn divider(thickness_token: impl Into<String>, thickness_points: f32) -> Self {
        Self::Divider(WorthUiPrimitiveDividerContentItem {
            thickness_token: thickness_token.into(),
            thickness_points,
        })
    }

    pub fn kind(&self) -> WorthUiPrimitiveContentItemKind {
        match self {
            Self::Text(_) => WorthUiPrimitiveContentItemKind::Text,
            Self::Icon(_) => WorthUiPrimitiveContentItemKind::Icon,
            Self::Image(_) => WorthUiPrimitiveContentItemKind::Image,
            Self::Spacer(_) => WorthUiPrimitiveContentItemKind::Spacer,
            Self::Badge(_) => WorthUiPrimitiveContentItemKind::Badge,
            Self::Divider(_) => WorthUiPrimitiveContentItemKind::Divider,
        }
    }

    pub fn width_points(&self) -> f32 {
        match self {
            Self::Text(item) => item.text().chars().count() as f32 * item.size_points() * 0.56,
            Self::Icon(item) => item.size_points(),
            Self::Image(item) => item.width_points(),
            Self::Spacer(item) => item.size_points(),
            Self::Badge(item) => {
                item.text().chars().count() as f32 * item.size_points() * 0.56 + 14.0
            }
            Self::Divider(item) => item.thickness_points(),
        }
    }

    pub fn height_points(&self) -> f32 {
        match self {
            Self::Text(item) => item.size_points(),
            Self::Icon(item) => item.size_points(),
            Self::Image(item) => item.height_points(),
            Self::Spacer(item) => item.size_points(),
            Self::Badge(item) => item.size_points() + 8.0,
            Self::Divider(item) => item.thickness_points(),
        }
    }

    pub fn baseline_points(&self) -> f32 {
        match self {
            Self::Text(item) => item.baseline_points(),
            Self::Icon(item) => item.baseline_points(),
            Self::Image(item) => item.baseline_points(),
            Self::Spacer(item) => item.size_points() * 0.5,
            Self::Badge(item) => item.baseline_points(),
            Self::Divider(item) => item.thickness_points() * 0.5,
        }
    }

    pub fn as_text(&self) -> Option<&WorthUiPrimitiveTextContentItem> {
        match self {
            Self::Text(item) => Some(item),
            _ => None,
        }
    }

    pub fn as_icon(&self) -> Option<&WorthUiPrimitiveIconContentItem> {
        match self {
            Self::Icon(item) => Some(item),
            _ => None,
        }
    }

    pub fn as_image(&self) -> Option<&WorthUiPrimitiveImageContentItem> {
        match self {
            Self::Image(item) => Some(item),
            _ => None,
        }
    }
}

impl WorthUiPrimitiveImageContentItem {
    pub fn asset(&self) -> &WorthUiPrimitiveImageAssetReceipt {
        &self.asset
    }

    pub fn asset_id(&self) -> &str {
        self.asset.asset_id()
    }

    pub fn source_kind(&self) -> &str {
        self.asset.source_kind().token()
    }

    pub fn source_key(&self) -> &str {
        self.asset.source_key()
    }

    pub fn width_points(&self) -> f32 {
        self.width_points
    }

    pub fn height_points(&self) -> f32 {
        self.height_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }
}

impl WorthUiPrimitiveTextContentItem {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn size_token(&self) -> &str {
        &self.size_token
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }
}

impl WorthUiPrimitiveIconContentItem {
    pub fn icon_id(&self) -> &str {
        &self.icon_id
    }

    pub fn source_kind(&self) -> &str {
        &self.source_kind
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn paint_command(&self) -> WorthUiPrimitiveContentIconPaintCommand {
        self.paint_command
    }

    pub fn render_posture(&self) -> WorthUiPrimitiveContentIconRenderPosture {
        self.render_posture
    }

    pub fn size_token(&self) -> &str {
        &self.size_token
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }

    pub fn stroke_token(&self) -> &str {
        &self.stroke_token
    }

    pub fn stroke_width_points(&self) -> f32 {
        self.stroke_width_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }
}

impl WorthUiPrimitiveSpacerContentItem {
    pub fn size_token(&self) -> &str {
        &self.size_token
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }
}

impl WorthUiPrimitiveBadgeContentItem {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn size_token(&self) -> &str {
        &self.size_token
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }
}

impl WorthUiPrimitiveDividerContentItem {
    pub fn thickness_token(&self) -> &str {
        &self.thickness_token
    }

    pub fn thickness_points(&self) -> f32 {
        self.thickness_points
    }
}
