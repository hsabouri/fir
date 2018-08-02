use tree::{Tree};

pub struct CloseDecisionNode<T> {
    pub condition: fn(T) -> (T, bool),
    pub if_true: Tree<T>,
    pub if_false: Tree<T>,
}

impl<T> CloseDecisionNode<T> {
    pub fn new(condition: fn(T) -> (T, bool), if_true: Tree<T>, if_false: Tree<T>) -> CloseDecisionNode<T> {
        CloseDecisionNode::<T> {
            condition: condition,
            if_true: if_true,
            if_false: if_false,
        }
    }

    pub fn new_boxed(condition: fn(T) -> (T, bool), if_true: Tree<T>, if_false: Tree<T>) -> Box<CloseDecisionNode<T>>
    {
        Box::new(CloseDecisionNode::<T>::new(condition, if_true, if_false))
    }
}

pub struct OpenDecisionNode<T> {
    pub condition: fn(T) -> (T, Tree<T>),
}

impl<T> OpenDecisionNode<T> {
    pub fn new(condition: fn(T) -> (T, Tree<T>)) -> OpenDecisionNode<T> {
        OpenDecisionNode::<T> {
            condition: condition
        }
    }

    pub fn new_boxed(condition: fn(T) -> (T, Tree<T>)) -> Box<OpenDecisionNode<T>>
    {
        Box::new(OpenDecisionNode::<T>::new(condition))
    }
}

pub struct ActionNode<T> {
    pub next: Tree<T>,
    pub f: fn(T) -> T,
}

impl<T> ActionNode<T> {
    pub fn new(f: fn(T) -> T, next: Tree<T>) -> ActionNode<T> {
        ActionNode::<T> {
            f: f,
            next: next
        }
    }

    pub fn new_boxed(f: fn(T) -> T, next: Tree<T>) -> Box<ActionNode<T>>
    {
        Box::new(ActionNode::<T>::new(f, next))
    }
}
