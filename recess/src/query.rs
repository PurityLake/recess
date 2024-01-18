use crate::component::Component;
use crate::entity::Entity;

#[derive(Default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components;
    use procmacros::ComponentDerive;

    #[allow(dead_code)]
    #[derive(ComponentDerive, Default, Clone, Debug)]
    struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        fn new(x: i32, y: i32) -> Self {
            Position { x, y }
        }
    }

    impl Component for Position {
        fn name(&self) -> String {
            "Position".into()
        }

        fn update(&mut self) {
            println!("HERE")
        }
    }

    #[test]
    fn test_query() {
        let query = Query::<Position>::new();
        let pos = Position::new(10, 10);
        let e = Entity::new(0, "player".into(), components![pos]);
        if let Ok(position) = query.query(e) {
            assert!(position.x == 10);
            assert!(position.y == 10);
        }
    }
}
