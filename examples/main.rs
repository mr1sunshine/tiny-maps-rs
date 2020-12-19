use eyre::Result;
use futures::executor::block_on;
use tiny_maps::Map;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let w_id = window.id();
    window.set_inner_size(PhysicalSize::new(800, 800));

    let mut map = Map::new(-0.15, 51.502, 15, window).await?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == w_id => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::U),
                        ..
                    } => {
                        match block_on(map.set_zoom(map.zoom() + 1)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to zoom in {}", e),
                        };
                    }
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Y),
                        ..
                    } => {
                        let current_zoom = map.zoom();
                        if current_zoom == 0 {
                            return;
                        }
                        match block_on(map.set_zoom(current_zoom - 1)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to zoom out {}", e),
                        };
                    }
                    _ => (),
                },
                _ => {}
            },
            Event::RedrawRequested(_) => {
                println!("redraw requested");
                match block_on(map.render()) {
                    Ok(_) => {}
                    Err(e) => println!("Failed to render {}", e),
                }
            }
            _ => (),
        }
    });
}
