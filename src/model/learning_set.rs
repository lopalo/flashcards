use super::flashcard::Flashcard;
use crate::local_storage::LocalStorageRecord;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, rc::Rc};
use yew::prelude::*;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningSet {
    pub queue: VecDeque<Rc<Flashcard>>,
}

pub enum Direction {
    Left,
    Right,
}

pub enum LearningSetAction {
    Replace(LearningSet),
    RotateQueue(Direction),
    MoveHeadItemForward { positions: usize },
}

impl Reducible for LearningSet {
    type Action = LearningSetAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use LearningSetAction::*;
        let this = Rc::make_mut(&mut self);

        match action {
            Replace(other) => *this = other,
            RotateQueue(Direction::Left) => this.queue.rotate_left(1),
            RotateQueue(Direction::Right) => this.queue.rotate_right(1),
            MoveHeadItemForward { positions } => {
                if let Some(head) = this.queue.pop_front() {
                    let pos = positions.min(this.queue.len());
                    this.queue.insert(pos, head)
                };
            }
        };

        self.save_in_local_storage();
        self
    }
}

impl LocalStorageRecord for LearningSet {
    const KEY: &'static str = "learning-set";
}
