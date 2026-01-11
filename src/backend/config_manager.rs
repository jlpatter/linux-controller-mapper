use std::collections::HashMap;
use enigo::Key;
use gilrs::{Axis, Button};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LayoutConfig {
    controller_name: String,
    controller_uuid: Uuid,
    button_map: HashMap<Button, Key>,
    axis_map: HashMap<Axis, String>,
}
