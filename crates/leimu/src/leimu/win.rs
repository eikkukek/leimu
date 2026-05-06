use std::time;

use core::{
    ops::{Deref, DerefMut},
    fmt::{self, Display},
    hash::{self, Hash},
    borrow::Borrow,
};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::keyboard::{PhysicalKey, Key};
use ahash::AHashMap;
use leimu_mem::vec::Vec32;

use crate::{
    or_flag,
    sync::{Arc, atomic::{self, AtomicU64}},
    error::*,
};

use super::*;

pub use winit::window::WindowButtons;
pub use winit::window::Icon as WindowIcon;
pub use winit::keyboard::KeyCode;
pub use winit::event::MouseButton;
pub use winit::window::CursorIcon;
pub use winit::monitor::MonitorHandle;

#[derive(Clone, Copy, Debug)]
pub struct WindowId(pub(super) winit::window::WindowId, pub(super) gpu::SurfaceId);

impl Display for WindowId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}+{}", self.0, self.1)
    }
}

impl WindowId {

    /// Gets the [`surface id`][1] of the [`window`][2].
    ///
    /// [1]: gpu::SurfaceId
    /// [2]: Window
    #[inline]
    pub fn surface_id(&self) -> gpu::SurfaceId {
        self.1
    }
}

impl PartialEq for WindowId {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for WindowId {}

impl Hash for WindowId {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Borrow<winit::window::WindowId> for WindowId {

    #[inline]
    fn borrow(&self) -> &winit::window::WindowId {
        &self.0
    }
}

use winit::window::Window as WinitWindow;

pub(super) struct WinitHandle {
    window: WinitWindow,
    size: AtomicU64,
}

impl HasDisplayHandle for WinitHandle {

    fn display_handle(&self) -> core::result::Result<
        raw_window_handle::DisplayHandle<'_>,
        raw_window_handle::HandleError
    > {
        self.window.display_handle()
    }
}


impl HasWindowHandle for WinitHandle {

    fn window_handle(&self) -> core::result::Result<
        raw_window_handle::WindowHandle<'_>,
        raw_window_handle::HandleError
    > {
        self.window.window_handle()
    }
}

unsafe impl gpu::VulkanWindow for WinitHandle {
    
    fn inner_size(&self) -> (u32, u32) {
        let size = self.size.load(atomic::Ordering::Acquire);
        (
            size as u32,
            (size >> 32) as u32,
        )
    }
}

impl Deref for WinitHandle {

    type Target = WinitWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

use winit::dpi::LogicalSize;

#[derive(Clone)]
pub struct WindowAttributes {
    title: String,
    size: [u32; 2],
    enabled_buttons: win::WindowButtons,
    icon: Option<win::WindowIcon>,
    attr: u32,
}

/// Creates default [`WindowAttributes`].
pub fn default_attributes() -> WindowAttributes {
    WindowAttributes::new()
}

impl WindowAttributes {

    const RESIZABLE: u32 = 0x1;
    const MAXIMIZED: u32 = 0x2;
    const TRANSPARENT: u32 = 0x4;
    const BLUR: u32 = 0x8;
    const DECORATIONS: u32 = 0x10;

    /// Creates [`WindowAttributes`] with default values.
    #[inline]
    pub(super) fn new() -> Self
    {
        Self {
            title: String::new(),
            size: [100, 100],
            enabled_buttons: win::WindowButtons::all(),
            icon: None,
            attr: Self::DECORATIONS,
        }
    }

    /// Sets the `title` of the window.
    ///
    /// The default is an empty string.
    #[inline]
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the size of the window's content area.
    ///
    /// The default is `[100, 100]`
    #[inline]
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = [width, height];
        self
    }

    /// Sets whether the window is resizeble or not.
    ///
    /// The default is `false`.
    #[inline]
    pub fn with_resizable(mut self, value: bool) -> Self {
        self.attr &= !Self::RESIZABLE;
        or_flag!(self.attr, Self::RESIZABLE, value);
        self
    }

    /// Requests that the window is maximized upon creation.
    ///
    /// The default is `false`
    #[inline]
    pub fn with_maximized(mut self, value: bool) -> Self {
        self.attr &= !Self::MAXIMIZED;
        or_flag!(self.attr, Self::MAXIMIZED, value);
        self
    }

    /// Sets whether the background of the window should be transparent.
    ///
    /// If this is set to `true`, alpha values different than `1.0` on the swapchain image will
    /// produce a transparent window.
    ///
    /// The default is `false`.
    #[inline]
    pub fn with_transparent(mut self, value: bool) -> Self {
        self.attr &= !Self::TRANSPARENT;
        or_flag!(self.attr, Self::TRANSPARENT, value);
        self
    }

