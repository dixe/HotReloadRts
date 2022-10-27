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


# Ai system

How to make an enemy work. Fx baisc 

Start ->
  is low hp -> 
    true -> heal self
    false -> 
       has nearby target ->
         true -> kill target
         false -> find target and move to target
         

This tree can be walked through each update step, and if a target dies, the next update step will find a new one.
When hp is low heal will be cast

## How to structure this as code?
```
One way is to create a tree
struct Tree {
 nodes: Vec::<Node> 
}

enum Node {
  Branch(Branch),
  Leaf(Leaf)
}

enum Desicion {
  Move_to(V3),
  Attack(entityId),
  Cast_spell,
  ..
}

struct Leaf {
  decision_fn: fn (entitiyId, &state) -> Desicion,
}

struct Branch {
  decision_fn: fn (entitiyId, &state) -> childIndex (usize), // function that take state and returns the next node to try, either   
  children: usize, // number of children, 
  children_start_index: usize, // which node in the tree that is the first child  
}
```
