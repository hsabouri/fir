mod tree;
mod node;

use tree::{Tree};

#[derive(Clone)]
struct State {
    name: &'static str,
    age: i32,
}

fn is_age_over_21(state: State) -> (State, bool) {
    (state.clone(), state.age > 21)
}

fn say_hello(state: State) -> State {
    println!("Hello {}!", state.name);
    state
}

fn say_bye(state: State) -> State {
    println!("Goodbye {}!", state.name);
    state
}

fn reducer(state: State) -> (State, bool) {
    let res = state.name == "Jack";
    (state, res)
}

fn main() {
    let tree = Tree::<State>::close_decision(
        is_age_over_21,
        Tree::<State>::action(
            say_hello,
            Tree::<State>::build_open_decision(
                Tree::<State>::action(
                    say_bye,
                    Tree::<State>::finished()
                )
            ).link(
                reducer,
                Tree::<State>::action(
                    say_hello,
                    Tree::<State>::finished()
                )
            ).build(),
        ),
        Tree::<State>::action(
            say_bye,
            Tree::Finished::<State>
        )
    );

    tree.explore(State {
        name: "Jack",
        age: 35
    });
}
