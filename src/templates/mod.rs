pub mod android_player;
pub mod ig_note;
pub mod ios_glass;
pub mod traits;

use std::collections::HashMap;
use std::sync::Arc;
use traits::CardTemplate;

pub struct TemplateRegistry {
    templates: HashMap<&'static str, Arc<dyn CardTemplate>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            templates: HashMap::new(),
        };
        reg.register(Arc::new(ios_glass::IosGlassTemplate::new()));
        reg.register(Arc::new(android_player::AndroidTemplate::new()));
        reg.register(Arc::new(ig_note::IgNoteTemplate::new()));
        reg
    }

    pub fn register(&mut self, template: Arc<dyn CardTemplate>) {
        self.templates.insert(template.id(), template);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn CardTemplate>> {
        self.templates.get(id).cloned()
    }

    pub fn list_all(&self) -> Vec<Arc<dyn CardTemplate>> {
        self.templates.values().cloned().collect()
    }
}
