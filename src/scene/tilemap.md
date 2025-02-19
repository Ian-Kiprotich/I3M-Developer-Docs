# Tile Map

Tile map is a 2D "image", made out of a small blocks called tiles. Tile maps used in 2D games to build game worlds 
quickly and easily. 

> ⚠️ This functionality is available only on nightly version of the engine and will be a part of the next stable release.
> If you want to use it, read [this chapter](../beginning/scripting.md#using-the-latest-engine-version) to learn how to
> switch to the nightly version of the engine.

An example of a tile map could be something like this:

![tile map](tile_map.png)

## How to Create

As usual, there are two major ways of creating a tile map - via code or via the editor. Code-based approach is ideal for
procedural worlds, while the editor-based approach is good for hand-crafted worlds.

### Code

The following example creates a simple tile map with two tile types - grass and stone. It creates stone foundation and 
lays grass on top of it.

```rust
{{#include ../code/snippets/src/scene/tilemap.rs:create_tile_map}}
```

Please refer to the API docs for more info about each method.

### Editor

Editor-based approach requires a bit of preparation, yet it is still simple. At first, create a scene, then you need a 
tile set, something like this:

![tile set](tileset.png)

It is a 11x11 sprite sheet for a top-down game. Now you need to create a tile set resource from this tile set. Navigate
the asset browser and click on `+` button near the search bar. Select `TileSet` resource and click `OK`. Find the resource
you've just created in the asset browser, double-click on it, and you should see something like this:

![tile set editor](tile_set_editor.png)

At this point you could add tiles individually, or import them all at once from a sprite sheet. Keep in mind, that unlike 
other game engine, I3M allows you to specify not just textures, but materials for each tile. This is much more flexible
solution, since it allows you to have custom shaders for each tile. To sum everything up there are three ways of adding
tiles to the tile set:

1) Import from a sprite sheet - this way the engine will create a unique embedded material (based on standard 2D shader), 
that will use the sprite sheet as diffuse texture. Sprite sheet will be split into a number of tiles and each tile will
have its own portion (texture coordinates) of the sprite sheet.
2) Drag and drop a texture to the tile set - almost the same as the previous option, but the texture coordinates will 
take the entire image.
3) Drag and drop a material to the tile set - the most flexible way, since it allows you to specify your own material for
tile.

For simplicity, we'll use the sprite sheet. Click on `Import` button and drop the sprite sheet to the region with 
checkerboard, set the appropriate number of rows and columns:

![import tile set](import_tile_set.png)

Now click `Import` and you should see something like this:

![imported tile set](imported_tile_set.png)

At this point you can select desired tiles and edit their properties in the inspector on the right side. As you can see
you can change tile's material, texture coordinates, collider (more on this below), color.

Now we have the tile set, and we can start creating a tile map using it. Click `Create -> 2D -> Tile Map` and you should
see something like this:

![empty tile map](empty_tile_map.png)

If you look closely, the editor warns us about missing tile set. Find the tile set you've just made and drag'n'drop it 
from the asset browser to the `Tile Set` field in the inspector. There's one more step before we can start editing the
tile map - we need a brush to paint on the tile map. Click `+` button in the asset browser and select `TileMapBrush`,
set a name for it and click `OK`. Now select the tile map scene node and click on `+` sign in the `Brushes` field, drag'n'drop
the brush you've just created to the newly created property. Navigate to the `Tile Map Control Panel` and select the
brush from the dropdown list. For now the brush is empty, the simplest way to fill it is to just drag'n'drop the tile set
to it:

![brush](brush.png)

At this point everything is ready for painting, click `Edit` button on the `Tile Map Control Panel` and you should see the
grid:

![grid](grid.png)

Select some tiles on the palette and start drawing:

![drawing](drawing.png)

There are number of tools (apart from the drawing itself) that could be useful while editing tile maps:

1) Erase - erases tiles using the shape of the current brush, could be activated using `Shift` key or by clicking on the
button with eraser icon.
2) Flood fill - fills a region with the same tile kind (or empty space) using random tiles from the current brush. Could
be activated using the button with paint bucket icon.
3) Pick - picks a rectangular region of tiles from the tile map itself and turns them into the current brush. Could be
activated using `Alt` key or by clicking the button with pipette icon.
4) Rectangular fill - fills a rectangular region with the tiles from the current brush. It tiles the given region using the
tiles from current brush. Could be activated using `Ctrl` key of by clicking on the button with the tiles icon.