use crate::prelude::*;

#[system]
pub fn end_turn(
  #[resource] turn_state: &mut TurnState
){
  *turn_state = match turn_state {
    TurnState::WaitingInput => return,
    TurnState::Player => TurnState::Enemy,
    TurnState::Enemy => TurnState::WaitingInput
  }
}