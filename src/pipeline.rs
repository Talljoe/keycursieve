#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HandlerAction<Event> {
    /// Continue on to the next handler.
    Continue(Event),
    /// Restart the pipeline from the top.
    Restart(Event),
    /// End processing of this event.
    Halt,
}

pub trait Handler<Event> {
    fn handle(&mut self, event: Event) -> HandlerAction<Event> {
        HandlerAction::Continue(event)
    }
}

pub trait Pipeline<Event> {
    fn process(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }
}

#[derive(new)]
pub struct HandlerPipeline<'h, Event> {
    handlers: Vec<&'h mut dyn Handler<Event>>,
}

impl<'h, Event> Pipeline<Event> for HandlerPipeline<'h, Event> {
    fn process<'e>(&mut self, event: Event) -> Option<Event> {
        let mut event = event;
        'restart: loop {
            for handler in &mut self.handlers {
                match handler.handle(event) {
                    HandlerAction::Halt => return None,
                    HandlerAction::Continue(new_event) => event = new_event,
                    HandlerAction::Restart(new_event) => {
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
    use speculate::speculate;

    #[derive(PartialEq, Debug, Clone, Default)]
    struct Event(i32);

    struct AddOne {}

    impl Handler<Event> for AddOne {
        fn handle(&mut self, Event(value): Event) -> HandlerAction<Event> {
            HandlerAction::Continue(Event(value + 1))
        }
    }

    #[derive(Default)]
    struct Counter {
        count: i32,
    }

    impl Counter {
        fn get_count(&self) -> i32 {
            self.count
        }
    }

    impl Handler<Event> for Counter {
        fn handle(&mut self, event: Event) -> HandlerAction<Event> {
            self.count += 1;
            HandlerAction::Continue(event)
        }
    }

    struct Halter {}

    impl Handler<Event> for Halter {
        fn handle(&mut self, _: Event) -> HandlerAction<Event> {
            HandlerAction::Halt
        }
    }

    #[test]
    fn it_calls_handler() {
        let mut handler = AddOne {};
        let mut pipeline = HandlerPipeline::new(vec![&mut handler]);
        let result = pipeline.process(Event(1));
        assert_eq!(result, Some(Event(2)));
    }

    speculate! {
        describe "given a pipeline" {
            before {
                let mut halter = Halter {};
                let mut counter = Counter::default();
                let mut pipeline = HandlerPipeline::new(vec![
                    &mut halter, &mut counter,
                ]);
            }

            describe "when called with an event" {
                before {
                    let _result = pipeline.process(Event::default());
                }

                it "should not call later handlers" {
                    assert_eq!(counter.get_count(), 0);
                }

                it "should return None" {
                    assert!(_result.is_none());
                }
            }

        }
    }
}
