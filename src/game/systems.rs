use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match sim_state.get() {
            SimulationState::Running => {
                next_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            }
            SimulationState::Paused => {
                next_state.set(SimulationState::Running);
                println!("Simulation Resumed");
            }
        }
    }
}

pub fn pause_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}
