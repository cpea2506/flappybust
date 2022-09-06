pub trait Math {
    fn half(self) -> f32;
}

impl Math for f32 {
    fn half(self) -> f32 {
        self / 2.
    }
}
