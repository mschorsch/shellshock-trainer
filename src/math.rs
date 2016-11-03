
// Resources
// https://en.wikipedia.org/wiki/Trajectory_of_a_projectile
// http://gamedev.stackexchange.com/questions/17467/calculating-velocity-needed-to-hit-target-in-parabolic-arc

use ::platform::{Rect, Cursor};

use std::cmp::Ordering;

const BASE_WINDOW_RESOLUTION: (u32, u32) = (1768, 992);
const BASE_METER_2_PIXEL: f64 = 2.271f64; // m -> px; magic constant; found manually

const EARTH_GRAVITY: f64 = 9.81; // m/s^2
const GRAVITY: f64 = EARTH_GRAVITY * BASE_METER_2_PIXEL; // m/s^2 -> px/2^2

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

    pub fn get_velocity(&self) -> u32 {
        self.velocity
    }

    pub fn get_angle(&self) -> i32 {
        self.angle
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

        let v_px = (v as f64) * BASE_METER_2_PIXEL; // velocity in px/s
        let s = (v_px * v_px * v_px * v_px) - GRAVITY * (GRAVITY * (x * x) + 2f64 * y * (v_px * v_px)); // substitution
        let o = (((v_px * v_px) + s.sqrt()) / (GRAVITY * x)).atan(); // launch angle in radians

        if o.is_nan() {
            continue;
        }

        angles.push((v, o.to_degrees()));
    }

    angles.sort_by(fraction_comparator); // sort; best first
    angles.iter().map(|val| Hit::new(val.0, val.1.round() as i32)).collect()
}

fn fraction_comparator(a: &(u32, f64), b: &(u32, f64)) -> Ordering {
    let frac_a = a.1 - (a.1 as i32) as f64;
    let frac_b = b.1 - (b.1 as i32) as f64;

    let frac_a = if frac_a > 0.5 { 1.0 - frac_a } else { frac_a };
    let frac_b = if frac_b > 0.5 { 1.0 - frac_b } else { frac_b };

    if frac_a > frac_b {
        Ordering::Greater
    } else if frac_a < frac_b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}