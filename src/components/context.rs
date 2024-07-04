use crate::model::{LearningSet, Settings};
use yew::prelude::*;

pub type SettingsCtx = UseReducerHandle<Settings>;

pub type LearningSetCtx = UseReducerHandle<LearningSet>;
