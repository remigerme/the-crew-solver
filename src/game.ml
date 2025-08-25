open Card
open Trick
open Hand

type task_status = Done | Failed | Unknown

type player = {
  hand : hand;
  tricks : trick Dynarray.t;
  tasks : task Dynarray.t;
}

and state = {
  players : player array;
  mutable current_trick : trick;
  mutable first_player : int;
}

and task = state -> task_status

let is_captain player =
  let is_trump_4 c = c == Trump 4 in
  Dynarray.exists is_trump_4 player.hand
  || Dynarray.exists (fun t -> Dynarray.exists is_trump_4 t.cards) player.tricks

let retrieve_captain state =
  let ic = ref 0 in
  Array.iteri (fun i p -> if is_captain p then ic := i) state.players;
  !ic

let n_players state = Array.length state.players

let current_player_index state =
  (state.first_player + Dynarray.length state.current_trick.cards)
  mod n_players state

let current_player state = state.players.(current_player_index state)

let game_failed state =
  let task_failed t = t state == Failed in
  let player_failed p = Dynarray.exists task_failed p.tasks in
  Array.exists player_failed state.players

let game_won state =
  let task_done t = t state == Done in
  let player_done p = Dynarray.for_all task_done p.tasks in
  Array.for_all player_done state.players

let game_over state =
  let cards_left =
    Array.fold_left (fun x p -> x + Dynarray.length p.hand) 0 state.players
  in
  nb_cards mod n_players state == cards_left
