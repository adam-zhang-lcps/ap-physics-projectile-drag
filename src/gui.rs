use std::collections::HashMap;

use iced::{
    widget::{self, image::Handle, row},
    Application, Color, Command, Element, Length, Renderer,
};
use rgb::RGBA8;

use crate::{graph::graph, physics::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFields {
    CrossArea,
    FluidDensity,
    DragCoefficient,
    Mass,
    InitialVelocity,
    InitialAngle,
    InitialX,
    InitialY,
    EndingTime,
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(TextFields, String),
}

pub struct Gui {
    image: Option<Vec<RGBA8>>,
    text_fields: HashMap<TextFields, String>,
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
                text_fields: HashMap::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Physics".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(field, s) => *self.text_fields.entry(field).or_insert(String::new()) = s,
        };

        let parameters = (|| {
            Some(Parameters::new(
                self.text_fields.get(&TextFields::CrossArea)?.parse().ok()?,
                self.text_fields
                    .get(&TextFields::FluidDensity)?
                    .parse()
                    .ok()?,
                self.text_fields
                    .get(&TextFields::DragCoefficient)?
                    .parse()
                    .ok()?,
                self.text_fields.get(&TextFields::Mass)?.parse().ok()?,
                0.001,
                MotionState {
                    position: Vec2 {
                        x: self.text_fields.get(&TextFields::InitialX)?.parse().ok()?,
                        y: self.text_fields.get(&TextFields::InitialY)?.parse().ok()?,
                    },
                    velocity: Vec2::from_magnitude_angle(
                        self.text_fields
                            .get(&TextFields::InitialVelocity)?
                            .parse()
                            .ok()?,
                        self.text_fields
                            .get(&TextFields::InitialAngle)?
                            .parse()
                            .ok()?,
                    ),
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    time: 0.0,
                },
                self.text_fields
                    .get(&TextFields::EndingTime)?
                    .parse()
                    .ok()?,
            ))
        })();

        if let Some(parameters) = parameters {
            let motion = simulate_motion(parameters);
            let motion_no_drag = simulate_motion(Parameters {
                drag_coefficient: 0.0,
                ..parameters
            });
            self.image = Some(graph(motion, motion_no_drag));
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let inputs: Vec<Element<_>> = vec![
            widget::text_input(
                "Cross-sectional area",
                self.text_fields
                    .get(&TextFields::CrossArea)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::CrossArea, s))
            .into(),
            widget::text_input(
                "Fluid density",
                self.text_fields
                    .get(&TextFields::FluidDensity)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::FluidDensity, s))
            .into(),
            widget::text_input(
                "Drag coefficient",
                self.text_fields
                    .get(&TextFields::DragCoefficient)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::DragCoefficient, s))
            .into(),
            widget::text_input(
                "Mass",
                self.text_fields
                    .get(&TextFields::Mass)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::Mass, s))
            .into(),
            widget::text_input(
                "Initial velocity",
                self.text_fields
                    .get(&TextFields::InitialVelocity)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::InitialVelocity, s))
            .into(),
            widget::text_input(
                "Initial angle",
                self.text_fields
                    .get(&TextFields::InitialAngle)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::InitialAngle, s))
            .into(),
            widget::text_input(
                "Initial x",
                self.text_fields
                    .get(&TextFields::InitialX)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::InitialX, s))
            .into(),
            widget::text_input(
                "Initial y",
                self.text_fields
                    .get(&TextFields::InitialY)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::InitialY, s))
            .into(),
            widget::text_input(
                "Ending time",
                self.text_fields
                    .get(&TextFields::EndingTime)
                    .map_or("", String::as_str),
            )
            .on_input(|s| Message::TextChanged(TextFields::EndingTime, s))
            .into(),
        ];
        let inputs = widget::column(inputs).width(Length::FillPortion(1)).into();

        let image = match self.image {
            Some(ref image) => {
                let image_buffer_flattened: Vec<u8> = image
                    .iter()
                    .cloned()
                    .flat_map(|rgba| Into::<[u8; 4]>::into(rgba))
                    .collect();
                let handle = Handle::from_pixels(800, 600, image_buffer_flattened);
                widget::image(handle).width(Length::FillPortion(2)).into()
            }
            None => widget::horizontal_space(Length::FillPortion(2)).into(),
        };

        widget::row(vec![inputs, image]).into()
    }
}
