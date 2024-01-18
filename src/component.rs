use crate::AToAny;

pub trait Component: AToAny {
    fn name(&self) -> String;
    fn update(&mut self);
}
