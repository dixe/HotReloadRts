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


## System
E

# Entity files
Have some files that describes entitties.
Can be different
* Spells: The animation, spell info: function, cooldown,
* Entity: Stats and spells. This will also indirectly has info on which animations that will be cast, since they are contained in spells.

Can we create checks, for entity skeleton, so we know they match. Fx if a boss has a different skeleton then a player char, can we ensure that the skeleton is different. Fx have animation speficy the skeleton, by id? And at load time we can check that the model and all spell->animations match.

# Ui
We want to display selected units, maybe also informations about the units, like hp, spells, active spell, ect.


Datadriven ui would be we see what is in the different tables, fx selected table has entries, we show them in a panel.
Rendering should be quite simple, deside panel size and position draw a panel, on top of that draw box or whatever for each selected. 
Still need some layout code, fx if number of selected is so big that we have to use multiple lines. 
Maybe split out the rust-gl-lib widget layout part, so we can reuse that, but handle rendering our self. 
But we also need some logic for checking what was clicked. If we as

The problem with widget is that they just take a gl and a renderContext. So to render them we need to have all the information inside the widget. 
It would be better to be able to have access to the whole game struct or state struct when rendering. On the other hand then for a slider 
it makes sense that they store their whole state. We also run into a problem because we store the widgets as dyn Widget. One solution is to add a generic argument `<T>` to UiState. This argument is then passed into the render functions. And for sliders and such it is just ignored. This forces all widgets to take the same object
as reference. Making reusable widgets might also be a problem, or not, since they can just take a T, where specific widgets take a concrete type. And not render also
takes a `&T` or `&mut T`.


## Flow
General flow
* User presses mouse.
* handle_input calls down into widgets and it returns wether the event was "capture" by any ui element or not. 
* if captured we just return from handle_inputs
* otherwise we handle normally
* after handle_inputs we can now run each widget dispatcher/listener function

use case:
use clicks on a unit in the units panel, to only select that unit.
Given the general flow and a panel that can push events like struct `SelectUnit {id: EntityId}` 
On click the panel pushes a SelectUnit event to the `DispatcherQueue` 

After handle_inputs we loop over the events in `DispatcherQueue` like they where sdl event and react to them like handle_inputs.
When we get a `SelectUnit {id: EntityId}` we know know that we can change the game state to only have that one unit selected


Use case:
We select a group of unit, we want the panel to show the selected units.
We need to push the list of units into the widget. One way is to use a queue where we send a struct to the widget. And the widget then reads the struct and can
update the internal state based on the struct. Fx we send a struct with a `selected: &Vec::<EntityId>` and the widget can then read the vec and update its own internal
state. Same with slider, where we can push updates with a simple `f32` to the slider_id. 

Rename the queues from dispatcher listeners ect. To something that tells which way it goes. Like `WidgetOutputQueue` and `WidgetInputQueue`. Then knowing what kind
of queue it is, might be easier then dispatcher and listener.


Anternative to using two queues is to have a update function that gets implemented, and it should take the `T`. The problem is that now we cannot make a slider, since 
it need to be specific to the `T` and we have to reimplement this update method for each usecase most likely.


Use case:
user clicks a spell to activate that spell. If instant just cast, if like heal, transform state so mouse now can cast spell. Just like hot key h.






  

All the widget examples have the whole game state inside the UiInfo. Which is kinda backwards.


Maybe look at intermediate mode gui


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
