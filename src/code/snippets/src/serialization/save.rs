use i3m::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext},
    scene::Scene,
};
use std::path::Path;

// ANCHOR: save
#[derive(Visit, Reflect, Debug, Default)]
struct MyGame {
    scene: Handle<Scene>,
}

impl MyGame {
    fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        // Load the scene as usual.
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene: Handle::NONE,
        }
    }

    fn save_game(&mut self, context: &mut PluginContext) {
        let mut visitor = Visitor::new();
        // Serialize the current scene.
        context.scenes[self.scene]
            .save("Scene", &mut visitor)
            .unwrap();
        // Save it to a file.
        visitor.save_binary(Path::new("save.rgs")).unwrap()
    }

    fn load_game(&mut self, context: &mut PluginContext) {
        // Loading of a saved game is very easy - just ask the engine to load your save file.
        // Note the difference with `Game::new` - here we use `request_raw` instead of
        // `request` method. The main difference is that `request` creates a derived scene
        // from a source scene, but `request_raw` loads the scene without any modifications.
        context.async_scene_loader.request_raw("save.rgs");
    }
}

impl Plugin for MyGame {
    fn on_scene_begin_loading(&mut self, _path: &Path, context: &mut PluginContext) {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
// ANCHOR_END: save
