#[derive(Default)]
struct App {
    ip: String,
}

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentIp(String),
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        println!("update");

        match message {
            Message::Refetch => iced::Task::perform(fetch_ip(), Message::CurrentIp),
            Message::CurrentIp(text) => {
                self.ip = text;
                iced::Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::column![
            self.ip.as_str(),
            iced::widget::button("Start task").on_press(Message::Refetch)
        ]
        .into()
    }
}

async fn fetch_ip() -> String {
    println!("fetch_ip");

    reqwest::get("https://api.seeip.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
