use eyre::Result;
use futures::executor::block_on;
use tiny_maps::Map;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const HELSINKI: (f64, f64) = (24.945831, 60.192059);

#[tokio::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let w_id = window.id();
    window.set_inner_size(PhysicalSize::new(800, 800));

    let mut map = Map::new(&HELSINKI.into(), 15, window).await?;

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
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    } => {
                        let mut current_point = map.point();
                        current_point.set_lng(current_point.lng() - 0.01);
                        match block_on(map.set_point(current_point)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to set point {}", e),
                        };
                    }
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    } => {
                        let mut current_point = map.point();
                        current_point.set_lng(current_point.lng() + 0.005);
                        match block_on(map.set_point(current_point)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to set point {}", e),
                        };
                    }
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    } => {
                        let mut current_point = map.point();
                        current_point.set_lat(current_point.lat() + 0.005);
                        match block_on(map.set_point(current_point)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to set point {}", e),
                        };
                    }
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    } => {
                        let mut current_point = map.point();
                        current_point.set_lat(current_point.lat() - 0.01);
                        match block_on(map.set_point(current_point)) {
                            Ok(_) => {}
                            Err(e) => println!("Failed to set point {}", e),
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
