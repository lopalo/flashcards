use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/training")]
    Training,
    #[at("/learning-set")]
    LearningSet,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn page_title(&self) -> &'static str {
        match self {
            Self::Training => "Training",
            Self::LearningSet => "Learning Set",
            Self::Settings => "Settings",
            Self::Home | Self::NotFound => "",
        }
    }

    pub fn navigation_icon(&self) -> &'static str {
        match self {
            Self::Training => "school",
            Self::LearningSet => "book",
            Self::Settings => "settings",
            Self::Home => "home",
            Self::NotFound => "question_mark",
        }
    }
}

impl yew::html::ImplicitClone for Route {}
