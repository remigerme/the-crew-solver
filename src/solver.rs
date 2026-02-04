use crate::{state::State, task::TaskStatus};

impl State {
    pub fn play(&mut self) -> (usize, usize, usize) {
        let mut stack = vec![self.clone()];

        // We count the number of dead/successful branches
        let mut n_done = 0;
        let mut n_failed = 0;
        let mut n_unknown = 0;

        while let Some(state) = stack.pop() {
            // Checking if the game status can be determined, in which case
            // we update the counters and do not early exit anymore.
            match state.game_status() {
                TaskStatus::Done => {
                    n_done += 1;
                    continue;
                }
                TaskStatus::Failed => {
                    n_failed += 1;
                    continue;
                }
                TaskStatus::Unknown => n_unknown += 1,
            };

            // If the status is unknown, the current player tries to play everything he can.
            for card in state
                .get_current_player()
                .get_hand()
                .playable_cards(state.get_current_trick().get(0))
            {
                let mut new_state = state.clone();
                new_state.play_card(&card).unwrap();
                stack.push(new_state);
            }
        }

        (n_done, n_failed, n_unknown)
    }
}
