(*******************)
(* Modelling cards *)
(*******************)
let nb_cards = 40

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

let copy_without hand card =
  let new_hand = Dynarray.create () in
  Dynarray.iter (fun c -> if c <> card then Dynarray.add_last new_hand c) hand;
  new_hand

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
  mutable current_trick : trick;
  mutable first_player : int;
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

let game_won state =
  let task_done t = t state == Done in
  let player_done p = Dynarray.for_all task_done p.tasks in
  Array.for_all player_done state.players

let game_over state =
  let cards_left =
    Array.fold_left (fun x p -> x + Dynarray.length p.hand) 0 state.players
  in
  nb_cards mod n_players state == cards_left

(******************)
(* MAIN GAME LOOP *)
(******************)
let rec play s =
  if game_won s then print_string "Found a winning state.\n";
  if game_over s then ()
  else
    let p = current_player s in
    let pid = current_player_index s in
    let playable_hand =
      if Dynarray.length s.current_trick == 0 then p.hand
      else
        let first_card = Dynarray.get s.current_trick 0 in
        playable_cards p.hand first_card
    in
    for i = 0 to Dynarray.length playable_hand - 1 do
      (* Preparing next state *)
      let c = Dynarray.get playable_hand i in
      let new_hand = copy_without p.hand c in
      let new_p = { p with hand = new_hand } in
      (* Saving existing state *)
      let old_first_player = s.first_player in
      let old_trick = Dynarray.copy s.current_trick in
      let old_players =
        Array.map
          (fun p -> { p with tricks = Dynarray.copy p.tricks })
          s.players
      in
      (* Updating state *)
      s.players.(pid) <- new_p;
      Dynarray.add_last s.current_trick c;
      (* More update if we finished the trick *)
      if n_players s == Dynarray.length s.current_trick then (
        let winner_rel = winner s.current_trick in
        let winner = (winner_rel + s.first_player) mod n_players s in
        s.first_player <- winner;
        Dynarray.add_last s.players.(winner).tricks
          (Dynarray.copy s.current_trick);
        Dynarray.clear s.current_trick);
      (* Recursive call *)
      if not (game_failed s) then play s;
      (* Resetting state *)
      s.first_player <- old_first_player;
      s.current_trick <- old_trick;
      Array.iteri (fun i old_p -> s.players.(i) <- old_p) old_players
    done

let init distrib =
  let n_players = Dynarray.length distrib in
  let players =
    Array.make n_players
      {
        hand = Dynarray.create ();
        tricks = Dynarray.create ();
        tasks = Dynarray.create ();
      }
  in
  for i = 0 to n_players - 1 do
    let hand, tasks = Dynarray.get distrib i in
    players.(i) <- { hand; tricks = Dynarray.create (); tasks }
  done;
  let s = { players; current_trick = Dynarray.create (); first_player = 0 } in
  let captain = retrieve_captain s in
  s.first_player <- captain;
  s
