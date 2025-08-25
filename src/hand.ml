open Card

type hand = card Dynarray.t

let playable_cards hand first_card =
  if Dynarray.exists (same_color first_card) hand then
    Dynarray.filter (same_color first_card) hand
  else hand

let copy_without hand card =
  let new_hand = Dynarray.create () in
  Dynarray.iter (fun c -> if c <> card then Dynarray.add_last new_hand c) hand;
  new_hand
