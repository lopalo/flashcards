use crate::model::{LearningSet, Settings};
use super::snackbar::SnackbarMessage;
use yew::prelude::*;

pub type DisplayErrorCtx = UseReducerDispatcher<SnackbarMessage>;

pub type SettingsCtx = UseReducerHandle<Settings>;

pub type LearningSetCtx = UseReducerHandle<LearningSet>;
