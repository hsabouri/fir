mod tree;
mod nodes;

use tree::{Node};
use nodes::{CloseDecisionNode, OpenDecisionNode, ActionNode};

#[derive(Clone)]
struct State {
    name: &'static str,
    age: i32,
    home: &'static str,
}

fn is_age_over_18(state: State) -> (State, bool) {
    (state.clone(), state.age > 18)
}

fn say_hello(state: State) -> State {
    println!("Hello!");
    state
}

fn say_bye(state: State) -> State {
    println!("Goodbye enculé!");
    state
}

fn reducer(state: State) -> (State, Node<State>) {
    if state.name == "hugo" {
        (state, Node::<State>::action(
            say_hello,
            Node::Finished::<State>
        ))
    } else {
        (state, Node::<State>::action(
            say_bye,
            Node::Finished::<State>
        ))
    }
}

fn main() {
    let tree = Node::<State>::close_decision(
        is_age_over_18,
        Node::<State>::action(
            say_hello,
            Node::<State>::open_decision(reducer),
        ),
        Node::<State>::action(
            say_bye,
            Node::Finished::<State>
        )
    );

    tree.explore(State {
        name: "hugo",
        age: 35,
        home: "ique"
    });
}
