use enigo::{Coordinate, Enigo, Mouse, Settings};
use gilrs::{Axis, Event, Gilrs};
use gilrs::EventType::AxisChanged;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut mouse_x_amt = 0;
    let mut mouse_y_amt = 0;

    loop {
        // Examine new events
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);

            if let AxisChanged(axis, amt, _) = event {
                if axis == Axis::LeftStickX {
                    mouse_x_amt = (amt * 2.0) as i32;

                } else if axis == Axis::LeftStickY {
                    mouse_y_amt = -(amt * 2.0) as i32

                }
            }
        }

        enigo.move_mouse(mouse_x_amt, 0, Coordinate::Rel).unwrap();
        enigo.move_mouse(0, mouse_y_amt, Coordinate::Rel).unwrap();
    }
}
