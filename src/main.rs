use iced::{
    Length::Fill,
    Theme,
    widget::{Container, button, column, container, pick_list, row, text},
};

#[derive(Default)]
struct Counter {
    value: i64,
    theme: Option<Theme>,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    ThemeChanged(Theme),
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::ThemeChanged(theme) => self.theme = Some(theme),
        }
    }

    fn view(&self) -> Container<'_, Message> {
        container(
            column![
                text("こんばんは！").size(30),
                container(
                    row![
                        button("-").on_press(Message::Decrement),
                        container(text(self.value))
                            .width(20)
                            .align_x(iced::Alignment::Center),
                        button("+").on_press(Message::Increment),
                    ]
                    .align_y(iced::Alignment::Center)
                    .spacing(20)
                )
                .padding(10)
                .style(container::bordered_box),
                pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged)
                    .placeholder("choose a theme...")
            ]
            .align_x(iced::Alignment::Center)
            .spacing(20),
        )
        .center_x(Fill)
        .padding(iced::padding::top(10))
    }

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("IcedTesting")
        .theme(Counter::theme)
        .run()
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter::default();

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}
