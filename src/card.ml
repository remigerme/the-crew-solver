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
