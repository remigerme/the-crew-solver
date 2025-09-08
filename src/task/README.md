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

Refer to the docstrings in [`state.rs`](../state.rs). It basically contains all information on current trick, as well as past tricks. Many util functions are provided, but if one is missing, feel free to add it via a PR.

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

## Checklist

**Status: 62/96.**

From [BGG forum](https://boardgamegeek.com/thread/2631311/all-the-mission-cards):

- done in [`TaskWinCards`](win_cards.rs)
  - [x] (1/1/1) Win the pink 3
  - [x] (1/1/1) Win the yellow 1
  - [x] (1/1/1) Win the blue 4
  - [x] (1/1/1) Win the green 6
  - [x] (3/4/5) Win all four 3s.
  - [x] (4/5/6) Win all four 9s.
  - [x] (2/3/3) Win the blue 1,2 and 3
  - [x] (2/2/3) Win the blue 6 and the yellow 7
  - [x] (2/2/3) Win the pink 5 and yellow 6
  - [x] (2/2/3) Win the green 5 and blue 8
  - [x] (2/2/3) Win the blue 5 and pink 8
  - [x] (2/2/3) Win the pink 9 and yellow 8
  - [x] (2/2/2) Win the pink 1 and green 7
  - [x] (2/3/3) Win the yellow 9 and blue 7
  - [x] (3/4/4) Win the green 3 and the yellow 4 and 5
  - [x] (1/1/1) Win the 3 submarine
- done in [`TaskDontWinCards`](dont_win_cards.rs)
  - [x] (2/2/2) Win no pink cards.
  - [x] (1/1/1) Win no submarines
  - [x] (2/2/2) Don’t win any green cards
  - [x] (2/2/2) Don’t win any yellow cards
  - [x] (3/3/3) Don’t win any pink or blue cards
  - [x] (3/3/3) Don’t win any yellow or green cards
  - [x] (3/3/2) Don’t win any 8s or 9s
  - [x] (1/1/1) Don’t win any 9s
  - [x] (1/2/2) Don’t win any 5s
  - [x] (2/2/2) Don’t win any 1s
  - [x] (3/3/3) Don’t win any 1s, 2s or 3s
- done in [`TaskDontOpenTrickWith`](dont_open_trick_with.rs)
  - [x] (4/3/3) Don’t open a trick with a pink, yellow or blue card
  - [x] (2/1/1) Don’t open a trick with a pink or green card
- done in [`TaskWinTrickWith`](win_trick_with.rs)
  - [x] (2/3/3) Win a trick with a 6.
  - [x] (2/3/4) Win a trick with a 5.
  - [x] (3/4/5) Win a trick with a 3.
  - [x] (1/2/2) Win a 5 with a 7.
  - [x] (3/4/5) Win an 8 with a 4.
  - [x] (2/3/4) Win any 6 with a another 6.
  - [x] (3/4/5) Win a trick with a 2.
- done in [`TaskWinNbTricksComparedCaptain`](win_nb_tricks_compared_captain.rs)
  - [x] (2/2/3) Win more tricks than the captain (the captain can’t take this mission)
  - [x] (2/2/2) Win fewer tricks than the captain (the captain can’t take this mission)
  - [x] (4/3/3) Win the same number of tricks as the captain (the captain can’t take this mission)
- done in [`TaskWinTrickWithPred`](win_trick_with_pred.rs)
  - [x] (2/3/3) Win a trick where all cards are of lower value than 7 without submarines
  - [x] (2/3/4) Win a trick where all cards are of greater value than 5
  - [x] (2/5/6) Win a trick that has only even numbers (2,4,6,8)
  - [x] (2/4/5) Win a trick that has only odd numbers (1,3,5,7,9)
  - [x] (3/3/4) Win a trick with a total value higher than 23/28/31 (3/4/5 players) without submarines
  - [x] (3/3/4) Win a trick with a total value lower than 8/12/16 (3/4/5 players) without submarines
  - [x] (3/3/4) Win a trick with a total value of 22 or 23
  - [x] (2/3/3) Win the same amount of green and yellow cards in a trick (more than 0)
  - [x] (2/3/3) Win the same amount of pink and blue cards in a trick (more than 0)
  - [x] (3/3/3) Win the pink 7 with a submarine
  - [x] (3/3/3) Win the green 9 with a submarine
  - [x] (3/4/5) Win the green 2 in the last trick
- done in [`TaskDontWinTricks`](dont_win_tricks.rs)
  - [x] (1/2/3) Don’t win any of the first four tricks
  - [x] (1/2/2) Don’t win any of the first three tricks
  - [x] (2/3/3) Don’t win any of the first five tricks
  - [x] (4/3/3) Don’t win any tricks
- done in [`TaskWinTricks`](win_tricks.rs)
  - [x] (2/3/3) Win the last trick
  - [x] (2/3/4) Win the first three tricks
  - [x] (1/1/2) Win the first two tricks
  - [x] (1/1/1) Win the first trick
  - [x] (3/4/4) Win the first and the last trick
  - [x] (4/4/4) I win only the last trick
  - [x] (4/3/3) I win only the first trick
- [ ] (2/3/3) Win more tricks than everyone else
- [ ] (3/4/5) Win more tricks than everyone else together
- [ ] (2/2/3) Win fewer tricks than everyone else
- [ ] (3/4/5) Win at least three 5s.
- [ ] (3/4/5) Win at least three 9s.
- [ ] (2/2/2) Win at least two 7s.
- [ ] (3/4/4) Win exactly three 6s
- [ ] (2/3/3) Win exactly two 9s
- [ ] (4/4/4) Win exactly one pink and one green card
- [ ] (3/3/3) Win at least seven yellow cards
- [ ] (2/3/3) Win at least five pink cards
- [ ] (3/4/4) Win exactly two green cards.
- [ ] (3/4/4) Win exactly two blue cards.
- [ ] (3/3/4) Win exactly one pink card.
- [ ] (2/3/4) Win at least one card of each colour (excluding submarines)
- [ ] (3/4/5) Win all cards of at least one colour (excluding submarines)
- [ ] (3/3/3) Win exactly one submarine (deal new cards if someone has all submarines in hand)
- [ ] (3/3/3) Win the 1 submarine and no other (deal new cards if someone has submarines no. 1 and 4 or 1,2,3 in hand)
- [ ] (3/3/3) Win the 2 submarine and no other (deal new cards if someone has submarines no. 2 and 4 or 1,2,3 in hand)
- [ ] (3/3/4) Win exactly two submarines (deal new cards if someone has submarines no. 2,3,4 in hand)
- [ ] (3/4/4) Win exactly three submarines (deal new cards if someone has all submarines in hand)
- [ ] (3/2/2) Do not win two consecutive tricks.
- [ ] (3/2/2) Win exactly one trick
- [ ] (2/2/2) Win exactly two tricks
- [ ] (1/1/1) Win two consecutive tricks
- [ ] (2/3/4) Win three consecutive tricks
- [ ] (2/3/5) Win exactly four tricks
- [ ] (3/3/4) Win exactly three consecutive tricks.
- [ ] (3/3/3) Win exactly two consecutive tricks.
- [ ] (3/2/2) Win X tricks (predict the exact number and show)
- [ ] (4/3/3) Win X tricks (predict the exact number but keep hidden)
- [ ] (4/4/4) Win the same amount of pink and yellow cards (more than 0)
- [ ] (1/1/1) Win more yellow cards than blue cards (0 blue cards are allowed)
- [ ] (1/1/1) Win more pink cards than green cards (0 green cards are allowed)
