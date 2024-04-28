use std::{mem::transmute, collections::VecDeque, pin::Pin};

use streaming_iterator::StreamingIterator;

use crate::{State, Event, EventType};


pub struct EventIterator<'a, EventSource>
where
    EventSource: Iterator<Item = &'a Event>,
{
    state: Pin<Box<State>>,
    source: EventSource,
    // events stored, after occured CombatEnd
    cached_events: VecDeque<&'a Event>,
    stream_output: Option<StateNonOwnedEvent<'a>>,
}

pub struct StateNonOwnedEvent<'event> {
    state: &'static State,
    event: &'event Event,
}

impl<'a, EventSource> EventIterator<'a, EventSource>
where
    EventSource: Iterator<Item = &'a Event>,
{
    pub fn new<IntoIter>(iter: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = EventSource>,
    {
        Self {
            state: Box::pin(State::new()),
            source: iter.into_iter(),
            cached_events: Default::default(),
            stream_output: None,
        }
    }

    fn next_event(&mut self) {
        let event;

        if self.cached_events.is_empty() {
            event = self.source.next();
        } else {
            event = self.cached_events.pop_front();
        }

        unsafe {
            self.stream_output = event.map(|event| StateNonOwnedEvent::new(&self.state, event));
        }
    }
}

impl<'a, EventSource> StreamingIterator for EventIterator<'a, EventSource>
where
    EventSource: Iterator<Item = &'a Event>,
{
    type Item = StateNonOwnedEvent<'a>;

    fn advance(&mut self) {
        self.next_event();

        let state_event = self.stream_output
            .as_ref()
            .map(|se| se.state_event());

        if let Some((state, Event { timestamp, event: EventType::EndCombat(_)})) = state_event {

        } else {
            state_event.map(|state_event| 
                self.state.handle_event(state_event.1)
            );
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        self.stream_output
            .as_ref()
    }
}

impl<'a, 'event> StateNonOwnedEvent<'event> {
    unsafe fn new(state: &State, event: &'event Event) -> Self {
        Self {
            state: transmute::<&'_ State, &'static State>(state),
            event,
        }
    }

    pub fn state_event(&'a self) -> (&'a State, &'event Event) {
        let state = unsafe {
            transmute::<&'static State, &'a State>(self.state)
        };

        (state, self.event)
    }

    pub fn state(&'a self) -> &'a State {
        self.state_event().0
    }

    pub fn event(&self) -> &'event Event {
        self.state_event().1
    }
}

// pub struct EventIteratorOwned<'b, EventSource>
// where
//     EventSource: Iterator<Item = Event>,
// {
//     state: State,
//     source: EventSource,
//     iter_out: Option<<Self as StreamingIterator>::Item>,
//     _self_lifetime: PhantomData<&'b ()>,
// }

// impl<'b, EventSource> StreamingIterator for EventIteratorOwned<'b, EventSource>
// where
//     EventSource: Iterator<Item = Event>,
// {
//     type Item = (&'b State, Event);

//     fn advance(&mut self) {
//         self.iter_out = self.source.next()
//             .map(|e| unsafe {
//                 (transmute::<&'_ State, &'_ State>(&self.state), e)
//             });
//     }

//     fn get(&self) -> Option<&Self::Item> {
//         self.iter_out
//             .as_ref()
//     }
// }
