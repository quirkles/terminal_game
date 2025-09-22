use crate::particle::ParticleId;

#[derive(Clone, Debug)]
pub enum Collision {
    // A refuel-type collision occurring at a specific console cell
    // with the list of participating particle IDs (stable across storage changes).
    Refuel {
        participants: Vec<ParticleId>,
    },
}
