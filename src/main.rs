#[derive(Default)]
struct State;

#[derive(Debug)]
enum Message {}

fn main() -> iced::Result {
    iced::application("hoditor", update, view).run()
}

fn update(state: &mut State, message: Message) {
    unimplemented!();
}

fn view(state: &State) -> iced::Element<Message> {
    iced::widget::text("Hello World!").into()
}
