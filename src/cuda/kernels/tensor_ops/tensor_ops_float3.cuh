__device__ inline float dot(const float3 &a, const float3 &b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

__device__ inline float length(const float3 &v) {
    return __fsqrt_rn(dot(v, v));
}

__device__ inline float3 normalize(const float3 &v) {
    double inv_len = 1.0 / length(v);
    return make_float3(v.x * inv_len, v.y * inv_len, v.z * inv_len);
}

__device__ inline float3 operator*(const float3 &lhs, const float3 &rhs) {
    return make_float3(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z);
}

__device__ inline float3 operator+(const float3 &lhs, const float3 &rhs) {
    return make_float3(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z);
}

__device__ inline float3 operator-(const float3 &lhs, const float3 &rhs) {
    return make_float3(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z);
}

__device__ inline float3 operator/(const float3 &lhs, const float3 &rhs) {
    return make_float3(lhs.x / rhs.x, lhs.y / rhs.y, lhs.z / rhs.z);
}

__device__ inline float3 operator*(const float3 &lhs, float rhs) {
    return make_float3(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs);
}

__device__ inline float3 operator*(float lhs, const float3 &rhs) {
    return make_float3(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z);
}

__device__ inline float3 operator+(const float3 &lhs, float rhs) {
    return make_float3(lhs.x + rhs, lhs.y + rhs, lhs.z + rhs);
}

__device__ inline float3 operator+(float lhs, const float3 &rhs) {
    return make_float3(lhs + rhs.x, lhs + rhs.y, lhs + rhs.z);
}

__device__ inline float3 operator-(const float3 &lhs, float rhs) {
    return make_float3(lhs.x - rhs, lhs.y - rhs, lhs.z - rhs);
}

__device__ inline float3 operator-(float lhs, const float3 &rhs) {
    return make_float3(lhs - rhs.x, lhs - rhs.y, lhs - rhs.z);
}

__device__ inline float3 operator/(const float3 &lhs, float rhs) {
    return make_float3(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs);
}

__device__ inline float3 operator/(float lhs, const float3 &rhs) {
    return make_float3(lhs / rhs.x, lhs / rhs.y, lhs / rhs.z);
}

__device__ inline float3 operator-(const float3 &v) {
    return make_float3(-v.x, -v.y, -v.z);
}

__device__ inline float3 &operator+=(float3 &lhs, const float3 &rhs) {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
    return lhs;
}

__device__ inline float3 &operator-=(float3 &lhs, const float3 &rhs) {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    lhs.z -= rhs.z;
    return lhs;
}

__device__ inline float3 &operator*=(float3 &lhs, const float3 &rhs) {
    lhs.x *= rhs.x;
    lhs.y *= rhs.y;
    lhs.z *= rhs.z;
    return lhs;
}

__device__ inline float3 &operator/=(float3 &lhs, const float3 &rhs) {
    lhs.x /= rhs.x;
    lhs.y /= rhs.y;
    lhs.z /= rhs.z;
    return lhs;
}

__device__ inline float3 &operator*=(float3 &lhs, float rhs) {
    lhs.x *= rhs;
    lhs.y *= rhs;
    lhs.z *= rhs;
    return lhs;
}

__device__ inline float3 &operator/=(float3 &lhs, float rhs) {
    lhs.x /= rhs;
    lhs.y /= rhs;
    lhs.z /= rhs;
    return lhs;
}
