#[derive(Debug, Clone)]
struct MeaningOfLife(String);

fn calculate_meaning() -> MeaningOfLife {
    std::thread::sleep(std::time::Duration::from_secs(3));

    MeaningOfLife(String::from("the meaning of life is 42"))
}

async fn meaning_of_life() -> Option<MeaningOfLife> {
    let (result_tx, result_rx) = iced::futures::channel::oneshot::channel();

    std::thread::spawn(|| {
        let result = calculate_meaning();

        result_tx.send(result).expect("showing the meaning of life");
    });

    result_rx.await.ok()
}

#[derive(Debug, Clone)]
enum Message {
    FindTheMeaningOfLife,
    TheMeaningOfLife(Option<MeaningOfLife>),
}

#[derive(Default)]
enum State {
    #[default]
    Unknown,
    Searching,
    Found(MeaningOfLife),
    NotFound,
}

#[derive(Default)]
struct App {
    state: State,
}

impl App {
    pub fn view(&self) -> iced::Element<'_, Message> {
        let main: iced::Element<_> = match &self.state {
            State::Unknown => iced::widget::button("find the meaning of life")
                .on_press(Message::FindTheMeaningOfLife)
                .into(),
            State::Searching => "Searching...".into(),
            State::Found(MeaningOfLife(meaning)) => iced::widget::text(meaning).into(),
            State::NotFound => "could not find the meaning of life".into(),
        };

        iced::widget::center(main).into()
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::FindTheMeaningOfLife => {
                self.state = State::Searching;

                iced::Task::perform(meaning_of_life(), Message::TheMeaningOfLife)
            }

            Message::TheMeaningOfLife(meaning) => {
                self.state = match meaning {
                    Some(meaning) => State::Found(meaning),
                    None => State::NotFound,
                };

                iced::Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
