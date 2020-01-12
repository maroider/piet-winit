use direct2d::RenderTarget;
use piet::RenderContext;
use piet_direct2d::D2DRenderContext;
use piet_test::draw_test_picture;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use piet_winit_direct2d::create_render_target;

// TODO: Improve error handling

// TODO: Figure out how to make `Ctrl+C` not exit the application with `exit code: 0x80000003`

fn main() {
    let test_picture_number = std::env::args()
        .skip(1)
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    // Create the EventLoop and Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Create the D2D factory
    let d2d = direct2d::factory::Factory::new().unwrap();
    let dwrite = directwrite::factory::Factory::new().unwrap();

    // Create the render target
    let mut render_target = create_render_target(&window, &d2d);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                // Create a new render target with the new dimensions for the window
                render_target = create_render_target(&window, &d2d);
            }
            Event::RedrawRequested(_) => {
                render_target.begin_draw();
                let mut piet_context = D2DRenderContext::new(&d2d, &dwrite, &mut render_target);

                // Draw stuff
                draw_test_picture(&mut piet_context, test_picture_number).unwrap();

                piet_context.finish().unwrap();
                render_target.end_draw().unwrap();
            }
            _ => {}
        }
    });
}
