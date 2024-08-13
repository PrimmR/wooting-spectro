use easyfft::FftNum;

macro_rules! default_sample_val_impl(
    ( $( $x:ty ),+ ) => {
        $(
            impl SampleVal for $x {
                fn to_f32(&self) -> f32 {
                    (*self).into()
                }
            }
        )*
    };
);

/// A value that can represent a sample in a waveform
pub trait SampleVal: FftNum + NumberBits {
    fn to_f32(&self) -> f32;

    fn norm_to_f32(&self) -> f32 {
        self.to_f32() / <Self as NumberBits>::MAX
    }
}

default_sample_val_impl!(i8, i16);

impl SampleVal for i32 {
    fn to_f32(&self) -> f32 {
        *self as f32
    }
}

impl SampleVal for f32 {
    fn to_f32(&self) -> f32 {
        *self
    }

    fn norm_to_f32(&self) -> f32 {
        *self
    }
}

macro_rules! default_number_bits_impl(
    ( $( $x:ty ),+ ) => {
        $(
            impl NumberBits for $x {
                const MAX: f32 = <$x>::MAX as f32;
            }
        )*
    };
);

/// A trait that gives access to the MAX associated const that belongs to most numbers
pub trait NumberBits {
    const MAX: f32;
}

default_number_bits_impl!(f32, i8, i16, i32);
