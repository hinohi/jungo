pub mod light_random;
pub mod mc;
pub mod mcts;
pub mod minimax;
pub mod random;

pub use light_random::LightRandomAI;
pub use mc::MonteCarloAI;
pub use mcts::Mcts;
pub use minimax::MinimaxAI;
pub use random::RandomAI;
