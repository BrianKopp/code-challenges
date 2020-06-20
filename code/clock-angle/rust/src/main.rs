use std::env;

// Expect either a single command line argumnet like 5:12, which has two integers split by a colon.
// or expect two command line args which are two integers like 5 12.
fn main() {
    let args: Vec<String> = env::args().collect();
    let hour_string: &str;
    let minute_string: &str;
    if args.len() == 2 {
        let time_parts: Vec<&str> = args.get(1).unwrap().split(":").collect();
        hour_string = time_parts.get(0).expect("command line argument should be like 5:12");
        minute_string = time_parts.get(1).expect("command line argument should be like 5:12");
    } else if args.len() == 3 {
        hour_string = args.get(1).unwrap();
        minute_string = args.get(2).unwrap();
    } else {
        panic!("expected either one command line argument like '5:12' or two command line arguments like 'hour minute'");
    }

    let hour = hour_string.parse::<i32>().expect("expected command line argument hour position to be int");
    let minute = minute_string.parse::<i32>().expect("expected command line argument minute position to be int");
    println!("Command line args: {:?}", args);
    let angle = angle_between_clock_arms(hour, minute).expect("Invalid inputs");
    println!("angle between arms at {:?}:{:?} is {:?}", hour, minute, angle);
}

fn angle_between_clock_arms(hour_position: i32, minute_position: i32) -> Result<f64, &'static str> {
    if hour_position < 0 || hour_position > 12 {
        return Err("the hour hand cannot have a value outside of bounds (0, 12)");
    }
    if minute_position < 0 || minute_position > 60 {
        return Err("the minute hand cannot have value outside of bounds (0, 60)")
    }

    let total_angle = 360.;

    let hour_fraction = hour_position as f64 / 12.;
    let hour_angle = hour_fraction * total_angle;

    let minute_fraction = minute_position as f64 / 60.;
    let minute_angle = minute_fraction * total_angle;

    let mut difference = minute_angle - hour_angle;
    if difference < 0. {
        difference *= -1.;
    }

    Ok(difference)
}