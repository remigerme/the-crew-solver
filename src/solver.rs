use crate::{
    state::{GameError, State},
    task::TaskStatus,
};

impl State {
    pub fn play(&mut self) -> Result<Self, GameError> {
        let mut stack = vec![self.clone()];

        while let Some(state) = stack.pop() {
            // Checking if the game status can be determined, in which case we early exit.
            match state.game_status() {
                TaskStatus::Done => return Ok(state),
                TaskStatus::Failed => continue,
                TaskStatus::Unknown => (),
            };

            // If the status is unknown, the current player tries to play everything he can.
            for card in state
                .get_current_player()
                .get_hand()
                .playable_cards(state.get_current_trick().get(0))
            {
                let mut new_state = state.clone();
                new_state.play_card(&card)?;
                stack.push(new_state);
            }
        }

        // If we end up here, we've tried everything and it didn't work.
        return Err(GameError::NoSolutionFound);
    }
}
