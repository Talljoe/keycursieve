#[derive(PartialEq, Debug, Clone)]
pub enum HandlerResult<'e> {
    /// Continue on to the next handler.
    Continue(&'e KeyEvent),
    /// Restart the pipeline from the top.
    Restart(&'e KeyEvent),
    /// Pause the pipeline and resume it in the future.
    Yield,
    /// End processing of this event.
    Halt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyEvent {}

pub trait KeyHandler {
    fn process<'e>(&self, event: &'e KeyEvent) -> HandlerResult<'e> {
        HandlerResult::Continue(event)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    struct Dummy;

    impl KeyHandler for Dummy {}

    #[test]
    fn it_works() {
        let handler = Dummy {};
        let event = KeyEvent {};
        assert_eq!(
            handler.process(&event),
            HandlerResult::Continue(&event)
        );
    }
}
