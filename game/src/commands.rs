/// All commands that units can be given. Some might not apply to all types of units,
/// those could be ignored
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    DefaultRightClick(Target), // Default rightclick, along with where there was clicked, maybe we want this in world pos
    Attack(Target),
    Stop,
    Empty
}



/// A target to a command
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Position(f32,f32),
    Entity(usize) // TODO: make an entityId, that can be used here, to refere to entity clicked
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Move,
    Spell
}
