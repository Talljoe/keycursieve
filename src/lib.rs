#[macro_use]
extern crate derive_new;

pub mod pipeline;

#[derive(Debug, PartialEq, Clone)]
pub struct KeyEvent {}

#[cfg(test)]
mod tests {
    use crate::pipeline::{Handler, HandlerResult};
    use crate::*;

    struct Dummy;

    impl Handler<KeyEvent> for Dummy {}

    #[test]
    fn it_works() {
        let handler = Dummy {};
        let event = KeyEvent {};
        assert_eq!(
            handler.handle(event.clone()),
            HandlerResult::Continue(event)
        );
    }
}
