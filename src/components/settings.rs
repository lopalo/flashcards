use super::{
    common::{
        dropdown::Dropdown,
        form::{TextField, TextFieldVariant},
    },
    context::SettingsCtx,
};
use crate::{
    model::{flashcard::Language, settings::SettingsAction},
    speech_synthesis,
};
use yew::prelude::*;

#[function_component(Settings)]
pub fn settings() -> Html {
    let force_update = use_force_update();
    let on_voices_changed_guard = use_mut_ref(|| None);
    let settings: SettingsCtx = use_context().unwrap();

    use_effect(move || {
        let guard = speech_synthesis::set_on_voices_changed(move || {
            force_update.force_update()
        });
        *on_voices_changed_guard.borrow_mut() = Some(guard);
    });

    let repeat_card_distance = &settings.repeat_card_distance;
    let on_repeat_card_distance_change = {
        let dispatcher = settings.dispatcher();
        move |value: String| {
            dispatcher.dispatch(SettingsAction::SetRepeatCardDistance(
                value.parse().unwrap_or(1),
            ))
        }
    };

    let languages = Language::all_languages();
    let voices: Vec<_> = speech_synthesis::get_voices()
        .into_iter()
        .map(|v| v.name().into())
        .collect();
    let selected_voice_language = use_state(|| languages[0]);

    let on_select_voice_language = {
        let setter = selected_voice_language.setter();
        move |language| setter.set(language)
    };

    let on_select_voice = {
        let dispatcher = settings.dispatcher();
        let language = *selected_voice_language;
        move |voice: AttrValue| {
            dispatcher.dispatch(SettingsAction::SetVoice {
                language,
                voice: voice.as_cow().into_owned(),
            })
        }
    };

    let on_select_card_front_language = {
        let dispatcher = settings.dispatcher();
        move |language: Language| {
            dispatcher
                .dispatch(SettingsAction::SetCardFrontSideLanguage(language))
        }
    };

    let on_select_card_back_language = {
        let dispatcher = settings.dispatcher();
        move |language: Language| {
            dispatcher
                .dispatch(SettingsAction::SetCardBackSideLanguage(language))
        }
    };

    html! {
        <div class="mdc-layout-grid settings">
          <div class="mdc-layout-grid__inner">
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <TextField
                variant={TextFieldVariant::Number}
                label="Repeat card distance"
                helper_text={format!(
                  "When you press the 'repeat' button, a flashcard moves forward \
                  {repeat_card_distance} positions"
                )}
                value={repeat_card_distance.to_string()}
                on_change={on_repeat_card_distance_change}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-3">
              <Dropdown<Language>
                label="Voice language"
                items={languages}
                selected={*selected_voice_language}
                on_select={on_select_voice_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-9">
              <span class="voice">
                <Dropdown<AttrValue>
                  label="Voice"
                  items={voices}
                  selected={settings.get_voice(*selected_voice_language).to_owned()}
                  on_select={on_select_voice}
                />
              </span>
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-3">
              <Dropdown<Language>
                label="Default card front side language"
                items={languages}
                selected={settings.default_card_front_side_language}
                on_select={on_select_card_front_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-3">
              <Dropdown<Language>
                label="Default card back side language"
                items={languages}
                selected={settings.default_card_back_side_language}
                on_select={on_select_card_back_language}
              />
            </div>
          </div>
        </div>
    }
}
