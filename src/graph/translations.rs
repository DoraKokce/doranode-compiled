use std::collections::HashMap;

pub struct Translations(pub HashMap<String, String>);

impl Translations {
    pub fn get(&self, index: &str) -> Option<&String> {
        self.0.get(index)
    }
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {{
        $crate::graph::TRANSLATIONS
            .get()
            .and_then(|t| t.get($key))
            .map(|s| s.as_str())
            .unwrap_or($key)
    }};
}
