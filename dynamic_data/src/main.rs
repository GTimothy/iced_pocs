use dynamic_data::Message;
use iced_graphics::Renderer;
use iced_native::Element;
use iced_wgpu::Backend;

use iced::{button, Application, Button, Column, Settings};

struct DummyEntryLoader();

struct Entry(isize);

impl dynamic_data::DataLoader<Entry> for DummyEntryLoader {
    fn get_next_n(&self, n: isize, from: isize) -> Vec<Entry> {
        (from..from + n).map(|i| Entry(i)).collect()
    }

    fn get_previous_n(&self, n: isize, from: isize) -> Vec<Entry> {
        (from - n..from).map(|i| Entry(i)).collect()
    }
}

pub fn main() -> iced::Result {
    Model::run(Settings::default())
}

struct Model {
    dynamic_data: dynamic_data::DynamicData<Entry, DummyEntryLoader>,
    next_button: button::State,
    previous_button: button::State,
    scrollable_state: iced::scrollable::State,
}

impl iced::Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from("DynamicData - Iced")
    }

    fn new() -> Self {
        Model {
            dynamic_data: dynamic_data::DynamicData::new(DummyEntryLoader {}).capacity(100),
            scrollable_state: iced::scrollable::State::new(),
            previous_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Previous(n) => self.dynamic_data.fetch_previous(n),
            Message::Next(n) => self.dynamic_data.fetch_next(n),
        }
    }

    fn view(&mut self) -> Element<Message, Renderer<Backend>> {

        // This is where you choose how you want to display your data. Here it is a column with
        // text widgets, but it could be a canvas, a row, one of your own widgets, anything you
        // want.
        let data_column: Column<Message> =
            self.dynamic_data.data().into_iter().fold(Column::new(), |col, i| {
                col.push(iced::Text::new(i.0.to_string()))
            });

        let previous_button =
            Button::new(&mut self.previous_button, iced::Text::new("Load previous"))
                .on_press(Message::Previous(20));

        let next_button = Button::new(&mut self.next_button, iced::Text::new("Load next"))
            .on_press(Message::Next(40));

        iced::Scrollable::new(&mut self.scrollable_state)
            .push(
                Column::new()
                    .push(previous_button)
                    .push(data_column)
                    .push(next_button),
            )
            .height(iced::Length::Fill)
            .into()
    }
}
