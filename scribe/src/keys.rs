use iced::subscription;
use iced::{
    futures::{channel::mpsc, FutureExt, SinkExt, StreamExt, TryFutureExt},
    Subscription,
};
use rdev::listen;
use std::any::TypeId;
use std::thread;

pub enum State {
    Starting,
    Ready(mpsc::UnboundedReceiver<rdev::Event>),
}
#[derive(Debug, Clone)]
pub enum Event {
    Ready,
    KeyReceived(rdev::Event),
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::UnboundedSender<rdev::Event>);

pub fn bind() -> Subscription<Option<Event>> {
    struct Keys;

    subscription::unfold(TypeId::of::<Keys>(), State::Starting, |state| async move {
        match state {
            State::Starting => {
                let (mut sender, receiver) = mpsc::unbounded();
                thread::spawn(move || {
                    listen(move |event| {
                        sender
                            .send(event)
                            .unwrap_or_else(|e| println!("Could not send event {:?}", e))
                            .now_or_never();
                    })
                    .expect("Could not listen");
                });
                (Some(Event::Ready), State::Ready(receiver))
            }
            State::Ready(mut input) => {
                let received = input.next().await;
                match received {
                    Some(key) => (Some(Event::KeyReceived(key)), State::Ready(input)),
                    None => (None, State::Ready(input)),
                }
            }
        }
    })
}
