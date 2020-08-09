#[macro_use]
extern crate derive_new;

pub mod pipeline;

#[derive(Debug, PartialEq, Clone)]
pub struct KeyEvent {}

#[cfg(test)]
mod tests {
    use crate::pipeline::{Handler, HandlerAction};
    use crate::*;

    struct Dummy;

    impl Handler<KeyEvent> for Dummy {}

    #[test]
    fn it_works() {
        let mut handler = Dummy {};
        let event = KeyEvent {};
        assert_eq!(
            handler.handle(event.clone()),
            HandlerAction::Continue(event)
        );
    }
}
