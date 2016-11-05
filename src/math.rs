
// Resources
// https://en.wikipedia.org/wiki/Trajectory_of_a_projectile
// http://gamedev.stackexchange.com/questions/17467/calculating-velocity-needed-to-hit-target-in-parabolic-arc

use ::platform::{Rect, Cursor};

use std::cmp::Ordering;
use std::fmt;

const BASE_WINDOW_RESOLUTION: (u32, u32) = (1768, 992);
const BASE_METER_2_PIXEL: f64 = 2.271f64; // m -> px; magic constant; found manually
const GRAVITY: f64 = 9.81; // m/s^2; earth gravity (constant)

#[derive(Debug)]
pub struct Hit {
    velocity: u32,
    angle: i32,
}

impl Hit {
    fn new(velocity: u32, angle: i32) -> Self {
        Hit {
            velocity: velocity,
            angle: angle,
        }
    }

    #[allow(dead_code)]
    pub fn get_velocity(&self) -> u32 {
        self.velocity
    }

    pub fn get_angle(&self) -> i32 {
        self.angle
    }
}

impl fmt::Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.velocity, self.angle)
    }
}

// translates the target position relativ to 'from'
pub fn translate_target_position_relativ_to_origin(rect: &Rect,
                                                   from: &Cursor,
                                                   to: &Cursor)
                                                   -> (f64, f64) {
    let from_scaled = scale_position(rect, from);
    let to_scaled = scale_position(rect, to);

    // calc target (x,y) position relativ to 'from'
    let x = (from_scaled.0 - to_scaled.0).abs();
    let y = to_scaled.1 - from_scaled.1;

    (x, y)
}

fn scale_position(rect: &Rect, cursor: &Cursor) -> (f64, f64) {
    // window size
    let window_width = rect.get_width();
    let window_height = rect.get_height();

    // scale factors / scale to base resolution
    let scalex = BASE_WINDOW_RESOLUTION.0 as f64 / window_width as f64;
    let scaley = BASE_WINDOW_RESOLUTION.1 as f64 / window_height as f64;

    // cursor position
    let cx = cursor.get_x() as f64 * scalex;
    let cy = (window_height - cursor.get_y()) as f64 * scaley; // translate (0,0) from upperleft to bottomleft

    (cx, cy)
}

// calculates launch angles for (x, y) target
pub fn calc_launch_angles(x: f64, y: f64) -> Vec<Hit> {
    let mut angles = Vec::new();
    for v in 1..101 /* velocity in m/s */ {
        if let Some(o) = calc_launch_angle(v, x, y) {
            angles.push((v as f64, o));
        }
    }

    // sort; best first
    angles.sort_by(|a: &(f64, f64), b: &(f64, f64)| {
        let frac_a = get_fraction(a.1);
        let frac_b = get_fraction(b.1);
        order_by(frac_a, frac_b)
    });
    angles.iter().map(|val| Hit::new(val.0 as u32, val.1.round() as i32)).collect()
}

fn calc_launch_angle(v: u32, x: f64, y: f64) -> Option<f64> {
    let v = v as f64;
    let x = x / BASE_METER_2_PIXEL;
    let y = y / BASE_METER_2_PIXEL;

    let s = (v * v * v * v) - GRAVITY * (GRAVITY * (x * x) + 2f64 * y * (v * v)); // substitution
    let o = (((v * v) + s.sqrt()) / (GRAVITY * x)).atan(); // launch angle in radians

    if o.is_nan() {
        Option::None
    } else {
        Option::Some(o.to_degrees())
    }
}

pub fn calc_launch_velocities(x: f64, y: f64) -> Vec<Hit> {
    let mut velocities = Vec::new();
    for o in -90..91 /* angle in degrees */ {
        if let Some(v) = calc_launch_velocity(o, x, y) {
            velocities.push((v, o as f64));
        }
    }

    // sort; best first
    velocities.sort_by(|a: &(f64, f64), b: &(f64, f64)| {
        let frac_a = get_fraction(a.0);
        let frac_b = get_fraction(b.0);
        order_by(frac_a, frac_b)
    });
    velocities.iter().map(|val| Hit::new(val.0 as u32, val.1.round() as i32)).collect()
}

fn calc_launch_velocity(o: i32, x: f64, y: f64) -> Option<f64> {
    let x = x / BASE_METER_2_PIXEL;
    let y = y / BASE_METER_2_PIXEL;
    let o_rad_tan = (o as f64).to_radians().tan();

    // calculated from https://en.wikipedia.org/wiki/Trajectory_of_a_projectile
    let a = -GRAVITY * x * x * (1f64 + o_rad_tan * o_rad_tan);
    let b = 2f64 * (y - x * o_rad_tan);
    let v = (a / b).sqrt();

    if v.is_nan() || v > 100.0 {
        Option::None
    } else {
        Option::Some(v)
    }
}

fn order_by(a: f64, b: f64) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn get_fraction(val: f64) -> f64 {
    let frac = (val - (val as i64) as f64).abs();
    if frac > 0.5 { 1.0 - frac } else { frac }
}
