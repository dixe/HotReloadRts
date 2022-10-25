# Game/demo

Rts like combat



Goals:
* [x] selectable units
* [x] baisc commands to units, move
* [ ] Baisc command card, for selected unit.
* [ ] select all with ctrl + a
* [ ] animations
* [ ] [sound](#sound)
* [ ] [Health and damage](#health)



# Hot reload
Hot reload work on windows. Run the executable, can be run from root with `cargo r` when `game ` crate change, fx by running `cargo watch -x b` in the game crate, the game struct will be reloaded.

Just changing stuff in `logic` and rebuilding can be reloaded without a reset of state, with the `r` key. This also reloads shaders and .glb files (models from blender).


# <a name="sound"></a> Sound

Sound system stuff


# Health and damage <a name="health"></a>
Parts:
* [ ] Health, with health bars
* [ ] healing
* [x] Random damage
* [ ] death

## Healing
Maybe impl as a spell that affect an area, so units in that area are healed?


# Spell System
