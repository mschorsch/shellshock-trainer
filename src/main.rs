
//  
// A simple (non intrusive) trainer for http://www.shellshocklive.com/
// 

mod platform;
mod math;

use platform::{Handle, VK};

use std::thread;
use std::time;

const SHOW_MAX_HITS: usize = 8;

fn main() {
    println!("[INFO] Searching ...");
    let handle = if cfg!(target_os = "windows") {
        platform::windows::find_shellshock_handle()
    } else {
        unimplemented!() // TODO implement linux, macos
    };

    println!("[INFO] ShellShock found. Waiting for input ...");
    start_event_loop(handle);
}

fn start_event_loop<H: Handle>(handle: H) {
    let mut source = Option::None;
    let mut target = Option::None;

    let mut vk1_state = false;
    let mut vk2_state = false;
    let mut vk3_state = false;
    let mut vk4_state = false;

    loop {
        thread::sleep(time::Duration::from_millis(10)); // sleep duration in milliseconds

        let vk1_key_down = handle.is_key_pressed(VK::Key1);
        let vk2_key_down = handle.is_key_pressed(VK::Key2);
        let vk3_key_down = handle.is_key_pressed(VK::Key3);
        let vk4_key_down = handle.is_key_pressed(VK::Key4);

        // Set position 1
        if vk1_key_down && !vk1_state {
            vk1_state = true;

            let position = handle.get_mouse_position_in_window();
            println!("[INFO] Position 1 set.");
            source = Option::Some(position);
        } else if !vk1_key_down {
            vk1_state = false
        }

        // Set position 2
        if vk2_key_down && !vk2_state {
            vk2_state = true;

            let position = handle.get_mouse_position_in_window();
            println!("[INFO] Position 2 set.");
            target = Option::Some(position);
        } else if !vk2_key_down {
            vk2_state = false
        }

        // Calculate hits
        if vk3_key_down && !vk3_state {
            vk3_state = true;

            if source.is_some() && target.is_some() {
                let rect = handle.get_window_rect();
                let from = source.as_ref().unwrap();
                let to = target.as_ref().unwrap();

                let target_pos = math::translate_target_position_relativ_to_origin(&rect, from, to);
                let hits: Vec<math::Hit> = math::calc_launch_angles(target_pos.0, target_pos.1);
                print_hits(hits);
            }
        } else if !vk3_key_down {
            vk3_state = false
        }

        // Clear positions
        if vk4_key_down && !vk4_state {
            vk4_state = true;

            source = Option::None;
            target = Option::None;
            println!("[INFO] Positions cleared.");
        } else if !vk4_key_down {
            vk4_state = false
        }
    }
}

fn print_hits(hits: Vec<math::Hit>) {
    println!("------- RESULTS -------");
    for (i, hit) in hits.iter().take(SHOW_MAX_HITS).enumerate() {
        println!("{}. ({},{})", (i + 1), hit.get_velocity(), hit.get_angle());
    }
    println!("-----------------------");
    println!("");
}