    /// Sets whether the background of the window should be blurred.
    ///
    /// The default is `false`.
    #[inline]
    pub fn with_blur(mut self, value: bool) -> Self {
        self.attr &= !Self::BLUR;
        or_flag!(self.attr, Self::BLUR, value);
        self
    }

    /// Sets whether the window should have a border, a title bar, etc.
    ///
    /// The default is `false`.
    #[inline]
    pub fn with_decorations(mut self, value: bool) -> Self {
        self.attr &= !Self::DECORATIONS;
        or_flag!(self.attr, Self::DECORATIONS, value);
        self
    }

    /// Sets the enabled window buttons
    ///
    /// The default is [`WindowButtons::all`].
    ///
    /// Primarily effective on Windows systems.
    #[inline]
    pub fn with_enabled_buttons(mut self, buttons: win::WindowButtons) -> Self {
        self.enabled_buttons = buttons;
        self
    }

    /// Sets the window icon.
    ///
    /// The default is `None`
    #[inline]
    pub fn with_window_icon(mut self, icon: impl Into<Option<win::WindowIcon>>) -> Self {
        self.icon = icon.into();
        self
    }

    #[inline]
    pub fn resizable(&self) -> bool {
        self.attr & Self::RESIZABLE == Self::RESIZABLE
    }

    #[inline]
    pub fn maximized(&self) -> bool {
        self.attr & Self::MAXIMIZED == Self::MAXIMIZED
    }

    #[inline]
    pub fn transparent(&self) -> bool {
        self.attr & Self::TRANSPARENT == Self::TRANSPARENT
    }

    #[inline]
    pub fn blur(&self) -> bool {
        self.attr & Self::BLUR == Self::BLUR
    }

    #[inline]
    pub fn decorations(&self) -> bool {
        self.attr & Self::DECORATIONS == Self::DECORATIONS
    }

    #[inline]
    pub fn enabled_buttons(&self) -> win::WindowButtons {
        self.enabled_buttons
    }

    #[inline]
    pub fn icon(&self) -> Option<&win::WindowIcon> {
        self.icon.as_ref()
    }

    #[inline]
    pub(crate) fn into_winit_attr(self) -> winit::window::WindowAttributes {
        WinitWindow::default_attributes()
            .with_resizable(self.resizable())
            .with_maximized(self.maximized())
            .with_transparent(self.transparent())
            .with_blur(self.blur())
            .with_decorations(self.decorations())
            .with_enabled_buttons(self.enabled_buttons)
            .with_window_icon(self.icon)
            .with_title(self.title)
            .with_inner_size(LogicalSize::new(
                self.size[0],
                self.size[1],
            ))
            .with_min_inner_size(LogicalSize::new(1.0, 1.0))
    }
}

#[derive(Default, Clone, Copy)]
pub struct InputState {
    pub(super) pressed: bool,
    pub(super) released: bool,
    pub(super) held: bool,
    pub(super) repeat: bool,
}

impl InputState {

    #[inline]
    pub fn pressed(self) -> bool {
        self.pressed
    }

    #[inline]
    pub fn released(self) -> bool {
        self.released
    }

    #[inline]
    pub fn held(self) -> bool {
        self.held
    }

    #[inline]
    pub fn repeat(self) -> bool {
        self.repeat
    }
}

pub struct Window {
    gpu: gpu::Gpu,
    pub(super) surface_id: gpu::SurfaceId,
    pub(super) handle: Arc<WinitHandle>,
    physical_keys: AHashMap<PhysicalKey, InputState>,
    logical_keys: AHashMap<Key, InputState>,
    mouse_buttons: AHashMap<MouseButton, InputState>,
    pub(super) input_text: Vec32<(KeyCode, winit::keyboard::SmolStr)>,
    size: (u32, u32),
    cursor_position: (f64, f64),
    mouse_scroll_delta_pixels: (f64, f64),
    mouse_scroll_line_delta: (f32, f32),
    pub(super) current_cursor: CursorIcon,
    pub(super) flags: u32,
}

impl Window {

    pub(super) const CURSOR_MOVED: u32 = 0x1;
    pub(super) const CURSOR_SET: u32 = 0x2;
    pub(super) const TRANSPARENT: u32 = 0x4;
    pub(super) const TRANSPARENT_SET: u32 = 0x8;
    pub(super) const SHOULD_CLOSE: u32 = 0x10;

