//! f32 vectors, matrices, and quaternions evaluated by the vendored GLM headers.
//!
//! Vector `*` is component-wise, matching GLM. Use [`Vec3::dot`] and [`Vec3::cross`]
//! for those products. [`include_dir`] is the GLM header root for C++ compiled
//! from the same Cargo build. Downstream build scripts also see `DEP_GLM_INCLUDE`.

use std::ops::{Add, Div, Mul, Neg, Sub};

unsafe extern "C" {
    fn glm_vec2_add(a: *const Vec2, b: *const Vec2, out: *mut Vec2);
    fn glm_vec2_sub(a: *const Vec2, b: *const Vec2, out: *mut Vec2);
    fn glm_vec2_mul(a: *const Vec2, b: *const Vec2, out: *mut Vec2);
    fn glm_vec2_div(a: *const Vec2, b: *const Vec2, out: *mut Vec2);
    fn glm_vec2_scale(v: *const Vec2, s: f32, out: *mut Vec2);
    fn glm_vec2_neg(v: *const Vec2, out: *mut Vec2);
    fn glm_vec2_dot(a: *const Vec2, b: *const Vec2) -> f32;
    fn glm_vec2_length(v: *const Vec2) -> f32;
    fn glm_vec2_normalize(v: *const Vec2, out: *mut Vec2);
    fn glm_vec2_mix(a: *const Vec2, b: *const Vec2, t: f32, out: *mut Vec2);

    fn glm_vec3_add(a: *const Vec3, b: *const Vec3, out: *mut Vec3);
    fn glm_vec3_sub(a: *const Vec3, b: *const Vec3, out: *mut Vec3);
    fn glm_vec3_mul(a: *const Vec3, b: *const Vec3, out: *mut Vec3);
    fn glm_vec3_div(a: *const Vec3, b: *const Vec3, out: *mut Vec3);
    fn glm_vec3_scale(v: *const Vec3, s: f32, out: *mut Vec3);
    fn glm_vec3_neg(v: *const Vec3, out: *mut Vec3);
    fn glm_vec3_dot(a: *const Vec3, b: *const Vec3) -> f32;
    fn glm_vec3_cross(a: *const Vec3, b: *const Vec3, out: *mut Vec3);
    fn glm_vec3_length(v: *const Vec3) -> f32;
    fn glm_vec3_normalize(v: *const Vec3, out: *mut Vec3);
    fn glm_vec3_mix(a: *const Vec3, b: *const Vec3, t: f32, out: *mut Vec3);
    fn glm_vec3_reflect(i: *const Vec3, n: *const Vec3, out: *mut Vec3);

    fn glm_vec4_add(a: *const Vec4, b: *const Vec4, out: *mut Vec4);
    fn glm_vec4_sub(a: *const Vec4, b: *const Vec4, out: *mut Vec4);
    fn glm_vec4_mul(a: *const Vec4, b: *const Vec4, out: *mut Vec4);
    fn glm_vec4_div(a: *const Vec4, b: *const Vec4, out: *mut Vec4);
    fn glm_vec4_scale(v: *const Vec4, s: f32, out: *mut Vec4);
    fn glm_vec4_neg(v: *const Vec4, out: *mut Vec4);
    fn glm_vec4_dot(a: *const Vec4, b: *const Vec4) -> f32;
    fn glm_vec4_length(v: *const Vec4) -> f32;
    fn glm_vec4_normalize(v: *const Vec4, out: *mut Vec4);
    fn glm_vec4_mix(a: *const Vec4, b: *const Vec4, t: f32, out: *mut Vec4);

    fn glm_mat4_identity(out: *mut Mat4);
    fn glm_mat4_mul(a: *const Mat4, b: *const Mat4, out: *mut Mat4);
    fn glm_mat4_mul_vec4(m: *const Mat4, v: *const Vec4, out: *mut Vec4);
    fn glm_mat4_translate(m: *const Mat4, v: *const Vec3, out: *mut Mat4);
    fn glm_mat4_rotate(m: *const Mat4, angle: f32, axis: *const Vec3, out: *mut Mat4);
    fn glm_mat4_scale(m: *const Mat4, v: *const Vec3, out: *mut Mat4);
    fn glm_mat4_inverse(m: *const Mat4, out: *mut Mat4);
    fn glm_mat4_transpose(m: *const Mat4, out: *mut Mat4);
    fn glm_mat4_determinant(m: *const Mat4) -> f32;
    fn glm_perspective(fovy: f32, aspect: f32, near_plane: f32, far_plane: f32, out: *mut Mat4);
    fn glm_ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane: f32,
        far_plane: f32,
        out: *mut Mat4,
    );
    fn glm_look_at(eye: *const Vec3, center: *const Vec3, up: *const Vec3, out: *mut Mat4);

    fn glm_quat_identity(out: *mut Quat);
    fn glm_quat_mul(a: *const Quat, b: *const Quat, out: *mut Quat);
    fn glm_quat_mul_vec3(q: *const Quat, v: *const Vec3, out: *mut Vec3);
    fn glm_quat_normalize(q: *const Quat, out: *mut Quat);
    fn glm_quat_conjugate(q: *const Quat, out: *mut Quat);
    fn glm_quat_inverse(q: *const Quat, out: *mut Quat);
    fn glm_quat_slerp(a: *const Quat, b: *const Quat, t: f32, out: *mut Quat);
    fn glm_quat_angle_axis(angle: f32, axis: *const Vec3, out: *mut Quat);
    fn glm_quat_from_euler(pitch: f32, yaw: f32, roll: f32, out: *mut Quat);
    fn glm_quat_euler(q: *const Quat, out: *mut Vec3);
    fn glm_quat_to_mat4(q: *const Quat, out: *mut Mat4);
    fn glm_quat_from_mat4(m: *const Mat4, out: *mut Quat);
    fn glm_radians(degrees: f32) -> f32;
    fn glm_degrees(radians: f32) -> f32;
}

