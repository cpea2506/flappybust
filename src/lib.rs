pub trait BooleanSwitcher {
    fn on(&mut self);
    fn off(&mut self);
}

impl BooleanSwitcher for bool {
    fn off(&mut self) {
        *self = false
    }

    fn on(&mut self) {
        *self = true
    }
}

pub trait Math {
    fn half(self) -> f32;
}

impl Math for f32 {
    fn half(self) -> f32 {
        self / 2.
    }
}
