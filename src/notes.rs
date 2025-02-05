use strum_macros::EnumIter;

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Note {
    G3 = 196,
    G3Sharp = 207,
    A3 = 220,
    A3Sharp = 233,
    B3 = 247,
    C4 = 262,
    C4Sharp = 277,
    D4 = 294,
    D4Sharp = 311,
    E4 = 330,
    F4 = 349,
    F4Sharp = 370,
    G4 = 392,
    G4Sharp = 415,
    A4 = 440,
    A4Sharp = 466,
    B4 = 494,
    C5 = 523,
    C5Sharp = 554,
    D5 = 587,
    D5Sharp = 622,
    E5 = 659,
    F5 = 698,
    F5Sharp = 740,
    G5 = 784,
    G5Sharp = 831,
    A5 = 880,
    A5Sharp = 932,
    B5 = 988,
    C6 = 1047,
    C6Sharp = 1109,
    D6 = 1175,
    D6Sharp = 1245,
    E6 = 1319,
    F6 = 1397,
    F6Sharp = 1480,
    G6 = 1568,
    G6Sharp = 1661,
    A6 = 1760,
    A6Sharp = 1865,
    B6 = 1976,
    C7 = 2093,
    C7Sharp = 2217,
    D7 = 2349,
    D7Sharp = 2489,
    E7 = 2637,
}

impl Note {
    pub fn frequency(&self) -> f32 {
        *self as i32 as f32
    }
}