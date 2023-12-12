<img src="assets/logo.png" alt="logo" width="125" align="top" style="margin-right: 1rem"/>  

# 🎧 OxAg world generator
This is a world generator on steroids ^^

[![Static Badge](https://img.shields.io/badge/OxAg-audio_tool-orange)](https://github.com/xupremix/oxagaudiotool)

## Main features
#### 💠 Extremely customisable
It comes packed with tons of configuration options:
- tile type levels which allow you to have specific distributions of tile types
- tile content minimum and maximum spawn number & in batches spawing 
- random environmental conditions generator

#### 💠 River, Street, Maze & lava pots generation
'cause why not

#### 💠 Seed support
This generator accepts seeds as inputs so that you'll never loose your favourite generated worlds.

#### 💠 Cool presets
You'll find plenty of cool presets that can be used to generate unique worlds.

## Examples
You can find multiple examples under the `/examples` folder!

## Support & Feedback
Feel free to [open an issue](https://github.com/xupremix/oxagworldgenerator/issues) if either you have suggestions, think that something is missing or something is broken!



# TODOS:
- [x] add lava tiles  
- [x] add street tiles  
- [x] consider adding rivers  
- [x] modify the rate of content spawns  
- [ ] update presets  
- [ ] fix documentation `cargo doc --open`
- [x] make some examples of the builder
- [ ] make the tests `cargo test`
- [ ] update the visualizer for the faire
- [x] check if we want to add teleport tiles
- [x] consider if the want to add the wall tile
- [ ] improve logic by maybe using parallel iterators etc...  
- [ ] add in-line comments to the code
- [ ] make the audio tool
- [ ] create a good-looking README.md
- [ ] transmit the progress bar to an event handler - if specified
- [ ] run clippy fix for the lib warning `cargo clippy --fix`
- [ ] check for memory allocation (and if there are any improvements)
- [ ] add the logic for checking if a tile is spawning out of a range
- [ ] update the logic for the circles, allowing them to be placed on the corners
- [x] add serialization to file eg. load and save functions (option to clone the map)
- [x] check where to spawn the robot
- [x] consider the concept of savestate given a `HashMap<(usize, usize), Tile>` + private encryption