mod board;
use iced::keyboard;
use iced::{time, Element, Fill, Subscription};
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    TakeAction(Action),
}

#[derive(Debug, Clone)]
enum Action {
    Left,
    Right,
    ClockWise,
    CounterClockWise,
    FinishFall,
    SoftDrop,
}

pub fn main() -> iced::Result {
    iced::application(board::State::default, update, view)
        .subscription(subscription)
        .run()
}

fn subscription(_state: &board::State) -> Subscription<Message> {
    Subscription::batch([timer(), player_input()])
}

fn timer() -> Subscription<Message> {
    time::every(Duration::from_secs_f64(0.01)).map(|_| Message::Tick)
}

fn player_input() -> Subscription<Message> {
    keyboard::listen().filter_map(|event| {
        if let keyboard::Event::KeyPressed { key, .. } = event {
            match key {
                keyboard::Key::Character(ref c) => match c.as_ref() {
                    "a" => Some(Message::TakeAction(Action::Left)),
                    "d" => Some(Message::TakeAction(Action::Right)),
                    "w" => Some(Message::TakeAction(Action::ClockWise)),
                    "q" => Some(Message::TakeAction(Action::CounterClockWise)),
                    "s" => Some(Message::TakeAction(Action::SoftDrop)),
                    _ => None,
                },
                keyboard::Key::Named(named) => match named {
                    keyboard::key::Named::ArrowLeft => Some(Message::TakeAction(Action::Left)),
                    keyboard::key::Named::ArrowRight => Some(Message::TakeAction(Action::Right)),
                    keyboard::key::Named::ArrowUp => Some(Message::TakeAction(Action::ClockWise)),
                    keyboard::key::Named::ArrowDown => Some(Message::TakeAction(Action::SoftDrop)),
                    keyboard::key::Named::Space => Some(Message::TakeAction(Action::FinishFall)),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    })
}

fn update(state: &mut board::State, message: Message) {
    match message {
        Message::Tick => state.tick(),
        Message::TakeAction(action) => state.take_action(action),
    }
}

fn view(state: &board::State) -> Element<'_, Message> {
    iced::widget::container(state.view())
        .width(Fill)
        .height(Fill)
        .into()
}
