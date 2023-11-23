// main.rs
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

mod draw;
mod read_crapmap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_file = r"C:\Users\Gamer\Documents\Projekte\coding\rust\code\crapmap_rust\images\sample1.crapmap".to_string();
    let file_path = args.get(1).unwrap_or(&default_file);

    let image_data = read_crapmap::read_crapmap(file_path)
        .expect("Failed to read image data");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("CRAP Image Viewer")
        .build(&event_loop)
        .unwrap();

    let mut window_size = window.inner_size();
    let surface_texture: SurfaceTexture<Window> = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)
        .expect("Failed to create pixels context");

    println!("{}x{}", window_size.width.to_string(), window_size.height.to_string());

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Resize your pixels context if necessary
                window_size = window.inner_size();
                pixels.resize_surface(window_size.width, window_size.height);

                println!("Resized {}x{}", window_size.width.to_string(), window_size.height.to_string());
                // Call draw_scaled with the new size
                //draw::draw_scaled(&image_data, &mut pixels, window_size.width, window_size.height);
            }

            Event::RedrawRequested(_) => {
                println!("RedrawRequested {}x{}", window_size.width.to_string(), window_size.height.to_string());
                draw::draw_scaled(&image_data, &mut pixels, window_size.width, window_size.height);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            Event::MainEventsCleared => {
                // Redraw the application
                window.request_redraw();
            } _ => (),
        }
    });
}
