use super::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RenderArea {
    pub width: u32,
    pub height: u32,
    pub offset: Offset2D,
}

impl From<RenderArea> for vk::Rect2D {

    fn from(value: RenderArea) -> Self {
        Self {
            offset: value.offset.into(),
            extent: vk::Extent2D {
                width: value.width,
                height: value.height,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClearColorValue {
    Float([f32; 4]),
    Int([i32; 4]),
    UInt([u32; 4]),
}

impl Default for ClearColorValue {

    fn default() -> Self {
        Self::Int([0; 4])
    }
}

impl From<[f32; 4]> for ClearColorValue {

    fn from(value: [f32; 4]) -> Self {
        Self::Float([
            value[0],
            value[1],
            value[2],
            value[3],
        ])
    }
}

impl From<ClearColorValue> for vk::ClearColorValue {

    fn from(value: ClearColorValue) -> Self {
        match value {
            ClearColorValue::Float(v) => {
                Self {
                    float32: v,
                }
            },
            ClearColorValue::Int(v) => {
                Self {
                    int32: v,
                }
            },
            ClearColorValue::UInt(v) => {
                Self {
                    uint32: v,
                }
            },
        }
    }
}

impl From<ClearColorValue> for vk::ClearValue {

    fn from(value: ClearColorValue) -> Self {
        Self { color: value.into() }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClearValue {
    Color(ClearColorValue),
    DepthStencil { depth: f32, stencil: u32 },
}

impl Default for ClearValue {

    fn default() -> Self {
        Self::Color(Default::default())
    }
}

impl From<ClearColorValue> for ClearValue {

    fn from(value: ClearColorValue) -> Self {
        Self::Color(value)
    }
}

impl From<ClearValue> for vk::ClearValue {

    fn from(value: ClearValue) -> Self {
        match value {
            ClearValue::Color(v) => {
                Self {
                    color: v.into(),
                }
            },
            ClearValue::DepthStencil { depth, stencil } => {
                Self {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth,
                        stencil,
                    },
                }
            },
        }
    }
}

impl From<[f32; 4]> for ClearValue {

    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self::Color(ClearColorValue::Float(value))
    }
}

#[derive(Clone, Copy)]
pub struct ResolveInfo {
    pub image_view: BareImageViewId,
    pub mode: ResolveMode,
}

impl ResolveInfo {

    #[inline(always)]
    pub fn new<Id>(
        image_view: Id,
        mode: ResolveMode,
    ) -> Self
        where Id: Into<BareImageViewId>
    {
        Self {
            image_view: image_view.into(),
            mode,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PassAttachment {
    pub image_view: BareImageViewId,
    pub resolve: Option<ResolveInfo>,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub clear_value: ClearValue,
    pub preserve_contents: bool,
}

impl PassAttachment {

    #[inline(always)]
    pub fn new<Id>(image_view: Id) -> PassAttachment
        where Id: Into<BareImageViewId>
    {
        Self {
            image_view: image_view.into(),
            resolve: None,
            load_op: Default::default(),
            store_op: Default::default(),
            clear_value: Default::default(),
            preserve_contents: true,
        }
    }

    #[inline(always)]
    pub fn with_resolve(mut self, info: impl Into<Option<ResolveInfo>>) -> Self {
        self.resolve = info.into();
        self
    }

    #[inline(always)]
    pub fn with_load_op(mut self, load_op: AttachmentLoadOp) -> Self {
        self.load_op = load_op;
        self
    }

    #[inline(always)]
    pub fn with_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        self.store_op = store_op;
        self
    }

    #[inline(always)]
    pub fn with_clear_value(mut self, value: impl Into<ClearValue>) -> Self {
        self.clear_value = value.into();
        self
    }

    #[inline(always)]
    pub fn with_preserve_contents(mut self, value: bool) -> Self {
        self.preserve_contents = value;
        self
    }
}

#[derive(Default, Clone, Copy)]
pub enum DepthStencilAttachment {
    #[default]
    None,
    Depth(PassAttachment),
    Stencil(PassAttachment),
    DepthStencil { 
        depth: PassAttachment,
        stencil: PassAttachment,
    },
}

impl DepthStencilAttachment {

    #[inline(always)]
    pub fn depth(attachment: PassAttachment) -> Self
    {
        Self::Depth(attachment)
    }

    #[inline(always)]
    pub fn stencil(attachment: PassAttachment) -> Self
    {
        Self::Stencil(attachment)
    }

    #[inline(always)]
    pub fn depth_stencil(depth: PassAttachment, stencil: PassAttachment) -> Self
    {
        Self::DepthStencil { depth, stencil, }
    }
}