    pub(super) fn new(
        gpu: gpu::Gpu,
        window: WinitWindow,
        is_transparent: bool,
    ) -> Result<Self>
    {
        let size = window.inner_size();
        let window = Arc::new(WinitHandle {
            window,
            size: AtomicU64::new(size.width as u64 | (size.height as u64) << 32),
        });
        let mut flags = 0;
        or_flag!(flags, Self::TRANSPARENT, is_transparent);
        Ok(Self {
            surface_id: gpu.create_surface(window.clone())?,
            handle: window,
            gpu,
            physical_keys: AHashMap::default(),
            logical_keys: AHashMap::default(),
            mouse_buttons: AHashMap::default(),
            input_text: Default::default(),
            size: (size.width, size.height),
            cursor_position: (0.0, 0.0),
            mouse_scroll_delta_pixels: (0.0, 0.0),
            mouse_scroll_line_delta: (0.0, 0.0),
            current_cursor: CursorIcon::Default,
            flags,
        })
    }

    #[inline]
    pub(super) fn should_close(&self) -> bool {
        self.flags & Self::SHOULD_CLOSE == Self::SHOULD_CLOSE
    }

    #[inline]
    pub fn surface_id(&self) -> gpu::SurfaceId {
        self.surface_id
    }

    #[inline]
    pub fn handle(&self) -> &(impl HasWindowHandle + HasDisplayHandle) {
        &self.handle
    }

    /// Closes the window.
    ///
    /// The closed window will still be valid until the end of current frame.
    #[inline]
    pub fn close(&mut self) {
        self.flags |= Self::SHOULD_CLOSE;
    }

    #[inline]
    pub fn is_transparent(&self) -> bool {
        self.flags & Self::TRANSPARENT == Self::TRANSPARENT
    }

    #[inline]
    pub fn set_transparent(&mut self, value: bool) {
        self.flags &= !Self::TRANSPARENT;
        self.flags |= Self::TRANSPARENT_SET;
        or_flag!(self.flags, Self::TRANSPARENT, value);
    }

