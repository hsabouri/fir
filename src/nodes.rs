use tree::{Node};

pub struct CloseDecisionNode<T> {
    pub condition: fn(T) -> (T, bool),
    pub if_true: Node<T>,
    pub if_false: Node<T>,
}

impl<T> CloseDecisionNode<T> {
    pub fn new(condition: fn(T) -> (T, bool), if_true: Node<T>, if_false: Node<T>) -> CloseDecisionNode<T> {
        CloseDecisionNode::<T> {
            condition: condition,
            if_true: if_true,
            if_false: if_false,
        }
    }

    pub fn new_boxed(condition: fn(T) -> (T, bool), if_true: Node<T>, if_false: Node<T>) -> Box<CloseDecisionNode<T>>
    {
        Box::new(CloseDecisionNode::<T>::new(condition, if_true, if_false))
    }
}

pub struct OpenDecisionNode<T> {
    pub condition: fn(T) -> (T, Node<T>),
}

impl<T> OpenDecisionNode<T> {
    pub fn new(condition: fn(T) -> (T, Node<T>)) -> OpenDecisionNode<T> {
        OpenDecisionNode::<T> {
            condition: condition
        }
    }

    pub fn new_boxed(condition: fn(T) -> (T, Node<T>)) -> Box<OpenDecisionNode<T>>
    {
        Box::new(OpenDecisionNode::<T>::new(condition))
    }
}

pub struct ActionNode<T> {
    pub next: Node<T>,
    pub f: fn(T) -> T,
}

impl<T> ActionNode<T> {
    pub fn new(f: fn(T) -> T, next: Node<T>) -> ActionNode<T> {
        ActionNode::<T> {
            f: f,
            next: next
        }
    }

    pub fn new_boxed(f: fn(T) -> T, next: Node<T>) -> Box<ActionNode<T>>
    {
        Box::new(ActionNode::<T>::new(f, next))
    }
}