/// Absolute path of the vendored GLM headers (`glm/glm.hpp` lives under it).
pub fn include_dir() -> &'static str {
    env!("GLM_INCLUDE")
}

pub fn radians(degrees: f32) -> f32 {
    unsafe { glm_radians(degrees) }
}

pub fn degrees(radians: f32) -> f32 {
    unsafe { glm_degrees(radians) }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Column-major 4x4 matrix. `cols[column][row]` matches `glm::mat4`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

/// Quaternion stored as x, y, z, w, matching `glm::quat`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn dot(self, other: Self) -> f32 {
        unsafe { glm_vec2_dot(&self, &other) }
    }
    pub fn length(self) -> f32 {
        unsafe { glm_vec2_length(&self) }
    }
    pub fn normalize(self) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec2_normalize(&self, &mut out) };
        out
    }
    pub fn mix(self, other: Self, t: f32) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec2_mix(&self, &other, t, &mut out) };
        out
    }
    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(self, other: Self) -> f32 {
        unsafe { glm_vec3_dot(&self, &other) }
    }
    pub fn cross(self, other: Self) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec3_cross(&self, &other, &mut out) };
        out
    }
    pub fn length(self) -> f32 {
        unsafe { glm_vec3_length(&self) }
    }
    pub fn normalize(self) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec3_normalize(&self, &mut out) };
        out
    }
    pub fn mix(self, other: Self, t: f32) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec3_mix(&self, &other, t, &mut out) };
        out
    }
    pub fn reflect(self, normal: Self) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec3_reflect(&self, &normal, &mut out) };
        out
    }
    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn dot(self, other: Self) -> f32 {
        unsafe { glm_vec4_dot(&self, &other) }
    }
    pub fn length(self) -> f32 {
        unsafe { glm_vec4_length(&self) }
    }
    pub fn normalize(self) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec4_normalize(&self, &mut out) };
        out
    }
    pub fn mix(self, other: Self, t: f32) -> Self {
        let mut out = Self::ZERO;
        unsafe { glm_vec4_mix(&self, &other, t, &mut out) };
        out
    }
    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut out = Self {
            cols: [[0.0; 4]; 4],
        };
        unsafe { glm_mat4_identity(&mut out) };
        out
    }
    pub fn translate(self, offset: Vec3) -> Self {
        let mut out = self;
        unsafe { glm_mat4_translate(&self, &offset, &mut out) };
        out
    }
    pub fn rotate(self, angle_radians: f32, axis: Vec3) -> Self {
        let mut out = self;
        unsafe { glm_mat4_rotate(&self, angle_radians, &axis, &mut out) };
        out
    }
    pub fn scale(self, factors: Vec3) -> Self {
        let mut out = self;
        unsafe { glm_mat4_scale(&self, &factors, &mut out) };
        out
    }
    pub fn inverse(self) -> Self {
        let mut out = self;
        unsafe { glm_mat4_inverse(&self, &mut out) };
        out
    }
    pub fn transpose(self) -> Self {
        let mut out = self;
        unsafe { glm_mat4_transpose(&self, &mut out) };
        out
    }
    pub fn determinant(self) -> f32 {
        unsafe { glm_mat4_determinant(&self) }
    }
    pub fn perspective(fovy_radians: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut out = Self::identity();
        unsafe { glm_perspective(fovy_radians, aspect, near, far, &mut out) };
        out
    }
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut out = Self::identity();
        unsafe { glm_ortho(left, right, bottom, top, near, far, &mut out) };
        out
    }
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let mut out = Self::identity();
        unsafe { glm_look_at(&eye, &center, &up, &mut out) };
        out
    }
    pub fn mul_vec4(self, vector: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        unsafe { glm_mat4_mul_vec4(&self, &vector, &mut out) };
        out
    }
    /// Column-major pointer, the same order as `glm::value_ptr`.
    pub fn as_ptr(&self) -> *const f32 {
        self.cols.as_ptr().cast()
    }
}

