use crate::model::{LearningSet, Settings};
use std::rc::Rc;
use yew::prelude::*;

pub type SettingsCtx = Rc<Settings>;

pub type LearningSetCtx = UseReducerHandle<LearningSet>;
