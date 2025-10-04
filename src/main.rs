use iced::{Length, Task};

#[derive(Default)]
struct TextEditorState {
    content: iced::widget::text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(iced::widget::text_editor::Action),
}

fn main() -> iced::Result {
    iced::application("hoditor", update, view).run()
}

fn update(text_editor: &mut TextEditorState, msg: Message) {
    match msg {
        Message::ContentChanged(action) => {
            text_editor.content.perform(action);
        }
    }
}

fn view(text_editor: &TextEditorState) -> iced::Element<Message> {
    iced::widget::text_editor(&text_editor.content)
        .on_action(Message::ContentChanged)
        .into()
}
