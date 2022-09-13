//! A demonstration of the effects! and handler! macros, which allow you to group effects into one
//! type, and use one handler for all of them. The primary benefit of this is that this handler can
//! own or mutably borrow some data, and that data can be accessed by all of the arms of the
//! handler.
//! Here we use State as a classic (and generic!) example of a side effect. The same program could
//! just be written using a mutable variable, but that's no fun.

#![feature(generators)]
#![feature(generator_trait)]

use std::marker::PhantomData;

use effing_mad::{effectful, handle, run};

fn main() {
    let mut state = 34;
    let handled = handle!(
        use_state(),
        State<i32>,
        get(_) => state,
        put(v) => state = v,
    );
    run(handled);
    println!("final value: {}", state);
}

effing_mad::effects! {
    State<T> {
        fn<T> get(_marker: PhantomData<T>) -> T;
        fn<T> put(v: T) -> ();
    }
}

// Rust encourages immutability!
#[effectful(State<i32>)]
fn use_state() {
    let initial = yield State::get(PhantomData);
    println!("initial value: {}", initial);
    yield State::put(initial + 5);
}
