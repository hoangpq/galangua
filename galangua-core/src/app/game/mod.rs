pub mod effect;
pub mod enemy;
mod event_queue;
pub mod game_manager;
mod player;
pub mod score_holder;

pub use self::event_queue::{EventQueue, EventType};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CaptureState {
    NoCapture,
    CaptureAttacking,
    Capturing,
    Captured,
    Recapturing,
}
