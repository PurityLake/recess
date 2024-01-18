use crate::component::Component;

pub struct Entity {
    id: u32,
    name: String,
    pub components: Vec<Box<dyn Component>>,
}

#[allow(dead_code)]
impl Entity {
    pub fn new(id: u32, name: String, components: Vec<Box<dyn Component>>) -> Self {
        Entity {
            id,
            name,
            components,
        }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    pub fn name(self) -> String {
        self.name
    }
}

impl IntoIterator for Entity {
    type Item = Box<dyn Component>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}
