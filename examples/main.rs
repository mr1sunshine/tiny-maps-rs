use env_logger::TimestampPrecision;
use eyre::Result;
use log::{error, info};
use tiny_maps::Map;
use tokio::sync::mpsc;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const HELSINKI: (f32, f32) = (24.945831, 60.192_06);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1000, 1000))
        .build(&event_loop)?;
    let (tx, mut rx) = mpsc::channel(32);

    let mut map = Map::new(&HELSINKI.into(), 15, window).await?;

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            // info!("tokio event {:?}", event);
            match event {
                Event::RedrawRequested(_) => {
                    info!("redraw requested");
                    match tokio::try_join!(map.render()) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to render {}", e),
                    }
                }
                Event::WindowEvent { event, .. } => {
                    if let WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } = event
                    {
                        use VirtualKeyCode::*;
                        match key {
                            U => {
                                match tokio::try_join!(map.set_zoom(map.zoom() + 1)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to zoom in {}", e),
                                };
                            }
                            Y => {
                                let current_zoom = map.zoom();
                                if current_zoom == 0 {
                                    return;
                                }
                                match tokio::try_join!(map.set_zoom(current_zoom - 1)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to zoom out {}", e),
                                };
                            }
                            Left => {
                                let mut current_point = map.point();
                                current_point.set_lng(current_point.lng() - 0.01);
                                match tokio::try_join!(map.set_point(current_point)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to set point {}", e),
                                };
                            }
                            Right => {
                                let mut current_point = map.point();
                                current_point.set_lng(current_point.lng() + 0.005);
                                match tokio::try_join!(map.set_point(current_point)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to set point {}", e),
                                };
                            }
                            Up => {
                                let mut current_point = map.point();
                                current_point.set_lat(current_point.lat() + 0.005);
                                match tokio::try_join!(map.set_point(current_point)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to set point {}", e),
                                };
                            }
                            Down => {
                                let mut current_point = map.point();
                                current_point.set_lat(current_point.lat() - 0.01);
                                match tokio::try_join!(map.set_point(current_point)) {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to set point {}", e),
                                };
                            }
                            _ => {}
                        }
                    }
                }
                _ => (),
            }
        }
    });

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Poll;

        let mut handle = false;
        // info!("event {:?}", event);
        match &event {
            Event::RedrawRequested(_) => handle = true,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::Destroyed
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => handle = true,
            },
            _ => (),
        }

        if handle {
            let static_event = event
                .to_static()
                .expect("We should always get static lifetime");

            let tx = tx.clone();
            tokio::spawn(async move { tx.send(static_event).await.unwrap() });
        }
    })
}
