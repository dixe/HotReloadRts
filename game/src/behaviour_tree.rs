use crate::state;
use crate::entity_system::EntityId;
use crate::math::V3;

pub struct Tree {
    nodes: Vec::<Node>
}

pub enum Node {
  Branch(Branch),
  Leaf(Leaf)
}

pub enum Decision {
    MoveTo(EntityId),
    Attack(EntityId),
    CastSpell,
    Nothing
}

pub type LeafFn = fn (EntityId, &state::State) -> Decision;

pub struct Leaf {
  decision_fn: LeafFn
}

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


pub fn kill(_: EntityId, _ :&state::State) -> Decision {

    // if we have a target and are in range
    let target_id = 0;
    let has_target = false;

    if has_target {
        Decision::Attack(target_id)
    } else {
        // Find target
        // if found
        Decision::MoveTo(target_id)
        // else return Nothing, everybody is dead, we can idle or dance
    }
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


pub fn


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
