// Defines game event types and their associated payloads.

#[derive(Clone, Debug)]
pub enum GameEvent {
    // A refuel event between a rocket and a fuel cell, identified by their indices in the scene.
    Refuel {
        rocket_idx: usize,
        fuel_cell_idx: usize,
    },
}
