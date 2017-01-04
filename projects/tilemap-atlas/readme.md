## Z-Index vs Blob TileSet


It is recommended that all tiles use transparent channels to obey tile index sorting.

![](https://lostgardenhome.files.wordpress.com/2019/09/essay-tiles-710781.jpg)


## Saving Format

The standard save format of the library is a row of 16 tiles, if there is a variant, placed directly below the corresponding position.

Ideally you should provide sprites with transparency channels.

## Supported Format

For tiles in non-standard cases, conversion functions are provided.

#### Tiny tiles: [from_tiny]()


#### Wang tiles: [GridCornerAtlas::from_wang](), [GridEdgeAtlas::from_wang]()


#### RPG Maker XP tiles: [GridCornerAtlas::from_rpg_maker_xp]()

#### RPG Maker MV tiles: [GridCornerAtlas::from_rpg_maker_mv]()


## Questions and Answers

### Q: Is the save format too wasteful?

Yes, this resource format will have a lot of useless transparent pixels.

But this is just a lossless resource format for editor, it will compile into a compact format in game.

And thanks to the png compression algorithm, it doesn't take up too much extra disk space