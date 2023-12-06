use iced::{
    widget::{self, image::Handle, row},
    Application, Color, Command, Element, Length, Renderer,
};
use rgb::RGBA8;

#[derive(Debug)]
pub enum Message {}

pub struct Gui {
    image: Vec<RGBA8>,
}

impl Application for Gui {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Vec<RGBA8>;

    fn new(image: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { image }, Command::none())
    }

    fn title(&self) -> String {
        "Physics".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let image_buffer_flattened: Vec<u8> = self
            .image
            .iter()
            .flat_map(|rgba| [rgba.r, rgba.g, rgba.b, rgba.a])
            .collect();
        let handle = Handle::from_pixels(800, 600, image_buffer_flattened);
        let element: Element<_> = widget::image(handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        row![element.explain(Color::BLACK)].into()
    }
}