impl Quat {
    pub fn identity() -> Self {
        let mut out = Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        unsafe { glm_quat_identity(&mut out) };
        out
    }
    pub fn normalize(self) -> Self {
        let mut out = self;
        unsafe { glm_quat_normalize(&self, &mut out) };
        out
    }
    pub fn conjugate(self) -> Self {
        let mut out = self;
        unsafe { glm_quat_conjugate(&self, &mut out) };
        out
    }
    pub fn inverse(self) -> Self {
        let mut out = self;
        unsafe { glm_quat_inverse(&self, &mut out) };
        out
    }
    pub fn slerp(self, other: Self, t: f32) -> Self {
        let mut out = self;
        unsafe { glm_quat_slerp(&self, &other, t, &mut out) };
        out
    }
    pub fn angle_axis(angle_radians: f32, axis: Vec3) -> Self {
        let mut out = Self::identity();
        unsafe { glm_quat_angle_axis(angle_radians, &axis, &mut out) };
        out
    }
    /// Pitch, yaw, and roll in radians. Same convention as `glm::quat(glm::vec3)`.
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
        let mut out = Self::identity();
        unsafe { glm_quat_from_euler(pitch, yaw, roll, &mut out) };
        out
    }
    pub fn euler_angles(self) -> Vec3 {
        let mut out = Vec3::ZERO;
        unsafe { glm_quat_euler(&self, &mut out) };
        out
    }
    pub fn to_mat4(self) -> Mat4 {
        let mut out = Mat4::identity();
        unsafe { glm_quat_to_mat4(&self, &mut out) };
        out
    }
    pub fn from_mat4(matrix: Mat4) -> Self {
        let mut out = Self::identity();
        unsafe { glm_quat_from_mat4(&matrix, &mut out) };
        out
    }
    pub fn mul_vec3(self, vector: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        unsafe { glm_quat_mul_vec3(&self, &vector, &mut out) };
        out
    }
    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

