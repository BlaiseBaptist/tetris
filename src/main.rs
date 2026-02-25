mod board;
use iced::keyboard;
use iced::widget::container;
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
        if let keyboard::Event::KeyPressed {
            key: keyboard::Key::Character(key),
            ..
        } = event
        {
            return match key.as_ref() {
                "a" => Some(Message::TakeAction(Action::Left)),
                "d" => Some(Message::TakeAction(Action::Right)),
                _ => None,
            };
        }
        None
    })
}

fn update(state: &mut board::State, message: Message) {
    match message {
        Message::Tick => {
            state.tick();
        }
        Message::TakeAction(action) => {
            state.take_action(action);
        }
    }
}

fn view(state: &board::State) -> Element<'_, Message> {
    container(container(state.view()).max_width(500))
        .center(Fill)
        .into()
}
