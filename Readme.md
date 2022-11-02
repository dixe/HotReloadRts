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
* [x] Health, with health bars
* [x] healing
* [x] Random damage
* [x] death


## Healing
Maybe impl as a spell that affect an area, so units in that area are healed?


# Spell System
* [x] spell duration
* [ ] spell tick
* [ ] cooldown

# Animation
Import sekeletal animation
When an animation is playing think about how we make sure they are played to end?
Maybe have a cast table/animaiton, that has info about when the cast/animation is done, and if it is Cancelable, ect.
Only call behaviour tree when nothing is active in this table. That way we automaticly fully play animations.
And the animation state will be driven by the behaviour/tables. And not a 'set' tree.



# Entity files
Have some files that describes entitties.
Can be different
* Spells: The animation, spell info: function, cooldown, 
* Entity: Stats and spells. This will also indirectly has info on which animations that will be cast, since they are contained in spells.

Can we create checks, for entity skeleton, so we know they match. Fx if a boss has a different skeleton then a player char, can we ensure that the skeleton is different. Fx have animation speficy the skeleton, by id? And at load time we can check that the model and all spell->animations match.



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

enum Child {
  Left,
  Right
}

struct Branch {
  decision_fn: fn (entitiyId, &state) -> Child, // function that take state and returns either left or right child. A simple version is to use binary branching always
  children_start: usize, // start of children, and then they are [..,left, right,..]
}
```


This will require a desicion state with current target, ect unless we set this in the state in tables??

This tree will end up returning some kind of decision.
And from that each a system can take all the decision for each agent and update their entity tables, animations ect.
Seperation the desicion tree and state handling seems like a good idea.


Can be made more 'scriptable' by creating a dsl language that translate to rust code fx

Start: if hp < 0.2 then SelfHeal else :Kill

SelfHeal: Cast Heal(self)
Kill: if has_target and target is alive then Attack else FindTarget

Attack: Attack Target
FindTarget: Target = GetNearest
