const GRAVITY: f64 = 9.81;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn from_magnitude_angle(magnitude: f64, angle: f64) -> Vec2 {
        Vec2 {
            x: magnitude * angle.to_radians().cos(),
            y: magnitude * angle.to_radians().sin(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x).to_degrees()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MotionState {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub time: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub cross_area: f64,
    pub fluid_density: f64,
    pub drag_coefficient: f64,
    pub mass: f64,
    pub delta_time: f64,
    pub initial_conditions: MotionState,
    pub ending_time: f64,
    pub drag_proportion: f64,
}

impl Parameters {
    pub fn new(
        cross_area: f64,
        fluid_density: f64,
        drag_coefficient: f64,
        mass: f64,
        delta_time: f64,
        initial_conditions: MotionState,
        ending_time: f64,
    ) -> Parameters {
        let drag_proportion = cross_area * fluid_density * drag_coefficient / 2.0;
        Parameters {
            cross_area,
            fluid_density,
            drag_coefficient,
            mass,
            delta_time,
            initial_conditions,
            ending_time,
            drag_proportion,
        }
    }
}

pub fn simulate_motion(parameters: Parameters) -> Vec<MotionState> {
    let mut state = parameters.initial_conditions;
    let mut states = vec![state];
    while state.time < parameters.ending_time {
        state.acceleration = calculate_acceleration(&parameters, &state);
        state.velocity = calculate_velocity(&parameters, &state);
        state.position = calculate_position(&parameters, &state);
        state.time += parameters.delta_time;
        states.push(state);
        if state.position.y < 0.0 {
            break;
        }
    }
    states
}

fn calculate_acceleration(parameters: &Parameters, state: &MotionState) -> Vec2 {
    Vec2 {
        x: -(parameters.drag_proportion / parameters.mass)
            * state.velocity.magnitude()
            * state.velocity.x,
        y: -GRAVITY
            - (parameters.drag_proportion / parameters.mass)
                * state.velocity.magnitude()
                * state.velocity.y,
    }
}

fn calculate_velocity(parameters: &Parameters, state: &MotionState) -> Vec2 {
    Vec2 {
        x: state.velocity.x + state.acceleration.x * parameters.delta_time,
        y: state.velocity.y + state.acceleration.y * parameters.delta_time,
    }
}

fn calculate_position(parameters: &Parameters, state: &MotionState) -> Vec2 {
    Vec2 {
        x: state.position.x
            + state.velocity.x * parameters.delta_time
            + 0.5 * state.acceleration.x * parameters.delta_time.powi(2),
        y: state.position.y
            + state.velocity.y * parameters.delta_time
            + 0.5 * state.acceleration.y * parameters.delta_time.powi(2),
    }
}
