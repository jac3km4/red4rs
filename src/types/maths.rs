use crate::raw::root::RED4ext as red;
use crate::repr::NativeRepr;

#[repr(transparent)]
pub struct Vector2(red::Vector2);

impl Vector2 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.X
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.Y
    }
}

#[cfg(feature = "mint")]
impl From<mint::Vector2<f32>> for Vector2 {
    fn from(value: mint::Vector2<f32>) -> Self {
        Self(red::Vector2 {
            X: value.x,
            Y: value.y,
        })
    }
}

#[cfg(feature = "mint")]
impl From<Vector2> for mint::Vector2<f32> {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.0.X,
            y: value.0.Y,
        }
    }
}

unsafe impl NativeRepr for Vector2 {
    const NAME: &'static str = "Vector2";
}

#[repr(transparent)]
pub struct Vector3(red::Vector3);

impl Vector3 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.X
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.Y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0.Z
    }
}

unsafe impl NativeRepr for Vector3 {
    const NAME: &'static str = "Vector3";
}

#[cfg(feature = "mint")]
impl From<mint::Vector3<f32>> for Vector3 {
    fn from(value: mint::Vector3<f32>) -> Self {
        Self(red::Vector3 {
            X: value.x,
            Y: value.y,
            Z: value.z,
        })
    }
}

#[cfg(feature = "mint")]
impl From<Vector3> for mint::Vector3<f32> {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.0.X,
            y: value.0.Y,
            z: value.0.Z,
        }
    }
}

#[repr(transparent)]
pub struct Vector4(red::Vector4);

impl Vector4 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.X
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.Y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0.Z
    }

    #[inline]
    pub fn w(&self) -> f32 {
        self.0.W
    }
}

unsafe impl NativeRepr for Vector4 {
    const NAME: &'static str = "Vector4";
}

#[cfg(feature = "mint")]
impl From<mint::Vector4<f32>> for Vector4 {
    fn from(value: mint::Vector4<f32>) -> Self {
        Self(red::Vector4 {
            X: value.x,
            Y: value.y,
            Z: value.z,
            W: value.w,
        })
    }
}

#[cfg(feature = "mint")]
impl From<Vector4> for mint::Vector4<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.0.X,
            y: value.0.Y,
            z: value.0.Z,
            w: value.0.W,
        }
    }
}

#[repr(transparent)]
pub struct Quaternion(red::Quaternion);

impl Quaternion {
    #[inline]
    pub fn i(&self) -> f32 {
        self.0.i
    }

    #[inline]
    pub fn j(&self) -> f32 {
        self.0.j
    }

    #[inline]
    pub fn k(&self) -> f32 {
        self.0.k
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.0.r
    }
}

unsafe impl NativeRepr for Quaternion {
    const NAME: &'static str = "Quaternion";
}

#[cfg(feature = "mint")]
impl From<mint::Quaternion<f32>> for Quaternion {
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self(red::Quaternion {
            i: value.v.x,
            j: value.v.y,
            k: value.v.z,
            r: value.s,
        })
    }
}

#[cfg(feature = "mint")]
impl From<Quaternion> for mint::Quaternion<f32> {
    fn from(value: Quaternion) -> Self {
        Self {
            v: mint::Vector3 {
                x: value.0.i,
                y: value.0.j,
                z: value.0.k,
            },
            s: value.0.r,
        }
    }
}

#[repr(transparent)]
pub struct EulerAngles(red::EulerAngles);

impl EulerAngles {
    #[inline]
    pub fn roll(&self) -> f32 {
        self.0.Roll
    }

    #[inline]
    pub fn pitch(&self) -> f32 {
        self.0.Pitch
    }

    #[inline]
    pub fn yaw(&self) -> f32 {
        self.0.Yaw
    }
}

#[cfg(feature = "mint")]
impl<B> From<mint::EulerAngles<f32, B>> for EulerAngles {
    fn from(value: mint::EulerAngles<f32, B>) -> Self {
        Self(red::EulerAngles {
            Roll: value.c,
            Pitch: value.a,
            Yaw: value.b,
        })
    }
}

#[cfg(feature = "mint")]
impl<B> From<EulerAngles> for mint::EulerAngles<f32, B> {
    fn from(value: EulerAngles) -> Self {
        Self {
            c: value.0.Roll,
            a: value.0.Pitch,
            b: value.0.Yaw,
            marker: std::marker::PhantomData,
        }
    }
}
