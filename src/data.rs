use atm_refraction::Environment;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Altitude {
    Absolute(f64),
    Relative(f64),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Altitude,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Frame {
    pub direction: f64,
    pub tilt: f64,
    pub fov: f64,
    pub max_distance: f64,
    pub water_level: f64,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct View {
    pub position: Position,
    pub frame: Frame,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Tick {
    Single {
        azimuth: f64,
        size: u32,
        labelled: bool,
    },
    Multiple {
        bias: f64,
        step: f64,
        size: u32,
        labelled: bool,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Output {
    pub file: String,
    pub file_metadata: Option<String>,
    pub width: u16,
    pub height: u16,
    pub ticks: Vec<Tick>,
    pub show_eye_level: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
    pub terrain_folder: String,
    pub view: View,
    pub env: Environment,
    pub straight_rays: bool,
    pub simulation_step: f64,
    pub output: Output,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResultPixel {
    pub lat: f64,
    pub lon: f64,
    pub distance: f64,
    pub elevation: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllData {
    pub params: Params,
    pub result: Vec<Vec<Option<ResultPixel>>>,
}
