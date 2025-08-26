# Tasks

This folder contains the implementation of the different tasks of the game.

## What is a task?

First, we need to take a look at possible task statuses. These statuses are:

- `Done`: task is already completed by the player
- `Failed`: task is failed and cannot be completed anymore
- `Unknown`: if it is not yet possible to state wether the task is done or has failed.

At a given state of the game, each task is in of the three statuses. Intuitively, a task is a function from the current state of the game to one of the statuses above.

The corresponding Rust code is available in [task.rs](../task.rs):

```rust
pub enum TaskStatus {
    Done,
    Unknown,
    Failed,
}

pub trait Task {
    fn eval(&self, state: &State, ip: usize) -> TaskStatus;
}
```

A task is just an object providing an `eval` function, which takes:

- the task itself (might be "win the 1, 2, and 3 of blue" for example) `self`
- a given state of the game `state`
- the index of the player which has to fulfill the task `ip`

and returns a `TaskStatus`.

Okay, but...

## What does the state contain?

Refer to the docstrings in [`game.rs`](../game.rs). It basically contains all information on current trick, as well as past tricks. Many util functions are provided, but if one is missing, feel free to add it via a PR.

## Implementing a new task

To create a task:

1. create new file in this folder which will contain the task
2. define a public struct with a name prefixed with `Task` - feel free to define as many fields as needed, as well as private methods
3. implement a public `new` method to construct the task (keep it generic)
4. implement the [`Task`](../task.rs) trait - that is, implement the logic of the task
5. (optional) write some tests to ensure the code behaves as expected
6. add your public module to [`task.rs`](../task.rs).

You can check the provided [example](win_cards.rs).

---

Feel free to open an issue if you have any question.

Thank you for your help!
