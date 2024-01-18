#![feature(box_into_inner)]
use component::Component;

use entity::Entity;
use query::Query;

mod component;
mod entity;
mod query;

pub trait AToAny {
    fn to_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AToAny for T {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[allow(dead_code)]
#[derive(Default, Clone, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl From<Position> for Box<dyn Component> {
    fn from(value: Position) -> Self {
        Box::new(value)
    }
}

impl TryFrom<Box<dyn Component>> for Position {
    type Error = ();

    fn try_from(value: Box<dyn Component>) -> Result<Self, Self::Error> {
        let any = value.to_any();
        if let Some(pos) = any.downcast_ref::<Position>() {
            Ok(pos.clone())
        } else {
            Err(())
        }
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

fn main() {
    let query = Query::<Position>::new();
    let _pos = Position::new(10, 10);
    let _e = Entity::new(0, "player".into(), vec![Box::new(_pos)]);
    if let Ok(query) = query.query(_e) {
        println!("{:?}", query);
    }
}
