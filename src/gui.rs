use std::{
    iter::repeat_with,
    ops::{Index, IndexMut},
};

use iced::{
    widget::{self, image::Handle},
    Application, Command, Element, Length, Renderer,
};
use rgb::RGBA8;

use crate::{graph::graph, physics::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextField {
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

impl Index<TextField> for [String; 9] {
    type Output = String;

    fn index(&self, index: TextField) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<TextField> for [String; 9] {
    fn index_mut(&mut self, index: TextField) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(TextField, String),
}

pub struct Gui {
    image: Option<Vec<RGBA8>>,
    text_fields: [String; 9],
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
                text_fields: repeat_with(String::new)
                    .take(9)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Physics".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(field, s) => self.text_fields[field as usize] = s,
        };

        let parameters: Result<_, std::num::ParseFloatError> = (|| {
            Ok(Parameters::new(
                self.text_fields[TextField::CrossArea].parse()?,
                self.text_fields[TextField::FluidDensity].parse()?,
                self.text_fields[TextField::DragCoefficient].parse()?,
                self.text_fields[TextField::Mass].parse()?,
                0.001,
                MotionState {
                    position: Vec2 {
                        x: self.text_fields[TextField::InitialX].parse()?,
                        y: self.text_fields[TextField::InitialY].parse()?,
                    },
                    velocity: Vec2::from_magnitude_angle(
                        self.text_fields[TextField::InitialVelocity].parse()?,
                        self.text_fields[TextField::InitialAngle].parse()?,
                    ),
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    time: 0.0,
                },
                self.text_fields[TextField::EndingTime].parse()?,
            ))
        })();

        if let Ok(parameters) = parameters {
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
                self.text_fields[TextField::CrossArea].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::CrossArea, s))
            .into(),
            widget::text_input(
                "Fluid density",
                self.text_fields[TextField::FluidDensity].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::FluidDensity, s))
            .into(),
            widget::text_input(
                "Drag coefficient",
                self.text_fields[TextField::DragCoefficient].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::DragCoefficient, s))
            .into(),
            widget::text_input("Mass", self.text_fields[TextField::Mass].as_str())
                .on_input(|s| Message::TextChanged(TextField::Mass, s))
                .into(),
            widget::text_input(
                "Initial velocity",
                self.text_fields[TextField::InitialVelocity].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::InitialVelocity, s))
            .into(),
            widget::text_input(
                "Initial angle",
                self.text_fields[TextField::InitialAngle].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::InitialAngle, s))
            .into(),
            widget::text_input("Initial x", self.text_fields[TextField::InitialX].as_str())
                .on_input(|s| Message::TextChanged(TextField::InitialX, s))
                .into(),
            widget::text_input("Initial y", self.text_fields[TextField::InitialY].as_str())
                .on_input(|s| Message::TextChanged(TextField::InitialY, s))
                .into(),
            widget::text_input(
                "Ending time",
                self.text_fields[TextField::EndingTime].as_str(),
            )
            .on_input(|s| Message::TextChanged(TextField::EndingTime, s))
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
