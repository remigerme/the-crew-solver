# Tasks

This folder contains the implementation of the different tasks of the game.

## Implementing a new task

In general, when defining a new task, the goal is to define a **public** struct:

- providing a `new` method to construct the task
- implementing the [`Task`](../task.rs) trait.

[`new`]: TaskWinCards::new

Once it is done, you

You can check the provided [example](win_cards.rs).
