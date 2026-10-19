enum Message {
    Event(iced::event::Event),
}

fn update(state: &mut u32, message: Message) {
    match message {
        Message::Event(event) => {
            if let iced::event::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                key, ..
            }) = event
            {
                println!("key {key:?} was pressed");
                *state += 1;
            }
        }
    }
}

fn view(state: &u32) -> iced::Element<'_, Message> {
    iced::widget::text!("keys pressed: {state}").into()
}

fn main() -> iced::Result {
    iced::application(|| 0, update, view)
        .subscription(|_state| iced::event::listen().map(Message::Event))
        .run()
}
