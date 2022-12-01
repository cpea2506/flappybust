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

/// This is to replace the cumbersome original short-hand if-else
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $if:expr, $else:expr) => {
        if $condition {
            $if
        } else {
            $else
        }
    };
}
