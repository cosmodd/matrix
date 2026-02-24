const PI: f64 = 3.14159265358979323846264338327950288;

pub trait Trig {
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
    fn to_radians(self) -> Self;
    fn to_degrees(self) -> Self;
}

macro_rules! impl_trig_float {
    ($($t:ty),*) => {
        $(
        impl Trig for $t {
            fn cos(self) -> Self { self.cos() }
            fn sin(self) -> Self { self.sin() }
            fn tan(self) -> Self { self.tan() }
            fn to_radians(self) -> Self {
                self * ((PI / 180.0) as $t)
            }
            fn to_degrees(self) -> Self {
                self * ((180. / PI) as $t)
            }
        }
        )*
    }
}

impl_trig_float!(f32, f64);