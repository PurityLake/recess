use crate::component::Component;
use crate::entity::Entity;

pub struct Query<Types: Default + Component + Clone + 'static> {
    pub types: Types,
}

impl<Types: Default + Component + Clone + 'static> Query<Types> {
    pub fn new() -> Self {
        Query {
            types: Types::default(),
        }
    }

    pub fn query(self, e: Entity) -> Result<Types, bool> {
        for comp in e.components.iter() {
            let any = comp.to_any();
            match any.downcast_ref::<Types>() {
                Some(types) => {
                    return Ok(types.clone());
                }
                None => continue,
            }
        }
        Err(false)
    }
}
