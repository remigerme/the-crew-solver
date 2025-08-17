(*******************)
(* Modelling cards *)
(*******************)
type card =
  | Red of int
  | Blue of int
  | Green of int
  | Yellow of int
  | Trump of int

let valid_card = function
  | Red x | Blue x | Green x | Yellow x -> 1 <= x && x <= 9
  | Trump x -> 1 <= x && x <= 4

let value = function Red x | Blue x | Green x | Yellow x | Trump x -> x
let is_trump = function Trump _ -> true | _ -> false

let same_color a b =
  match (a, b) with
  | Red _, Red _ -> true
  | Blue _, Blue _ -> true
  | Green _, Green _ -> true
  | Yellow _, Yellow _ -> true
  | Trump _, Trump _ -> true
  | _ -> false

(********************)
(* Modelling tricks *)
(********************)
type trick = card Dynarray.t

let argmax_trick f trick =
  let best_i = ref 0 in
  Dynarray.iteri
    (fun i card ->
      if f card && value card > value (Dynarray.get trick !best_i) then
        best_i := i)
    trick;
  !best_i

let winner trick =
  if Dynarray.exists is_trump trick then argmax_trick is_trump trick
  else argmax_trick (same_color (Dynarray.get trick 0)) trick

(*******************)
(* Modelling hands *)
(*******************)
type hand = card Dynarray.t

let playable_cards hand first_card =
  if Dynarray.exists (same_color first_card) hand then
    Dynarray.filter (same_color first_card) hand
  else hand

(*******************************)
(* Modelling tasks and players *)
(*******************************)
type task_status = Done | Failed | Unknown

type player = {
  hand : hand;
  tricks : trick Dynarray.t;
  tasks : task Dynarray.t;
}

and state = {
  players : player array;
  current_trick : trick;
  first_player : int;
}

and task = state -> task_status

let is_captain player =
  let is_trump_4 c = c == Trump 4 in
  Dynarray.exists is_trump_4 player.hand
  || Dynarray.exists (Dynarray.exists is_trump_4) player.tricks

let retrieve_captain state =
  let ic = ref 0 in
  Array.iteri (fun i p -> if is_captain p then ic := i) state.players;
  !ic

let n_players state = Array.length state.players

let current_player_index state =
  (state.first_player + Dynarray.length state.current_trick) mod n_players state

let current_player state = state.players.(current_player_index state)

let game_failed state =
  let task_failed t = t state == Failed in
  let player_failed p = Dynarray.exists task_failed p.tasks in
  Array.exists player_failed state.players

(********)

let play s =
  let p = current_player s in
  (* Opening a new trick *)
  if Dynarray.length s.current_trick == 0 then ()
  else
    let first_card = Dynarray.get s.current_trick 0 in
    let c = playable_cards p.hand first_card in
    ()
