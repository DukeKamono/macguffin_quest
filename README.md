# MacGuffin Quest
Copyright (c) 2019 James T Moore and William Olsen

Search lands far and wide, dungeons dark and deep, ruins desolate and decrepit for the most scared of grails… the MacGuffin!

## Description
The Ruler of the lands has issued a proclamation of new quest to all interested adventures. The proclamation reads:

> To all of the brave of heart, stout of character, or keen of mind adventurers in the land. I Ruler WhatsTheirName is in need of a most sacred relic; the MacGuffin. I know not where the MacGuffin rests; which is where you come in. To the adventurer that brings me the MacGuffin I promise everlasting fame and a wondrous reward. Go forth and bring to me the MacGuffin.

Upon hearing this proclamation many adventures set out in search of the MacGuffin. You are one such adventurer, are you capable of finding the famed MacGuffin?

---

MacGuffin Quest is an adventure game where you are tasked with finding the famed MacGuffin and returning it to the MacGuffin Man. To do this you will need to explore a dangerous dungeon filled with monsters, traps, and puzzles. If you survive... the MacGuffin just might be yours for the taking.

## Operation

### Setup
1. Install [Rust](https://www.rust-lang.org/)  
2. Run `git clone https://github.com/DukeKamono/macguffin_quest.git`
3. Run `cd macguffin_quest`
4. Run `cargo build`

### Running Game

1. Be located in macguffin_quest folder.
2. Run `cargo run --release --bin macguffin_quest`

We recommend running in release mode because debug ggez is slow drawing so man sprites.

### Playing the Game
Find the MacGuffin in the level and return it back to the MacGuffin Man to Win!
Colliding with an enemy sprite will cause you damage and eventual death.

Use `WASD` keys to move character sprite around the screen.  
Press `Space` to do a slashing attack.  
Press `Q` to cast a spell.  
Hold `Shift` to run.  
Pressing `P` will pause the game.  
Hit `Esc` to quit.  

### Testing the Game
The primary method of testing was general QA of playing the game.
There are limited tests, but the ggez Context is causing issues.

For the current tests provided we recommend running single thread.
`cargo test -- --test-threads=1`

### Running Level Builder

1. Be located in macguffin_quest folder.
2. Run `cargo run --bin levelbuilder`

### Running examples
Probably not of great interest due to fact that the examples are primarily demo programs to learn features of ggez.

Run with command `cargo run --example example_name`

See [example readme](https://github.com/DukeKamono/macguffin_quest/blob/master/examples/README.md) for more information.

## License 
This program is licensed under the MIT License located in the [LICENSE](https://github.com/DukeKamono/macguffin_quest/blob/master/LICENSE) file.

## Technology/Resources Used
(needs to be updated revised for any new dependencies)

* [Rust Prgramming Language](https://www.rust-lang.org/)  
The core programming language being used.  
Using version 1.36.0

* [ggez - Good Games Easily](https://github.com/ggez/ggez)  
The Rust game api being used.  
Using version 0.5.0-rc.0

* [rand](https://github.com/rust-random/rand)  
Used to generate random numbers.  
Using version 0.7

## Credit
Credit to Outside Sources:

dapper-skeleton, elf fighter, skeleton, macguffin-man created with [Universal-LPC-Spritesheet-Character-Generator](http://gaurav.munjal.us/Universal-LPC-Spritesheet-Character-Generator/)  
Ghost created by [Luis Zuno (ansimuz)](https://opengameart.org/content/chibi-monsters-set-01)  
Gel created by [AndHeGames](https://opengameart.org/content/adorable-blue-slimegelliving-water-droplet )  
Grue created by [Jesse McCarthy](https://opengameart.org/content/grue)  
Items creaetd by [Franco Giachetti (LudicArts)](https://opengameart.org/content/free-rpg-icons-2)