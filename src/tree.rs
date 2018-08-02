use node::{CloseDecisionNode, OpenDecisionNode, ActionNode};

pub enum Tree<T> {
    CloseDecision(Box<CloseDecisionNode<T>>),
    OpenDecision(Box<OpenDecisionNode<T>>),
    Action(Box<ActionNode<T>>),
    Finished,
}

impl<T> Tree<T> {
    pub fn close_decision(condition: fn(T) -> (T, bool), if_true: Tree<T>, if_false: Tree<T>) -> Tree<T> {
        Tree::CloseDecision::<T>(
            CloseDecisionNode::new_boxed(condition, if_true, if_false)
        )
    }

    pub fn open_decision(reducer: fn(T) -> (T, Tree<T>)) -> Tree<T> {
        Tree::OpenDecision::<T>(
            OpenDecisionNode::new_boxed(reducer)
        )
    }

    pub fn action(f: fn(T) -> T, next: Tree<T>) -> Tree<T> {
        Tree::Action::<T>(
            ActionNode::new_boxed(f, next)
        )
    }

    pub fn finished() -> Tree<T> {
        Tree::Finished::<T>
    }

    pub fn explore(self, state: T) -> T {
        match self {
            Tree::CloseDecision(node) => {
                let (state, decision) = (node.condition)(state);

                match decision {
                    true => node.if_true.explore(state),
                    false => node.if_false.explore(state)
                }
            },
            Tree::OpenDecision(node) => {
                let (state, next) = (node.condition)(state);

                next.explore(state)
            },
            Tree::Action(node) => {
                let state = (node.f)(state);
                
                node.next.explore(state)
            },
            Tree::Finished => state
        }
    }
}
