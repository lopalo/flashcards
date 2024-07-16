use gloo::storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) trait LocalStorageRecord:
    Default + Serialize + DeserializeOwned
{
    const KEY: &'static str;

    fn key() -> String {
        format!("flashcards:{}", Self::KEY)
    }

    fn save_in_local_storage(&self) {
        LocalStorage::set(Self::key(), self).unwrap();
    }

    fn restore_from_local_storage() -> Self {
        LocalStorage::get(Self::key()).unwrap_or_default()
    }
}
