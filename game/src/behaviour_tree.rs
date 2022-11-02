use crate::state;
use crate::entity_system::EntityId;
use crate::math::V3;
use std::fmt;

pub struct BehaviourTrees {


}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec::<Node>
}

impl Tree {

    pub fn run(&self, id: EntityId, state: &state::State) -> Decision {

        let mut node = 0;
        loop {
            match &self.nodes[node] {
                Node::Branch(b) => {
                    let r = (b.decision_fn)(id, state);
                    node = b.children_start + match r {
                        Child::Left => 0,
                        Child::Right => 1
                    };

                },
                Node::Leaf(l) => {
                    return (l.decision_fn)(id, state);
                }
            }

        }
    }
}

#[derive(Debug)]
pub enum Node {
  Branch(Branch),
  Leaf(Leaf)
}

#[derive(Debug, Clone, Copy)]
pub enum Decision {
    MoveTo(EntityId),
    AttackTarget,
    Target(EntityId),
    UnTarget,
    CastSpell,
    Nothing
}

pub type LeafFn = fn (EntityId, &state::State) -> Decision;


pub struct Leaf {
  decision_fn: LeafFn
}

#[derive(Debug)]
pub enum Child {
  Left,
  Right
}

pub type BranchFn  = fn (EntityId, &state::State) -> Child;

pub struct Branch {
    pub decision_fn: BranchFn, // function that take state and returns either left or right child. A simple version is to use binary branching always
    pub children_start: usize, // start of children, and then they are [..,left, right,..]
}

pub struct TreeBuilder {
    nodes: Vec::<Node>,
}

impl TreeBuilder {

    pub fn new() -> Self {
        Self {
            nodes: vec![empty_leaf()],
        }
    }

    /// Add a branch to the given index, return index of the first child of branch
    pub fn add_branch(&mut self, fun: BranchFn, index :usize) -> usize {
        let children_start = self.nodes.len();

        // Set node
        self.nodes[index] = Node::Branch(
            Branch {
                decision_fn: fun,
                children_start
            }
        );

        // push 2 children
        self.nodes.push(empty_leaf());
        self.nodes.push(empty_leaf());

        let child_index = self.nodes.len();
        children_start
    }

    pub fn add_leaf(&mut self, fun: LeafFn, index: usize) {
        self.nodes[index] = Node::Leaf( Leaf {
            decision_fn: fun
        });
    }

    pub fn build(self) -> Tree {
        Tree {
            nodes: self.nodes
        }
    }
}



// LEAF FUNCTIONS
pub fn empty_leaf() -> Node {
    Node::Leaf(
        Leaf {
            decision_fn: empty_leaf_fn
        }
    )
}


pub fn empty_leaf_fn(_: EntityId, _ :&state::State) -> Decision {
    Decision::Nothing
}



pub fn cast_heal(_: EntityId, _ :&state::State) -> Decision {
    Decision::CastSpell
}


pub fn kill(id: EntityId, state :&state::State) -> Decision {

    if let Some(target_id) = state.entities.targets.get(&id) {

        let my_idx = match state.entities.id_to_index.get(&id) {
            Some(idx) => *idx,
            None => {
                return Decision::UnTarget;
            }
        };
        if let Some(&target_idx) = state.entities.id_to_index.get(target_id) {

            let my_pos = state.entities.positions[my_idx];
            let target_pos = state.entities.positions[target_idx];
            let dist = (my_pos - target_pos).magnitude();

            if dist < 1.0 {
                Decision::AttackTarget
            } else {
                Decision::MoveTo(*target_id)
            }

        } else {
            Decision::UnTarget
        }

    } else {
        find_target(id, state)
    }
}


fn find_target(id: EntityId, state :&state::State) -> Decision {
    let mut target_id = 0;
    for i in 0..state.entities.positions.len() {
        if state.entities.team[i] == 1 {
            target_id = state.entities.ids[i];
        }
    }

    if target_id != 0 {
        return Decision::Target(target_id);
    }

    return Decision::Nothing;
}



// BRANCH FUNCTIONS
pub fn health_low(id: EntityId, state :&state::State) -> Child {

    if let Some(dmg) = state.entities.damage.get(&id) {
        if dmg.health < 0.2 {
            return Child::Left;
        }
    }

    return Child::Right;
}


pub fn heal_then_kill() -> Tree {
  let mut builder = TreeBuilder::new();

    let child_index = builder.add_branch(health_low, 0);

    // leaf for left child i.e. the heal
    builder.add_leaf(cast_heal, child_index);

    // leaf for right child. i.e the kill
    let child_index = builder.add_leaf(kill, child_index + 1);

    builder.build()
}



impl fmt::Debug for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Branch")
         .field("children_start", &self.children_start)
         .finish()
    }
}

impl std::fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Leaf")
         .finish()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn build1() {

        let mut builder = TreeBuilder::new();

        let child_index = builder.add_branch(health_low, 0);

        // leaf for left child i.e. the heal
        builder.add_leaf(cast_heal, child_index);

        // leaf for right child. i.e the kill
        let child_index = builder.add_leaf(kill, child_index + 1);


    }



    #[test]
    pub fn build2() {

        let mut builder = TreeBuilder::new();

        let child_index = builder.add_branch(health_low, 0);

        // leaf for left child i.e. the heal
        builder.add_leaf(cast_heal, child_index);

        // leaf for right child. i.e the kill
        let child_index = builder.add_leaf(kill, child_index + 1);


    }


}
