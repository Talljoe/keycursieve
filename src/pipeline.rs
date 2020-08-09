#[derive(PartialEq, Debug, Clone)]
pub enum HandlerResult<Event> {
    /// Continue on to the next handler.
    Continue(Event),
    /// Restart the pipeline from the top.
    Restart(Event),
    /// End processing of this event.
    Halt,
}

pub trait Handler<Event> {
    fn handle(&self, event: Event) -> HandlerResult<Event> {
        HandlerResult::Continue(event)
    }
}

pub trait Pipeline<Event> {
    fn process(&self, event: Event) -> Option<Event> {
        Some(event)
    }
}

#[derive(new)]
pub struct HandlerPipeline<'h, Event> {
    handlers: Vec<&'h dyn Handler<Event>>,
}

impl<'h, Event> Pipeline<Event> for HandlerPipeline<'h, Event> {
    fn process<'e>(&self, event: Event) -> Option<Event> {
        let mut event = event;
        'restart: loop {
            for handler in self.handlers.clone() {
                match handler.handle(event) {
                    HandlerResult::Halt => return None,
                    HandlerResult::Continue(new_event) => event = new_event,
                    HandlerResult::Restart(new_event) => {
                        event = new_event;
                        continue 'restart;
                    }
                }
            }

            return Some(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::*;

    struct AddOne;

    #[derive(PartialEq, Debug, Clone)]
    struct Event(i32);

    impl Handler<Event> for AddOne {
        fn handle(&self, event: Event) -> HandlerResult<Event> {
            match event {
                Event(x) => HandlerResult::Continue(Event(x + 1)),
            }
        }
    }

    #[test]
    fn it_calls_handler() {
        let handler = AddOne {};
        let pipeline = HandlerPipeline::new(vec![&handler]);
        let result = pipeline.process(Event(1));
        assert_eq!(result, Some(Event(2)));
    }
}
