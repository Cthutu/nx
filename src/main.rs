use pixels::{Pixels, SurfaceTexture};
use pixels_u32::PixelsExt;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

type Result<T> = anyhow::Result<T>;

struct EmulatorState {
    event_loop: EventLoop<()>,
    window: Window,
    scale: u32,
    pixels: Pixels,
}

impl EmulatorState {
    fn new() -> Result<Self> {
        const SCALE: u32 = 1;
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Nx Emulator")
            .with_inner_size(PhysicalSize {
                width: 800 * SCALE,
                height: 600 * SCALE,
            })
            .build(&event_loop)?;
        let pixels = {
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(size.width, size.height, surface)?
        };

        Ok(EmulatorState {
            event_loop,
            window,
            scale: 1,
            pixels,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let mut state = EmulatorState::new()?;
    state.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, window_id } if window_id == state.window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        resize(&mut state.pixels, state.scale, size.width, size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        resize(
                            &mut state.pixels,
                            state.scale,
                            new_inner_size.width,
                            new_inner_size.height,
                        );
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                draw(&mut state.pixels);
            }
            _ => {}
        }
    });
}

fn resize(pixels: &mut Pixels, scale: u32, width: u32, height: u32) {
    pixels.resize_surface(width * scale, height * scale);
    pixels.resize_buffer(width, height);
}

fn draw(pixels: &mut Pixels) {
    let frame = pixels.get_frame_u32();

    frame.iter_mut().for_each(|pixel| {
        *pixel = 0xff302010;
    });

    pixels.render().unwrap();
}
