pub trait ToAny {
    fn to_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> ToAny for T {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait Component: ToAny {
    fn name(&self) -> String;
    fn update(&mut self);
}

#[macro_export]
macro_rules! components {
    [] => {
        vec![]
    };
    [$head:expr] => {
        vec![Box::new($head)]
    };
    [$($list:expr),*] => {
        vec![$(Box::new($list)),*]
    };
}

pub use components;
