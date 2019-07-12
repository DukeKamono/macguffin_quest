# MacGuffin Quest
Copyright (c) 2019 James T Moore and William Olsen

Search lands far and wide, dungeons dark and deep, ruins desolate and decrepit for the most scared of grails… the MacGuffin!

## Description

The Ruler of the lands has issued a proclamation of new quest to all interested adventures. The proclamation reads:

> To all of the brave of heart, stout of character, or keen of mind adventurers in the land. I Ruler WhatsTheirName is in need of a most sacred relic; the MacGuffin. I know not where the MacGuffin rests; which is where you come in. To the adventurer that brings me the MacGuffin I promise everlasting fame and a wondrous reward. Go forth and bring to me the MacGuffin.

Upon hearing this proclamation many adventures set out in search of the MacGuffin. You are one such adventurer, are you capable of finding the famed MacGuffin?

---

MacGuffin Quest is an adventure game where you are tasked with finding the famed MacGuffin. To do this you will need to explore dangerous dungeons filled with monsters, traps, and puzzles. 

## Operation

The game is not near a completed state so all the controls/operations/features are not finished/listed.

(ie this will be updated... eventually)

### Setup

1. Install [Rust](https://www.rust-lang.org/)  
2. Run `git clone https://github.com/DukeKamono/macguffin_quest.git`
3. Run `cd macguffin_quest`
4. Run `cargo build`

### Running Game

1. Be located in macguffin_quest folder.
2. Run `cargo run --bin macguffin_quest`

### Playing the Game
Use WASD keys to move character sprite around the screen.

Collide with enemy sprite to cause it move to a new location.

Hit the q-key to quit.

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

* [ggez - goodies](https://github.com/ggez/ggez-goodies)  
Debating using this for the state system.