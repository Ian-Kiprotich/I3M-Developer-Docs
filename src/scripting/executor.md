# Executor 

Executor is a simple wrapper that drives your game plugins, it is intended to be used for production builds of your game.
The editor runs the executor in separate process when you're entering the play mode. Basically, there is no significant 
difference between running the game from the editor, or running it as a separate application. The main difference is that
the editor passes `scene_path` parameter for the executor when entering the play mode.

## Usage

Executor is meant to be a part of your project's workspace, its typical look could something like this:

```rust,no_run
# extern crate i3m;
# use i3m::{
#     core::{pool::Handle, uuid::Uuid},
#     engine::executor::Executor,
#     plugin::{Plugin, PluginConstructor, PluginContext},
#     scene::{Scene},
# };
# struct GameConstructor;
# impl PluginConstructor for GameConstructor {
#     fn create_instance(
#         &self,
#         _scene_path: Option<&str>,
#         _context: PluginContext,
#     ) -> Box<dyn Plugin> {
#         todo!()
#     }
# }
fn main() {
    let mut executor = Executor::new();
    // Register your game constructor here.
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
```

Executor has full access to the engine, and through it to the main application window. You can freely change desired
parts, `Executor` implements `Deref<Target = Engine> + DerefMut` traits, so you can use its instance as an "alias"
to engine instance. 

To add a plugin to the executor, just use `add_plugin_constructor` method, it accepts any entity that implements
`PluginConstructor` traits.

## Typical Use Cases

This section covers typical use cases for the `Executor`.

### Setting Window Title

You can set window title when creating executor instance:

```rust,no_run
# extern crate i3m;
# use i3m::engine::executor::Executor;
# use i3m::window::WindowAttributes;
# use i3m::engine::GraphicsContextParams;
# use i3m::event_loop::EventLoop;
let executor = Executor::from_params(
    EventLoop::new().unwrap(),
    GraphicsContextParams {
        window_attributes: WindowAttributes {
            title: "My Game".to_string(),
            ..Default::default()
        },
        vsync: true,
    },
);
```