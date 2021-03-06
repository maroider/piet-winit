use std::mem;

use direct2d::{
    enums::{AlphaMode, AntialiasMode, RenderTargetType},
    factory::Factory,
    render_target::{GenericRenderTarget, HwndRenderTarget, RenderTarget},
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winapi::{
    shared::windef::{HWND, RECT},
    um::winuser::GetClientRect,
};
use winit::window::Window;

// TODO: Improve error handling

/// Create a render target that covers the entire window
///
/// This function should be called again whenever the window's size chanes
pub fn create_render_target(window: &Window, d2d: &Factory) -> GenericRenderTarget {
    let hwnd = hwnd_from_window(window);

    let mut rect: RECT = unsafe { mem::zeroed() };
    if unsafe { GetClientRect(hwnd, &mut rect) } == 0 {
        panic!("GetClientRect failed")
    }
    let width = (rect.right - rect.left) as u32;
    let height = (rect.bottom - rect.top) as u32;

    let mut render_target = HwndRenderTarget::create(&d2d)
        .with_hwnd(hwnd)
        .with_target_type(RenderTargetType::Default)
        .with_alpha_mode(AlphaMode::Premultiplied)
        .with_pixel_size(width, height)
        .build()
        .unwrap();
    render_target.set_antialias_mode(AntialiasMode::PerPrimitive);

    render_target.as_generic()
}

fn hwnd_from_window(window: &Window) -> HWND {
    match window.raw_window_handle() {
        RawWindowHandle::Windows(handle) => handle.hwnd as HWND,
        _ => unreachable!(),
    }
}