macro_rules! vec_ops {
    ($ty:ty, $add:ident, $sub:ident, $mul:ident, $div:ident, $scale:ident, $neg:ident) => {
        impl Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                let mut out = self;
                unsafe { $add(&self, &rhs, &mut out) };
                out
            }
        }
        impl Sub for $ty {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                let mut out = self;
                unsafe { $sub(&self, &rhs, &mut out) };
                out
            }
        }
        impl Mul for $ty {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                let mut out = self;
                unsafe { $mul(&self, &rhs, &mut out) };
                out
            }
        }
        impl Div for $ty {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                let mut out = self;
                unsafe { $div(&self, &rhs, &mut out) };
                out
            }
        }
        impl Mul<f32> for $ty {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self {
                let mut out = self;
                unsafe { $scale(&self, rhs, &mut out) };
                out
            }
        }
        impl Div<f32> for $ty {
            type Output = Self;
            fn div(self, rhs: f32) -> Self {
                let mut out = self;
                unsafe { $scale(&self, 1.0 / rhs, &mut out) };
                out
            }
        }
        impl Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self {
                let mut out = self;
                unsafe { $neg(&self, &mut out) };
                out
            }
        }
    };
}

vec_ops!(
    Vec2,
    glm_vec2_add,
    glm_vec2_sub,
    glm_vec2_mul,
    glm_vec2_div,
    glm_vec2_scale,
    glm_vec2_neg
);
vec_ops!(
    Vec3,
    glm_vec3_add,
    glm_vec3_sub,
    glm_vec3_mul,
    glm_vec3_div,
    glm_vec3_scale,
    glm_vec3_neg
);
vec_ops!(
    Vec4,
    glm_vec4_add,
    glm_vec4_sub,
    glm_vec4_mul,
    glm_vec4_div,
    glm_vec4_scale,
    glm_vec4_neg
);

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut out = self;
        unsafe { glm_mat4_mul(&self, &rhs, &mut out) };
        out
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.mul_vec4(rhs)
    }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut out = self;
        unsafe { glm_quat_mul(&self, &rhs, &mut out) };
        out
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
    }

    #[test]
    fn vector_algebra_matches_glm() {
        let left = Vec3::new(1.0, 2.0, 3.0);
        let right = Vec3::new(4.0, 5.0, 6.0);
        let sum = left + right;
        close(sum.x, 5.0);
        close(sum.z, 9.0);
        close(left.dot(Vec3::X), 1.0);
        let cross = Vec3::X.cross(Vec3::Y);
        close(cross.z, 1.0);
        let normal = Vec3::new(0.0, 3.0, 0.0).normalize();
        close(normal.y, 1.0);
        close(radians(180.0), std::f32::consts::PI);
        assert!(include_dir().ends_with("glm"));
    }

    #[test]
    fn matrices_use_glms_column_major_layout() {
        let translated = Mat4::identity().translate(Vec3::new(1.0, 2.0, 3.0));
        close(translated.cols[3][0], 1.0);
        close(translated.cols[3][1], 2.0);
        close(translated.cols[3][2], 3.0);
        close(translated.determinant(), 1.0);

        let fovy = radians(90.0);
        let projection = Mat4::perspective(fovy, 1.0, 0.1, 100.0);
        close(projection.cols[0][0], 1.0);
        close(projection.cols[1][1], 1.0);
        close(projection.cols[2][3], -1.0);

        let view = Mat4::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        let origin = view * Vec4::new(0.0, 0.0, 0.0, 1.0);
        close(origin.z, -5.0);
    }

    #[test]
    fn quaternions_rotate_with_glm() {
        let turn = Quat::angle_axis(radians(90.0), Vec3::Y).normalize();
        let rotated = turn * Vec3::X;
        close(rotated.x, 0.0);
        close(rotated.z, -1.0);
        let matrix = turn.to_mat4();
        let back = Quat::from_mat4(matrix).normalize();
        close(back.w, turn.w);
        close(back.y, turn.y);
    }
}
