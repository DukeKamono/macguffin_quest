# Examples
Copyright (c) 2019 James T Moore and William Olsen

These are a bunch of demo programs created to learn features of ggez or test concept ideas. Probably not really interesting to anyone other than the developers...

## Running an Example
Use command `cargo run --example [example_name]` where `[example_name]` is one of the examples.

## List of Examples
* `collisions`  
Example of collision detection using trait.  
Move square with arrow keys.
* `display_user_text`  
Loads a font type file to display text in custom font.  
Accepts user text input, displaying to the screen.  
Has working backspace functionality.
* `draw_images`  
How to load and draw a sprite image.
* `generative_art`  
Based on [ggez guide Generative Art](https://github.com/ggez/ggez/blob/master/docs/guides/GenerativeArt.md).
* `hello_ggez`  
Based on [ggez guide Hello ggez](https://github.com/ggez/ggez/blob/master/docs/guides/HelloGgez.md).
* `move_circle`  
Based on ggez [keyboard input documentation](https://docs.rs/ggez/0.5.0-rc.2/ggez/input/keyboard/index.html) example.  
Takes in arrow key input from keyboard.
* `screen_coord`  
Tests moving the screen coordinates  
Also makes use of draw queued text
* `sprite_sheet`  
Simple example to load a sprite sheet as Image and make use of it.  
Sheet generated using this [tool](http://gaurav.munjal.us/Universal-LPC-Spritesheet-Character-Generator/#?sex=male&body=skeleton&eyes=none&legs=pants_red&clothes=formal&formal-shirt=1&formal-pants=1&formal-vest=1&mail=none&armor=none&tie=on&hair=shorthawk_green&arms=none&shoulders=none&spikes=none&bracers=leather&greaves=none&gloves=none&hat=none&=shoes_black&belt=none&bracelet=none&weapon=none&shield=none)