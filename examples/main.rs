use eyre::Result;
use futures::executor::block_on;
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tiny_maps::Map;

#[tokio::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    // let window = Rc::new(window);

    let mut map = Map::new(-0.15, 51.502, 0, &window).await?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } = input
                    {
                        *control_flow = ControlFlow::Exit
                    }
                }
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
