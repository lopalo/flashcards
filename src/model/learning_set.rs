use super::flashcard::Flashcard;
use gloo::storage::{LocalStorage, Storage};
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
        };
        self.save_in_local_storage();
        self
    }
}

impl LearningSet {
    const KEY: &'static str = "flashcards:learning-set";

    fn save_in_local_storage(&self) {
        LocalStorage::set(Self::KEY, self).unwrap();
    }

    pub fn restore_from_local_storage() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }
}
