use core::ops;

use crate::{vec2_t, vec3_t, vec4_t};

const PITCH: usize = crate::PITCH as usize;
const YAW: usize = crate::YAW as usize;
const ROLL: usize = crate::ROLL as usize;

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn sqrtf(f: f32) -> f32 {
    libm::sqrtf(f)
}

#[cfg(feature = "std")]
fn sqrtf(f: f32) -> f32 {
    f.sqrt()
}

macro_rules! impl_vec_ops {
    ($vec:ty[$n:expr], $op:ident::$m:ident, $op_assign:ident::$m_assign:ident) => {
        impl ops::$op for $vec {
            type Output = $vec;

            fn $m(mut self, rhs: $vec) -> Self::Output {
                ops::$op_assign::$m_assign(&mut self, rhs);
                self
            }
        }

        impl ops::$op<f32> for $vec {
            type Output = $vec;

            fn $m(mut self, rhs: f32) -> Self::Output {
                ops::$op_assign::$m_assign(&mut self, rhs);
                self
            }
        }

        impl ops::$op_assign for $vec {
            fn $m_assign(&mut self, rhs: $vec) {
                for i in 0..$n {
                    ops::$op_assign::$m_assign(&mut self[i], rhs[i]);
                }
            }
        }

        impl ops::$op_assign<f32> for $vec {
            fn $m_assign(&mut self, rhs: f32) {
                for i in 0..$n {
                    ops::$op_assign::$m_assign(&mut self[i], rhs);
                }
            }
        }
    };
}

macro_rules! impl_vec {
    ($vec:ty[$n:expr] { $($f:ident[$i:expr]($set:ident)),+ $(,)? }) => {
        impl $vec {
            pub const ZERO: $vec = Self::splat(0.0);

            pub const fn new($($f: f32),+) -> $vec {
                Self([$($f),+])
            }

            pub const fn splat(value: f32) -> $vec {
                Self([value; $n])
            }

            $(pub const fn $f(&self) -> f32 {
                self.0[$i]
            }

            pub fn $set(&mut self, $f: f32) {
                self.0[$i] = $f;
            })+

            pub const fn sum(&self) -> f32 {
                let mut acc = 0.0;
                $(acc += self.0[$i];)*
                acc
            }

            const fn mul(self, other: $vec) -> $vec {
                $(let $f = self.0[$i] * other.0[$i];)+
                Self([$($f),+])
            }

            pub const fn dot_product(self, other: $vec) -> f32 {
                (self.mul(other)).sum()
            }

            pub const fn average(self, other: $vec) -> $vec {
                $(let $f = (self.0[$i] + other.0[$i]) * 0.5;)+
                Self([$($f),+])
            }

            pub const fn to_radians(self) -> $vec {
                $(let $f = self.0[$i].to_radians();)+
                Self([$($f),+])
            }

            #[cfg(any(feature = "libm", feature = "std"))]
            pub fn distance(self, other: $vec) -> f32 {
                sqrtf(self.dot_product(other))
            }

            #[cfg(any(feature = "libm", feature = "std"))]
            pub fn length(self) -> f32 {
                self.distance(self)
            }

            #[cfg(any(feature = "libm", feature = "std"))]
            pub fn normalize_length(self) -> ($vec, f32) {
                let len = self.length();
                if len != 0.0 {
                    (self * (1.0 / len), len)
                } else {
                    (self, len)
                }
            }

            #[cfg(any(feature = "libm", feature = "std"))]
            pub fn normalize(self) -> $vec {
                self.normalize_length().0
            }
        }

        impl Default for $vec {
            fn default() -> Self {
                Self([0.0; $n])
            }
        }

        impl From<[f32; $n]> for $vec {
            fn from(arr: [f32; $n]) -> $vec {
                Self(arr)
            }
        }

        impl From<$vec> for [f32; $n] {
            fn from(vec: $vec) -> [f32; $n] {
                vec.0
            }
        }

        impl AsRef<[f32; $n]> for $vec {
            fn as_ref(&self) -> &[f32; $n] {
                &self.0
            }
        }

        impl AsMut<[f32; $n]> for $vec {
            fn as_mut(&mut self) -> &mut [f32; $n] {
                &mut self.0
            }
        }

        impl ops::Deref for $vec {
            type Target = [f32; $n];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ops::DerefMut for $vec {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl PartialEq for $vec {
            fn eq(&self, other: &$vec) -> bool {
                self.0 == other.0
            }
        }

        impl ops::Neg for $vec {
            type Output = $vec;

            fn neg(mut self) -> $vec {
                for i in 0..$n {
                    self[i] = -self[i];
                }
                self
            }
        }

        impl_vec_ops!($vec[$n], Add::add, AddAssign::add_assign);
        impl_vec_ops!($vec[$n], Sub::sub, SubAssign::sub_assign);
        impl_vec_ops!($vec[$n], Mul::mul, MulAssign::mul_assign);
        impl_vec_ops!($vec[$n], Div::div, DivAssign::div_assign);
        impl_vec_ops!($vec[$n], Rem::rem, RemAssign::rem_assign);
    }
}

impl_vec! {
    vec2_t[2] {
        x[0](set_x),
        y[1](set_y),
    }
}

impl_vec! {
    vec3_t[3] {
        x[0](set_x),
        y[1](set_y),
        z[2](set_z),
    }
}

impl_vec! {
    vec4_t[4] {
        x[0](set_x),
        y[1](set_y),
        z[2](set_z),
        w[3](set_w),
    }
}

impl vec2_t {
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const NX: Self = Self::new(-1.0, 0.0);
    pub const NY: Self = Self::new(0.0, -1.0);
}

impl vec3_t {
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const NX: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NY: Self = Self::new(0.0, -1.0, 0.0);
    pub const NZ: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn copy_with_x(self, x: f32) -> Self {
        Self::new(x, self.y(), self.z())
    }

    pub const fn copy_with_y(self, y: f32) -> Self {
        Self::new(self.x(), y, self.z())
    }

    pub const fn copy_with_z(self, z: f32) -> Self {
        Self::new(self.x(), self.y(), z)
    }

    pub fn cross_product(self, other: Self) -> Self {
        let a = &self.0;
        let b = &other.0;
        Self::new(
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        )
    }

    pub const fn pitch(&self) -> f32 {
        self.0[PITCH]
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.0[PITCH] = pitch;
    }

    pub const fn yaw(&self) -> f32 {
        self.0[YAW]
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.0[YAW] = yaw;
    }

    pub const fn roll(&self) -> f32 {
        self.0[ROLL]
    }

    pub fn set_roll(&mut self, roll: f32) {
        self.0[ROLL] = roll;
    }
}

impl vec4_t {
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const NX: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    pub const NY: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    pub const NZ: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    pub const NW: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    pub const fn copy_with_z(self, z: f32) -> Self {
        Self::new(self.x(), self.y(), z, self.w())
    }
}
