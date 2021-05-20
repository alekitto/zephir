use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u32 {
        const None = 0b00000000;
        const ForAllValues = 0b00000001;
        const ForAnyValue = 0b00000010;
        const IfExists = 0b00000100;
    }
}