    #[inline]
    pub(super) fn transparent_set(&self) -> bool {
        self.flags & Self::TRANSPARENT_SET == Self::TRANSPARENT_SET
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    #[inline]
    pub fn size_f32(&self) -> (f32, f32) {
        (self.size.0 as f32, self.size.1 as f32)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        self.size.0 as f64 /
        self.size.1 as f64
    }

    #[inline]
    pub fn cursor_moved(&self) -> bool {
        self.flags & Self::CURSOR_MOVED == Self::CURSOR_MOVED
    }

    #[inline]
    pub(super) fn cursor_set(&self) -> bool {
        self.flags & Self::CURSOR_SET == Self::CURSOR_SET
    }

    #[inline]
    pub fn set_cursor_hide(&self, hide: bool) {
        self.handle.set_cursor_visible(!hide);
    }

    #[inline]
    pub fn set_cursor(&mut self, cursor: CursorIcon) {
        self.current_cursor = cursor;
        self.flags |= Self::CURSOR_SET;
    } 

    #[inline]
    pub fn cursor_position(&self) -> (f64, f64) {
        self.cursor_position
    }

    #[inline]
    pub fn cursor_position_f32(&self) -> (f32, f32) {
        (
            self.cursor_position.0 as f32,
            self.cursor_position.1 as f32,
        )
    }

    #[inline]
    pub fn normalized_cursor_position(&self) -> (f64, f64) {
        (
            self.cursor_position.0 / self.size.0 as f64,
            self.cursor_position.1 / self.size.1 as f64,
        )
    }

    #[inline]
    pub fn normalized_cursor_position_f32(&self) -> (f32, f32) {
        (
            self.cursor_position.0 as f32 / self.size.0 as f32,
            self.cursor_position.1 as f32 / self.size.1 as f32,
        )
    }

    /// The difference in mouse scroll state this frame, in units of [`f64`] pixels.
    ///
    /// Positive values indicate that the scrolled content should move right/down.
    #[inline]
    pub fn mouse_scroll_delta_pixels(&self) -> (f64, f64) {
        self.mouse_scroll_delta_pixels
    }

    /// The difference in mouse scroll state this frame, in units of [`f32`] pixels. 
    ///
    /// Positive values indicate that the scrolled content should move right/down.
    #[inline]
    pub fn mouse_scroll_delta_pixels_f32(&self) -> (f32, f32) {
        (
            self.mouse_scroll_delta_pixels.0 as f32,
            self.mouse_scroll_delta_pixels.1 as f32,
        )
    }

    /// The difference in mouse scroll state this frame, in units of rows/lines.
    ///
    /// Positive values indicate that the scrolled content should move right/down.
    #[inline]
    pub fn mouse_scroll_delta_lines(&self) -> (f32, f32) {
        self.mouse_scroll_line_delta
    }

    #[inline]
    pub fn key_state(&self, key: KeyCode) -> InputState {
        self.physical_keys
            .get(&PhysicalKey::Code(key))
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    pub fn key_value(&self, key: KeyCode) -> f32 {
        self.physical_keys
            .get(&PhysicalKey::Code(key))
            .copied()
            .unwrap_or_default()
            .held as u32 as f32
    }

    #[inline]
    pub fn get_input_text(&self) -> (usize, impl Iterator<Item = (KeyCode, &str)>) {
        (
            self.input_text.len() as usize,
            self.input_text
                .iter()
                .map(|(key, text)| (*key, text.as_str()))
        )
    }

    #[inline]
    pub fn mouse_button_state(&self, button: MouseButton) -> InputState {
        self.mouse_buttons
            .get(&button)
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    pub fn mouse_button_value(&self, button: MouseButton) -> f32 {
        self.mouse_buttons
            .get(&button)
            .copied()
            .unwrap_or_default()
            .held as u32 as f32
    }

    pub(super) fn process_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.cursor_position = (position.x, position.y);
                self.flags |= Self::CURSOR_MOVED;
            },
            WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.mouse_scroll_line_delta = (x, y);
                    },
                    MouseScrollDelta::PixelDelta(d) => {
                        self.mouse_scroll_delta_pixels = (d.x, d.y);
                    }
                };
            },
            WindowEvent::KeyboardInput {
                device_id: _, 
                event: KeyEvent {
                    physical_key, logical_key, text, location: _, state, repeat, ..
                },
                is_synthetic: _ 
            } => {
                let phys = self.physical_keys
                    .entry(physical_key)
                    .or_default();
                phys.pressed = state == ElementState::Pressed;
                phys.released = state == ElementState::Released;
                phys.held = state != ElementState::Released;
                phys.repeat = repeat;
                if let Some(text) = text && (phys.pressed || repeat) {
                    self.input_text.push((
                        match physical_key {
                            PhysicalKey::Code(c) => c,
                            PhysicalKey::Unidentified(_) => KeyCode::Backspace,
                        },
                        text,
                    ));
                }
                let logic = self.logical_keys.entry(logical_key).or_default();
                logic.pressed = state == ElementState::Pressed;
                logic.released = state == ElementState::Released;
                logic.held = state != ElementState::Released;
            },
            WindowEvent::MouseInput { device_id: _, state, button } => {
                let button = self.mouse_buttons.entry(button).or_default();
                button.pressed = state == ElementState::Pressed;
                button.released = state == ElementState::Released;
                button.held = state != ElementState::Released;
            },
            WindowEvent::CloseRequested => {
                self.flags |= Self::SHOULD_CLOSE;
            },
            WindowEvent::Resized(size) => {
                self.size = (size.width, size.height);
                self.handle.size.store(
                    size.width as u64 | (size.height as u64) << 32,
                    atomic::Ordering::Release,
                );
                self.gpu.request_swapchain_update(
                    self.surface_id,
                    self.size,
                ).ok();
            },
            _ => {},
        }
    }

    pub(super) fn reset_input(&mut self) {
        self.mouse_scroll_delta_pixels = (0.0, 0.0); 
        self.mouse_scroll_line_delta = (0.0, 0.0);
        self.physical_keys.retain(|_, v| {
            v.pressed = false;
            v.released = false;
            v.held
        });
        self.logical_keys.retain(|_, v| {
            v.pressed = false;
            v.released = false;
            v.held
        });
        self.mouse_buttons.retain(|_, v| {
            v.pressed = false;
            v.released = false;
            v.held
        });
    }
}

impl Drop for Window {

    fn drop(&mut self) {
        self.gpu.destroy_surface(self.surface_id).ok();
    }
}

pub struct WindowContext<'a> {
    pub(super) window: &'a mut Window,
    pub(super) delta_time: time::Duration,
}

impl<'a> WindowContext<'a> {

    #[inline]
    pub fn delta_time(&self) -> time::Duration {
        self.delta_time
    }

    #[inline]
    pub fn delta_time_secs_f32(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}

impl<'a> Deref for WindowContext<'a> {

    type Target = Window;

    fn deref(&self) -> &Self::Target {
        self.window
    }
}

impl<'a> DerefMut for WindowContext<'a> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.window
    }
}
