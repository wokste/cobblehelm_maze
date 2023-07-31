# Inspiration

Primary sources of inspiration include:
- Kens Labyrinth
- Roguelikes and especially Brogue

Secondary sources of inspiration:
- Dungeons and Dragons (Especially random tables)
- Unreal Tournament
- Tomb Raider

# Art

| Asset                      | License   | By           | Source |
| -------------------------- | --------- | ------------ | --
| audio/monster_hurt.ogg     | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/pickup_coins.ogg     | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/pickup_heal.ogg      | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/pickup_key.ogg       | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/player_hurt.ogg      | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/shoot_blueblob.ogg   | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/shoot_redspikes.ogg  | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| audio/shoot_shock.ogg      | CC-0      | Steven Wokke | [jsfxr](https://sfxr.me/)
| other/bit_portion.ttf      | CC-BY-3.0 | Joeb Rogers  | [1001fonts.com](https://www.1001fonts.com/bitpotion-font.html)
| sprites/sprites.png        | CC-BY-4.0 | See below    | See below

For the sprites.png, most of the original art is created for Dungeon Crawl Stone Soup by [Chris Hamons](https://opengameart.org/content/dungeon-crawl-32x32-tiles) and [Medicine Storm](https://opengameart.org/content/dungeon-crawl-32x32-tiles-supplemental). Both these tilesets are under the CC-0 license. They are scaled up using [ScalePix](https://morgan3d.github.io/quadplay/tools/scalepix.html).

The changes are licensed CC-BY by Steven Wokke. Unfortunately, we haven't tracked how these sprites are changed. I do give explicit permission for DCSS developers to place these assets into CC-0 on my behalf. (This is as a thank you for all the hard work they did with their sprites).

# Used Code

The engine used is Bevy. I cannot be more happy with it. Note that care has been used to make the graphics feel old-school. This game is by no means a demonstration of the graphics quality of Bevy but it does show how a bevy-newbie can make a complex game with it.

A list of libraries used:
| Library | License | Notes |
| ------- | ------- | ----- |
| `bevy` | MIT | The main engine. 
| `bitflags` | MIT | To do some bitwise flags.
| `clap` | MIT | For command line parsing.
| `derive_more` | MIT | To derive more things.
| `fastrand` | MIT | A smaller and deterministic random library.
| `serde` | MIT | Will be used in the future to add config files for input controls.
| `tinyvec` | MIT | Reduces some allocations in some cases.
| `vergen` | MIT | To log the git commit and version.

A list of tutorials used:
* UI is inspired by [@jacques-dev on youtube](https://www.youtube.com/watch?v=GOl-kacs8TQ))

## A note on GLP

The GPL is notorious for the GPL traps where people accidentially use a GPL dependency and are forced to release their code under GPL.

The cobblehelm maze is currently released under the GPL 3 license or later. However, I have had no contributions from anyone else so I am still free to change the license for later versions. The truth is, I have no idea what to do with it (beyond just coding for fun).

When I do figure that out, the license may change. This may either give more or less freedom.