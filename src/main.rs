mod board;
use iced::widget::container;
use iced::{time, Element, Fill, Subscription};
use std::time::Duration;
#[derive(Debug, Clone)]
enum Message {
    Tick,
}
pub fn main() -> iced::Result {
    iced::application(board::State::default, update, view)
        .subscription(timer)
        .run()
}
fn timer(_state: &board::State) -> Subscription<Message> {
    time::every(Duration::from_secs_f64(0.05)).map(|_| Message::Tick)
}
fn update(state: &mut board::State, message: Message) {
    match message {
        Message::Tick => {
            state.tick();
        }
    }
}

fn view(state: &board::State) -> Element<'_, Message> {
    container(container(state.view()).max_width(500))
        .center(Fill)
        .into()
}
