use nodes::{CloseDecisionNode, OpenDecisionNode, ActionNode};

pub enum Node<T> {
    CloseDecision(Box<CloseDecisionNode<T>>),
    OpenDecision(Box<OpenDecisionNode<T>>),
    Action(Box<ActionNode<T>>),
    Finished,
}

impl<T> Node<T> {
    pub fn close_decision(condition: fn(T) -> (T, bool), if_true: Node<T>, if_false: Node<T>) -> Node<T> {
        Node::CloseDecision::<T>(
            CloseDecisionNode::new_boxed(condition, if_true, if_false)
        )
    }

    pub fn open_decision(reducer: fn(T) -> (T, Node<T>)) -> Node<T> {
        Node::OpenDecision::<T>(
            OpenDecisionNode::new_boxed(reducer)
        )
    }

    pub fn action(f: fn(T) -> T, next: Node<T>) -> Node<T> {
        Node::Action::<T>(
            ActionNode::new_boxed(f, next)
        )
    }

    pub fn finished() -> Node<T> {
        Node::Finished::<T>
    }

    pub fn explore(self, state: T) -> T {
        match self {
            Node::CloseDecision(node) => {
                let (state, decision) = (node.condition)(state);

                match decision {
                    true => node.if_true.explore(state),
                    false => node.if_false.explore(state)
                }
            },
            Node::OpenDecision(node) => {
                let (state, next) = (node.condition)(state);

                next.explore(state)
            },
            Node::Action(node) => {
                let state = (node.f)(state);
                
                node.next.explore(state)
            },
            Node::Finished => state
        }
    }
}
