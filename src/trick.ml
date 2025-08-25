open Card

type trick = { mutable idx : int; cards : card Dynarray.t }

let argmax_trick f cards =
  let best_i = ref 0 in
  Dynarray.iteri
    (fun i card ->
      if f card && value card > value (Dynarray.get cards !best_i) then
        best_i := i)
    cards;
  !best_i

let winner trick =
  let cards = trick.cards in
  if Dynarray.exists is_trump cards then argmax_trick is_trump cards
  else argmax_trick (same_color (Dynarray.get cards 0)) cards

let prepare_next trick =
  Dynarray.clear trick.cards;
  trick.idx <- trick.idx + 1
