#[derive(Clone)]
enum Message {
    CalculatedInformation(i32),
    StartCalculatingInformation,
}

#[derive(Default)]
struct App {
    hard_to_process_information: Option<i32>,
    calculation_in_progress: bool,
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::CalculatedInformation(information) => {
                self.hard_to_process_information = Some(information);
                self.calculation_in_progress = false;

                iced::Task::none()
            }
            Message::StartCalculatingInformation => {
                self.calculation_in_progress = true;
                iced::Task::future(async {
                    let information = tokio::task::spawn_blocking(|| {
                        println!("Calculating information...");

                        std::thread::sleep(std::time::Duration::from_secs(2));

                        42
                    })
                    .await
                    .unwrap();

                    Message::CalculatedInformation(information)
                })
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::column![
            iced::widget::Text::new(self.hard_to_process_information.map_or(
                "information will be ready in a second...".to_string(),
                |x| format!("information: {x}"),
            )),
            iced::widget::button("start calculation").on_press_maybe(
                if self.calculation_in_progress {
                    None
                } else {
                    Some(Message::StartCalculatingInformation)
                }
            )
        ]
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
