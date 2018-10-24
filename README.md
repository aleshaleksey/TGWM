# TGWM (Tales of the Great White Moose)[<img src="https://api.travis-ci.org/aleshaleksey/TGWM.svg?branch=master">](https://travis-ci.org/aleshaleksey/TGWM)

A flat RPG written in rust as an etude in using the language.

Aim
---
An etude in machine learning and using the conrod library for making a universal
GUI (one that can be easily recompiled between triples without having to reprogram
too many things. Since debugging a game is more fun, it ended up being a game.

About
-----
This document describes the above repository, first it covers the kind-of-technical details,
and then the gameplay related topics.


Current State
-------------
***Play:***

The mechanics work (for the most part), and you can run around the world getting killed and leveling up, but there is no story,
no real start or end.

***UI:***

Currently there is a working GUI with a main, game and battle menu.
Shortcut keys exist for overland travel, but not for navigating the menu (maybe a good idea).
Brightness and size of window can be adjusted. Due to how glium uses OpenGl, brightness of flat
texture widgets and images (ie sprites) are adjusted separately.

***Sound:***

There is a juke box powered by rodio, which plays music in battle, which can be customised or silenced.
There are currently no sound effects. (As this is an etude in programming and not composing, the game
music is half-hearted and the in-game song list can be changed. Currently looping needs tweaking in order to not be jarring.

***Graphics:***

Widget buttons as sprites. Uses images as backgrounds. World map is generated from instructions in the executable.
In battle sprite shake on damage.
*Update 11-08-2018: Basic graphics for spell effects and attacks have been implemented.*

***Mechanics:***

Implemented: The mechanisms for travelling the world, battle, in battle spellcasting,
exp. and dungeons have been implemented.
Mostly implemented: NPCs,story.
Unimplemented: Teleportation spells and inventory have not been implemented.

***AI:***

Currenly there are three mechanisms for the "AI"

***Random:***

Monsters take "random" actions against "random" targets.

***Traditional:***

Monsters use a simple algorithm in battle, as a function of the stats of a battle's participants.

***Algorithm:***

The computer simulates battles for ten seconds at the start of a battle (using the above algorithm),
and then uses this battle record to make a decision. Currenly the algorithm probability tables of actions and
consequences, and uses them to decide the best action. As a back up uses a cause-effect algorithm, however this currently has some "bugs" related to healing spells. If the "statistics are weak" it will resort to the first
two methods. Currently this approach is problematic for battles which aren't one-on-one, and where there is a
small chance of winning or losing. In fact it's problematic in general and needs work. It should be noted that obviously it's not a computationally efficient "AI" for a computer game. The idea in using this is to use the game as a machine learning platform (I will work on this more once the rest of the game is written).

Notes on compiling
------------------
~~Originally compiled with Rust  1.1.9-nightly to 1.2.4-nightly nightly (for now I think most versions will wor~~
This version is compiled using rust stable (so far compiled successfully with 1.22.1).
There are a lot of indirect dependencies which may go horribly wrong when compiling (see below),
therefore I have included an old Cargo.lock file for troubleshooting.

NB: for a clean, release version (on Windows) add " --release -C link-args="-Wl,--subsystem,windows" "

***Linux:***

cargo run --release --features="winit glium libc"

***Windows (x64):***

*From windows:*

cargo run --release --features="winit glium libc"

*Crosscompile from Linux:*

cargo rustc --bin q-moose --release --features="winit glium libc" --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc

***Dependencies:***

TGWM uses conrod 0.57.0 (newer versions may or may not work). However the compiled program seems to work correctly only if the dependencies to conrod are specified as *exact* (eg: glium = "=0.20"). However the dependencies on the glium and winit *also* have to be specified as exact (modify the Cargo.toml on the dependencies).


License
-------
Since this is not a useful project, but something of an etude, and anyone who feels like it
is free to "do whatever" with it, it almost doesn't need a licence. However, just in case,
it is under the GPL 3.0 licence.

Gameplay
--------
The game play is classical turn based party on party, player vs computer fantasy battler. The main feature is the player does not know the exact numerical value of their stats (nor does the main AI layer). There is one player party, theoretically containing up to 5 creatures (but currently only up to two) and four enemy parties (which do not have a condition of allied victory, but often consider you a high priority target).
The player travels the world, "explores" (currently about a dozen) dungeons and completes "quests" (currently none really).

Story Related Elements
----------------------
As of the 16th of September 2018, the storyline engine has been written, but the story line has not (a simple demo quest triggered when the player has spent 10 Exp points has been written).

Comment
-------
If anyone has comments on why this sucks and what should be changed to improve it, they are welcome.
