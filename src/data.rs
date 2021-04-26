use atm_refraction::Environment;
use nalgebra::Vector3;

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
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Coloring {
    Simple {
        water_level: f64,
        max_distance: f64,
    },
    Shading {
        water_level: f64,
        ambient_light: f64,
        light_dir: Vector3<f64>,
    },
}

impl Coloring {
    pub fn water_level(&self) -> f64 {
        match *self {
            Coloring::Simple { water_level, .. } | Coloring::Shading { water_level, .. } => {
                water_level
            }
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct View {
    pub position: Position,
    pub frame: Frame,
    pub coloring: Coloring,
    pub fog_distance: Option<f64>,
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

impl Params {
    pub fn get_azim_and_elev(&self, x: usize, y: usize) -> (f64, f64) {
        let width = self.output.width as f64;
        let height = self.output.height as f64;
        let x = x as f64 - width / 2.0;
        let y = height / 2.0 - y as f64;
        let fov = self.view.frame.fov;
        let mut azim = self.view.frame.direction + x * fov / width;
        if azim < 0.0 {
            azim += 360.0;
        }
        if azim >= 360.0 {
            azim -= 360.0;
        }
        let elev = self.view.frame.tilt + y * fov / width;
        (azim, elev)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResultPixel {
    pub lat: f64,
    pub lon: f64,
    pub distance: f64,
    pub elevation: f64,
    pub path_length: f64,
    pub normal: Vector3<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllData {
    pub params: Params,
    pub result: Vec<Vec<Option<ResultPixel>>>,
}
