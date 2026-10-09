#include <glm/glm.hpp>
#include <glm/ext/quaternion_common.hpp>
#include <glm/ext/quaternion_trigonometric.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>
#include <glm/gtc/type_ptr.hpp>

// GLM stores a quaternion as x, y, z, w. The component constructor takes w first.
static_assert(sizeof(glm::vec2) == sizeof(float) * 2, "glm::vec2 layout");
static_assert(sizeof(glm::vec3) == sizeof(float) * 3, "glm::vec3 layout");
static_assert(sizeof(glm::vec4) == sizeof(float) * 4, "glm::vec4 layout");
static_assert(sizeof(glm::mat4) == sizeof(float) * 16, "glm::mat4 layout");
static_assert(sizeof(glm::quat) == sizeof(float) * 4, "glm::quat layout");

extern "C" {

struct glm_vec2 {
    float x, y;
};
struct glm_vec3 {
    float x, y, z;
};
struct glm_vec4 {
    float x, y, z, w;
};
struct glm_quat {
    float x, y, z, w;
};
struct glm_mat4 {
    float m[16];
};

static glm::vec2 to2(const glm_vec2& v) { return glm::vec2(v.x, v.y); }
static glm_vec2 from2(const glm::vec2& v) { return {v.x, v.y}; }
static glm::vec3 to3(const glm_vec3& v) { return glm::vec3(v.x, v.y, v.z); }
static glm_vec3 from3(const glm::vec3& v) { return {v.x, v.y, v.z}; }
static glm::vec4 to4(const glm_vec4& v) { return glm::vec4(v.x, v.y, v.z, v.w); }
static glm_vec4 from4(const glm::vec4& v) { return {v.x, v.y, v.z, v.w}; }
static glm::quat toq(const glm_quat& v) { return glm::quat(v.w, v.x, v.y, v.z); }
static glm_quat fromq(const glm::quat& v) { return {v.x, v.y, v.z, v.w}; }
static glm::mat4 tom(const glm_mat4& v) { return glm::make_mat4(v.m); }
static glm_mat4 fromm(const glm::mat4& v) {
    glm_mat4 out{};
    const float* pointer = glm::value_ptr(v);
    for (int index = 0; index < 16; ++index) {
        out.m[index] = pointer[index];
    }
    return out;
}

void glm_vec2_add(const glm_vec2* a, const glm_vec2* b, glm_vec2* out) { *out = from2(to2(*a) + to2(*b)); }
void glm_vec2_sub(const glm_vec2* a, const glm_vec2* b, glm_vec2* out) { *out = from2(to2(*a) - to2(*b)); }
void glm_vec2_mul(const glm_vec2* a, const glm_vec2* b, glm_vec2* out) { *out = from2(to2(*a) * to2(*b)); }
void glm_vec2_div(const glm_vec2* a, const glm_vec2* b, glm_vec2* out) { *out = from2(to2(*a) / to2(*b)); }
void glm_vec2_scale(const glm_vec2* v, float s, glm_vec2* out) { *out = from2(to2(*v) * s); }
void glm_vec2_neg(const glm_vec2* v, glm_vec2* out) { *out = from2(-to2(*v)); }
float glm_vec2_dot(const glm_vec2* a, const glm_vec2* b) { return glm::dot(to2(*a), to2(*b)); }
float glm_vec2_length(const glm_vec2* v) { return glm::length(to2(*v)); }
void glm_vec2_normalize(const glm_vec2* v, glm_vec2* out) { *out = from2(glm::normalize(to2(*v))); }
void glm_vec2_mix(const glm_vec2* a, const glm_vec2* b, float t, glm_vec2* out) { *out = from2(glm::mix(to2(*a), to2(*b), t)); }

void glm_vec3_add(const glm_vec3* a, const glm_vec3* b, glm_vec3* out) { *out = from3(to3(*a) + to3(*b)); }
void glm_vec3_sub(const glm_vec3* a, const glm_vec3* b, glm_vec3* out) { *out = from3(to3(*a) - to3(*b)); }
void glm_vec3_mul(const glm_vec3* a, const glm_vec3* b, glm_vec3* out) { *out = from3(to3(*a) * to3(*b)); }
void glm_vec3_div(const glm_vec3* a, const glm_vec3* b, glm_vec3* out) { *out = from3(to3(*a) / to3(*b)); }
void glm_vec3_scale(const glm_vec3* v, float s, glm_vec3* out) { *out = from3(to3(*v) * s); }
void glm_vec3_neg(const glm_vec3* v, glm_vec3* out) { *out = from3(-to3(*v)); }
float glm_vec3_dot(const glm_vec3* a, const glm_vec3* b) { return glm::dot(to3(*a), to3(*b)); }
void glm_vec3_cross(const glm_vec3* a, const glm_vec3* b, glm_vec3* out) { *out = from3(glm::cross(to3(*a), to3(*b))); }
float glm_vec3_length(const glm_vec3* v) { return glm::length(to3(*v)); }
void glm_vec3_normalize(const glm_vec3* v, glm_vec3* out) { *out = from3(glm::normalize(to3(*v))); }
void glm_vec3_mix(const glm_vec3* a, const glm_vec3* b, float t, glm_vec3* out) { *out = from3(glm::mix(to3(*a), to3(*b), t)); }
void glm_vec3_reflect(const glm_vec3* i, const glm_vec3* n, glm_vec3* out) { *out = from3(glm::reflect(to3(*i), to3(*n))); }

void glm_vec4_add(const glm_vec4* a, const glm_vec4* b, glm_vec4* out) { *out = from4(to4(*a) + to4(*b)); }
void glm_vec4_sub(const glm_vec4* a, const glm_vec4* b, glm_vec4* out) { *out = from4(to4(*a) - to4(*b)); }
void glm_vec4_mul(const glm_vec4* a, const glm_vec4* b, glm_vec4* out) { *out = from4(to4(*a) * to4(*b)); }
void glm_vec4_div(const glm_vec4* a, const glm_vec4* b, glm_vec4* out) { *out = from4(to4(*a) / to4(*b)); }
void glm_vec4_scale(const glm_vec4* v, float s, glm_vec4* out) { *out = from4(to4(*v) * s); }
void glm_vec4_neg(const glm_vec4* v, glm_vec4* out) { *out = from4(-to4(*v)); }
float glm_vec4_dot(const glm_vec4* a, const glm_vec4* b) { return glm::dot(to4(*a), to4(*b)); }
float glm_vec4_length(const glm_vec4* v) { return glm::length(to4(*v)); }
void glm_vec4_normalize(const glm_vec4* v, glm_vec4* out) { *out = from4(glm::normalize(to4(*v))); }
void glm_vec4_mix(const glm_vec4* a, const glm_vec4* b, float t, glm_vec4* out) { *out = from4(glm::mix(to4(*a), to4(*b), t)); }

void glm_mat4_identity(glm_mat4* out) { *out = fromm(glm::mat4(1.0f)); }
void glm_mat4_mul(const glm_mat4* a, const glm_mat4* b, glm_mat4* out) { *out = fromm(tom(*a) * tom(*b)); }
void glm_mat4_mul_vec4(const glm_mat4* m, const glm_vec4* v, glm_vec4* out) { *out = from4(tom(*m) * to4(*v)); }
void glm_mat4_translate(const glm_mat4* m, const glm_vec3* v, glm_mat4* out) { *out = fromm(glm::translate(tom(*m), to3(*v))); }
void glm_mat4_rotate(const glm_mat4* m, float angle, const glm_vec3* axis, glm_mat4* out) {
    *out = fromm(glm::rotate(tom(*m), angle, to3(*axis)));
}
void glm_mat4_scale(const glm_mat4* m, const glm_vec3* v, glm_mat4* out) { *out = fromm(glm::scale(tom(*m), to3(*v))); }
void glm_mat4_inverse(const glm_mat4* m, glm_mat4* out) { *out = fromm(glm::inverse(tom(*m))); }
void glm_mat4_transpose(const glm_mat4* m, glm_mat4* out) { *out = fromm(glm::transpose(tom(*m))); }
float glm_mat4_determinant(const glm_mat4* m) { return glm::determinant(tom(*m)); }
void glm_perspective(float fovy, float aspect, float near_plane, float far_plane, glm_mat4* out) {
    *out = fromm(glm::perspective(fovy, aspect, near_plane, far_plane));
}
void glm_ortho(float left, float right, float bottom, float top, float near_plane, float far_plane, glm_mat4* out) {
    *out = fromm(glm::ortho(left, right, bottom, top, near_plane, far_plane));
}
void glm_look_at(const glm_vec3* eye, const glm_vec3* center, const glm_vec3* up, glm_mat4* out) {
    *out = fromm(glm::lookAt(to3(*eye), to3(*center), to3(*up)));
}

void glm_quat_identity(glm_quat* out) { *out = fromq(glm::quat(1.0f, 0.0f, 0.0f, 0.0f)); }
void glm_quat_mul(const glm_quat* a, const glm_quat* b, glm_quat* out) { *out = fromq(toq(*a) * toq(*b)); }
void glm_quat_mul_vec3(const glm_quat* q, const glm_vec3* v, glm_vec3* out) { *out = from3(toq(*q) * to3(*v)); }
void glm_quat_normalize(const glm_quat* q, glm_quat* out) { *out = fromq(glm::normalize(toq(*q))); }
void glm_quat_conjugate(const glm_quat* q, glm_quat* out) { *out = fromq(glm::conjugate(toq(*q))); }
void glm_quat_inverse(const glm_quat* q, glm_quat* out) { *out = fromq(glm::inverse(toq(*q))); }
void glm_quat_slerp(const glm_quat* a, const glm_quat* b, float t, glm_quat* out) {
    *out = fromq(glm::slerp(toq(*a), toq(*b), t));
}
void glm_quat_angle_axis(float angle, const glm_vec3* axis, glm_quat* out) {
    *out = fromq(glm::angleAxis(angle, to3(*axis)));
}
void glm_quat_from_euler(float pitch, float yaw, float roll, glm_quat* out) {
    *out = fromq(glm::quat(glm::vec3(pitch, yaw, roll)));
}
void glm_quat_euler(const glm_quat* q, glm_vec3* out) { *out = from3(glm::eulerAngles(toq(*q))); }
void glm_quat_to_mat4(const glm_quat* q, glm_mat4* out) { *out = fromm(glm::mat4_cast(toq(*q))); }
void glm_quat_from_mat4(const glm_mat4* m, glm_quat* out) { *out = fromq(glm::quat_cast(tom(*m))); }

float glm_radians(float degrees) { return glm::radians(degrees); }
float glm_degrees(float radians) { return glm::degrees(radians); }

}
