use std::ops::Index;

use super::effect::EarnedPointType;
use super::enemy::FormationIndex;

use crate::app::game::enemy::EnemyType;
use crate::framework::types::Vec2I;

pub struct EventQueue {
    queue: Vec<EventType>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn push(&mut self, event: EventType) {
        self.queue.push(event);
    }
}

impl Index<usize> for EventQueue {
    type Output = EventType;
    fn index(&self, i: usize) -> &Self::Output {
        &self.queue[i]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    MyShot(Vec2I, bool, i32),
    AddScore(u32),
    EneShot(Vec2I),
    EarnPointEffect(EarnedPointType, Vec2I),
    EnemyExplosion(Vec2I, i32, EnemyType),
    PlayerExplosion(Vec2I),
    DeadPlayer,
    StartCaptureAttack(FormationIndex),
    EndCaptureAttack,
    CapturePlayer(Vec2I),
    CapturePlayerCompleted,
    CaptureSequenceEnded,
    SpawnCapturedFighter(Vec2I, FormationIndex),
    RecapturePlayer(FormationIndex),
    MovePlayerHomePos,
    RecaptureEnded,
    EscapeCapturing,
    EscapeEnded,
    CapturedFighterDestroyed,
    PlaySe(u32, &'static str),
}
