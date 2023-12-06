use iced::{
    widget::{self, image::Handle, row},
    Application, Color, Command, Element, Length, Renderer,
};
use rgb::RGBA8;

use crate::physics::Parameters;

#[derive(Debug)]
pub enum Message {}

pub struct Gui {
    image: Option<Vec<RGBA8>>,
    parameters: Parameters,
}

impl Application for Gui {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                image: None,
                parameters: Parameters::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Physics".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mut row = widget::row(vec![]);

        if let Some(image) = &self.image {
            let image_buffer_flattened: Vec<u8> = self
                .image
                .as_ref()
                .unwrap()
                .iter()
                .cloned()
                .flat_map(|rgba| Into::<[u8; 4]>::into(rgba))
                .collect();
            let handle = Handle::from_pixels(800, 600, image_buffer_flattened);
            let element: Element<_> = widget::image(handle)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
            row = row.push(element);
        }

        row.into()
    }
}
