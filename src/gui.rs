use crate::{graph::graph, physics::*};
use iced::{
    widget::{self, image::Handle, row},
    Alignment, Application, Command, Element, Length, Renderer,
};
use rgb::RGBA8;
use std::{
    iter::repeat_with,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy)]
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
    DeltaTimeChanged(i32),
}

pub struct Gui {
    simulations: Option<(Vec<MotionState>, Vec<MotionState>)>,
    text_fields: [String; 9],
    delta_time_scale: i32,
}

impl Application for Gui {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                simulations: None,
                text_fields: repeat_with(String::new)
                    .take(9)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                delta_time_scale: 3,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Drag Simulation".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(field, s) => self.text_fields[field as usize] = s,
            Message::DeltaTimeChanged(scale) => self.delta_time_scale = scale,
        };

        let parameters: Result<_, std::num::ParseFloatError> = (|| {
            Ok(Parameters::new(
                self.text_fields[TextField::CrossArea].parse()?,
                self.text_fields[TextField::FluidDensity].parse()?,
                self.text_fields[TextField::DragCoefficient].parse()?,
                self.text_fields[TextField::Mass].parse()?,
                10f64.powi(-self.delta_time_scale),
                MotionState {
                    position: Vec2::new(
                        self.text_fields[TextField::InitialX].parse()?,
                        self.text_fields[TextField::InitialY].parse()?,
                    ),
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
                drag_proportion: 0.0,
                ..parameters
            });
            self.simulations = Some((motion, motion_no_drag));
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let inputs: Vec<Element<_>> = vec![
            self.make_input("Cross-sectional area", TextField::CrossArea),
            self.make_input("Fluid density", TextField::FluidDensity),
            self.make_input("Drag coefficient", TextField::DragCoefficient),
            self.make_input("Mass", TextField::Mass),
            self.make_input("Initial velocity", TextField::InitialVelocity),
            self.make_input("Initial angle", TextField::InitialAngle),
            self.make_input("Initial x", TextField::InitialX),
            self.make_input("Initial y", TextField::InitialY),
            self.make_input("Ending time", TextField::EndingTime),
            row![
                widget::text(format!(
                    "Time scale ({}):",
                    10f64.powi(-self.delta_time_scale)
                ))
                .width(Length::FillPortion(1)),
                widget::slider(0..=5, self.delta_time_scale, Message::DeltaTimeChanged)
                    .width(Length::FillPortion(2))
            ]
            .into(),
        ];
        let inputs = widget::column(inputs)
            .width(Length::FillPortion(1))
            .spacing(2)
            .padding(10)
            .into();

        let image = widget::container(widget::responsive(|size| {
            let size = (size.width as u32, size.height as u32);
            match self.generate_image(size) {
                Some(image) => {
                    let image_buffer_flattened: Vec<u8> = image
                        .iter()
                        .cloned()
                        .flat_map(|rgba| Into::<[u8; 4]>::into(rgba))
                        .collect();
                    let handle = Handle::from_pixels(size.0, size.1, image_buffer_flattened);
                    widget::image(handle).into()
                }
                None => widget::horizontal_space(Length::Fill).into(),
            }
        }))
        .width(Length::FillPortion(2))
        .into();

        widget::row(vec![inputs, image]).into()
    }
}

impl Gui {
    fn make_input(
        &self,
        label: &str,
        field: TextField,
    ) -> Element<'_, Message, Renderer<iced::Theme>> {
        row![
            widget::text(format!("{}:", label)).width(Length::FillPortion(1)),
            widget::text_input(label, &self.text_fields[field])
                .on_input(move |s| Message::TextChanged(field, s))
                .width(Length::FillPortion(2))
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn generate_image(&self, size: (u32, u32)) -> Option<Vec<RGBA8>> {
        if let Some(ref simulations) = self.simulations {
            Some(graph(&simulations.0, &simulations.1, (size.0, size.1)))
        } else {
            None
        }
    }
}
