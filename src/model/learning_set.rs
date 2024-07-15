use super::flashcard::Flashcard;
use crate::local_storage::LocalStorageRecord;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, rc::Rc};
use yew::Reducible;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningSet {
    pub queue: VecDeque<Rc<Flashcard>>,
}

impl LearningSet {
    fn flashcard_index(&self, flashcard_id: &str) -> Option<usize> {
        self.queue.iter().position(|item| (item.id == flashcard_id))
    }
}

pub enum Direction {
    Left,
    Right,
}

pub enum LearningSetAction {
    Replace(LearningSet),
    RotateQueue(Direction),
    MoveHeadCardForward { positions: usize },
    AppendCard(Flashcard),
    ReplaceCard(Flashcard),
    DeleteCard { flashcard_id: String },
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
            MoveHeadCardForward { positions } => {
                if let Some(head) = this.queue.pop_front() {
                    let pos = positions.min(this.queue.len());
                    this.queue.insert(pos, head)
                };
            }
            AppendCard(flashcard) => this.queue.push_back(flashcard.into()),
            ReplaceCard(flashcard) => {
                if let Some(idx) = this.flashcard_index(&flashcard.id) {
                    this.queue[idx] = flashcard.into();
                }
            }
            DeleteCard { flashcard_id } => {
                if let Some(idx) = this.flashcard_index(&flashcard_id) {
                    this.queue.remove(idx);
                }
            }
        };

        self.save_in_local_storage();
        self
    }
}

impl LocalStorageRecord for LearningSet {
    const KEY: &'static str = "learning-set";
}
