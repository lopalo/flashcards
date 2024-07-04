use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    SpeechSynthesis, SpeechSynthesisUtterance, SpeechSynthesisVoice,
};

fn speech_synthesis() -> SpeechSynthesis {
    gloo::utils::window().speech_synthesis().unwrap()
}

pub fn get_voices() -> Vec<SpeechSynthesisVoice> {
    let mut voices: Vec<SpeechSynthesisVoice> = speech_synthesis()
        .get_voices()
        .iter()
        .map(|v| v.dyn_into().unwrap())
        .collect();
    voices.sort_by_key(|v| v.name());
    voices
}

#[derive(Clone)]
pub struct SpeechGuard {
    _on_end: Rc<Closure<dyn Fn()>>,
}

pub fn activate<F>(voice_name: &str, text: &str, on_end: F) -> SpeechGuard
where
    F: Fn() + 'static,
{
    deactivate();
    let voices = get_voices();
    let utterance = SpeechSynthesisUtterance::new_with_text(text).unwrap();
    utterance.set_voice(
        voices.into_iter().find(|v| v.name() == voice_name).as_ref(),
    );
    let on_end = Closure::new(on_end);
    utterance.set_onend(Some(on_end.as_ref().unchecked_ref()));
    speech_synthesis().speak(&utterance);
    SpeechGuard {
        _on_end: Rc::new(on_end),
    }
}

pub fn deactivate() {
    speech_synthesis().cancel()
}

#[derive(Clone)]
pub struct OnVoicesChangedGuard {
    _callback: Rc<Closure<dyn Fn()>>,
}

pub fn set_on_voices_changed<F>(callback: F) -> OnVoicesChangedGuard
where
    F: Fn() + 'static,
{
    let callback = Closure::new(callback);
    speech_synthesis()
        .set_onvoiceschanged(Some(callback.as_ref().unchecked_ref()));
    OnVoicesChangedGuard {
        _callback: Rc::new(callback),
    }
}
