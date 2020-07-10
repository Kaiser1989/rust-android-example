#![cfg(target_os = "android")]


//////////////////////////////////////////////////
// Moduls

mod support;


//////////////////////////////////////////////////
// Using

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext};
use glutin::platform::android::{AndroidEventLoop, AndroidContextExt};


//////////////////////////////////////////////////
// Entry

#[ndk_glue::main(backtrace)]
pub fn main() {
    let mut el = AndroidEventLoop::new();
    let mut context_and_handle: Option<ContextAndHandle> = None;

    let mut running = true;
    while running {
        el.run_return(|event, el, control_flow| {
            *control_flow = ControlFlow::Exit;
    
            match event {
                Event::LoopDestroyed => {},
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        if let Some(context_and_handle) = context_and_handle.as_ref() {
                            context_and_handle.context.resize(physical_size);
                        }
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        running = false;
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    if let Some(context_and_handle) = context_and_handle.as_ref() {
                        context_and_handle.gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                        context_and_handle.context.swap_buffers().unwrap();
                    }
                },
                Event::Suspended => {
                    if let Some(context_and_handle) = context_and_handle.as_ref() {
                        context_and_handle.context.suspend();
                    }
                },
                Event::Resumed => {
                    if context_and_handle.is_none() { 
                        context_and_handle = init_context(el);
                    }
                    if let Some(context_and_handle) = context_and_handle.as_ref() {
                        context_and_handle.context.resume();
                    }
                },
                _ => (),
            }
        });
    }
}

struct ContextAndHandle {
    context: WindowedContext<PossiblyCurrent>,
    gl: support::Gl,
}

fn init_context(el: &EventLoopWindowTarget<()>) -> Option<ContextAndHandle> {
    if ndk_glue::native_window().is_some() {
        let wb = WindowBuilder::new().with_title("A fantastic window!");
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)))
            .with_gl_debug_flag(false)
            .with_srgb(false)
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let context = unsafe { context.make_current().unwrap() };

        println!("Pixel format of the window's GL context: {:?}", context.get_pixel_format());
        let gl = support::load(&context.context());

        Some(ContextAndHandle { context, gl })
    } else {
        None
    }
}