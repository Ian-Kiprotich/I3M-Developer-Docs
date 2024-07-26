# Custom game loop

**WARNING: Manual engine initialization considered obsolete, use plugins and scripts instead.**

The plugin might be limiting for some cases, in such situations the engine could be initialized manually. It
requires some decent amount of code and could error-prone for beginners. You should use custom game loop only if
you're experienced game developer!

Here is an example of custom game loop with comments that will guide your through all aspects of it:

```rust,no_run
# extern crate i3m;
# use i3m::{
#     core::{instant::Instant, log::{Log, MessageKind}},
#     engine::{GraphicsContextParams, Engine, EngineInitParams, GraphicsContext, SerializationContext},
#     asset::manager::ResourceManager, 
#     event::{Event, WindowEvent},
#     event_loop::{ControlFlow, EventLoop},
#     window::WindowAttributes,  
#     utils::{
#         translate_event,
#     },
# };
# use std::{sync::Arc};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // Then initialize the engine.
    let graphics_context_params = GraphicsContextParams {
        window_attributes: WindowAttributes {
            title: "Custom Game Loop".to_string(),
            resizable: true,
            ..Default::default()
        },
        vsync: true,
    };

    let serialization_context = Arc::new(SerializationContext::new());
    let mut engine = Engine::new(EngineInitParams {
        graphics_context_params,
        resource_manager: ResourceManager::new(),
        serialization_context,
    })
    .unwrap();

    // Define game loop variables.
    let mut previous = Instant::now();
    let fixed_timestep = 1.0 / 60.0;
    let mut lag = 0.0;

    // Finally run our event loop which will respond to OS and window events and update
    // engine state accordingly. Engine lets you to decide which event should be handled,
    // this is minimal working example if how it should be.
    event_loop.run(move |event, window_target, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // This main game loop - it has fixed time step which means that game
                // code will run at fixed speed even if renderer can't give you desired
                // 60 fps.
                let elapsed = previous.elapsed();
                previous = Instant::now();
                lag += elapsed.as_secs_f32();
                while lag >= fixed_timestep {
                    lag -= fixed_timestep;

                    // ************************
                    // ************************
                    // Put your game logic here.
                    // ************************
                    // ************************

                    // It is very important to update the engine every frame!
                    engine.update(fixed_timestep, control_flow, &mut lag, Default::default());
                }

                // It is very important to "pump" messages from UI. Even if don't need to
                // respond to such message, you should call this method, otherwise UI
                // might behave very weird.
                while let Some(_ui_event) = engine.user_interface.poll_message() {
                    // ************************
                    // ************************
                    // Put your data model synchronization code here. It should
                    // take message and update data in your game according to
                    // changes in UI.
                    // ************************
                    // ************************
                }

                // Rendering must be explicitly requested and handled after RedrawRequested event is received.
                if let GraphicsContext::Initialized(ref ctx) = engine.graphics_context {
                    ctx.window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Run renderer at max speed - it is not tied to game code.
                engine.render().unwrap();
            }
            Event::Resumed => {
                engine.initialize_graphics_context(window_target).unwrap();
            }
            Event::Suspended => {
                engine.destroy_graphics_context().unwrap();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        // It is very important to handle Resized event from window, because
                        // renderer knows nothing about window size - it must be notified
                        // directly when window size has changed.
                        if let Err(e) = engine.set_frame_size(size.into()) {
                            Log::writeln(
                                MessageKind::Error,
                                format!("Unable to set frame size: {:?}", e),
                            );
                        }
                    }
                    // Handle rest of events here if necessary.
                    _ => (),
                }

                // It is very important to "feed" user interface (UI) with events coming
                // from main window, otherwise UI won't respond to mouse, keyboard, or any
                // other event.
                if let Some(os_event) = translate_event(&event) {
                    engine.user_interface.process_os_event(&os_event);
                }
            }
            // Continue polling messages from OS.
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}
```