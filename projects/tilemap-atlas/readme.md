# TileSets



## Standard Format

The standard save format of the library is a row of 16 tiles, if there is a variant, placed directly below the corresponding position.

Ideally you should provide sprites with transparency channels.

![](https://lostgardenhome.files.wordpress.com/2019/09/essay-tiles-710781.jpg)

## Supported Format

- You can download tile sets from these sites:

- [opengameart.org](https://opengameart.org/art-search-advanced?field_art_tags_tid=tileset)
- [kenney.nl](https://kenney.nl/assets)
- [cr31.co.uk](http://www.cr31.co.uk/stagecast/wang/tiles_c.html)

For tiles in non-standard cases, conversion functions are provided.

#### RPG Maker XP tiles:

- [GridCornerAtlas::from_rpg_maker_xp](https://docs.rs/tileset/latest/tileset/struct.GridCornerAtlas.html#method.from_rpg_maker_xp)

#### RPG Maker MV tiles

- [GridCornerAtlas::from_rpg_maker_mv](https://docs.rs/tileset/latest/tileset/struct.GridCornerAtlas.html#method.from_rpg_maker_xp)

#### Tiny tiles

- [from_tiny](https://docs.rs/tileset/latest/tileset/struct.GridCornerAtlas.html#method.from_rpg_maker_xp)

#### Wang tiles

- [GridCornerAtlas::from_wang](https://docs.rs/tileset/latest/tileset/struct.GridCornerAtlas.html#method.from_rpg_maker_xp)

---

- [GridEdgeAtlas::from_wang](https://docs.rs/tileset/latest/tileset/struct.GridCornerAtlas.html#method.from_rpg_maker_xp)


## Packed Format





## Questions and Answers

### Q: Is the save format too wasteful?

Yes, this resource format will have a lot of useless transparent pixels.

But this is just a lossless resource format for editor, it will compile into a compact format in game.

And thanks to the png compression algorithm, it doesn't take up too much extra disk space

### Q: How about import 3-tiles or 4-tiles?

The library does not (and will not) support this, you must You have to manually decompose into 2-tiles.

### Q: NavMeshes, Layers?

This library is only for resource management, please use [TileMap](https://github.com/oovm/Tilemap/tree/dev/projects/tilemap) for related functions.