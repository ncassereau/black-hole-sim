__device__ inline double dot(const double3 &a, const double3 &b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

__device__ inline double length(const double3 &v) {
    return __fsqrt_rn(dot(v, v));
}

__device__ inline double3 normalize(const double3 &v) {
    double inv_len = 1.0 / length(v);
    return make_double3(v.x * inv_len, v.y * inv_len, v.z * inv_len);
}

__device__ inline double3 operator*(const double3 &lhs, const double3 &rhs) {
    return make_double3(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z);
}

__device__ inline double3 operator+(const double3 &lhs, const double3 &rhs) {
    return make_double3(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z);
}

__device__ inline double3 operator-(const double3 &lhs, const double3 &rhs) {
    return make_double3(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z);
}

__device__ inline double3 operator/(const double3 &lhs, const double3 &rhs) {
    return make_double3(lhs.x / rhs.x, lhs.y / rhs.y, lhs.z / rhs.z);
}

__device__ inline double3 operator*(const double3 &lhs, double rhs) {
    return make_double3(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs);
}

__device__ inline double3 operator*(double lhs, const double3 &rhs) {
    return make_double3(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z);
}

__device__ inline double3 operator+(const double3 &lhs, double rhs) {
    return make_double3(lhs.x + rhs, lhs.y + rhs, lhs.z + rhs);
}

__device__ inline double3 operator+(double lhs, const double3 &rhs) {
    return make_double3(lhs + rhs.x, lhs + rhs.y, lhs + rhs.z);
}

__device__ inline double3 operator-(const double3 &lhs, double rhs) {
    return make_double3(lhs.x - rhs, lhs.y - rhs, lhs.z - rhs);
}

__device__ inline double3 operator-(double lhs, const double3 &rhs) {
    return make_double3(lhs - rhs.x, lhs - rhs.y, lhs - rhs.z);
}

__device__ inline double3 operator/(const double3 &lhs, double rhs) {
    return make_double3(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs);
}

__device__ inline double3 operator/(double lhs, const double3 &rhs) {
    return make_double3(lhs / rhs.x, lhs / rhs.y, lhs / rhs.z);
}

__device__ inline double3 operator-(const double3 &v) {
    return make_double3(-v.x, -v.y, -v.z);
}

__device__ inline double3 &operator+=(double3 &lhs, const double3 &rhs) {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
    return lhs;
}

__device__ inline double3 &operator-=(double3 &lhs, const double3 &rhs) {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    lhs.z -= rhs.z;
    return lhs;
}

__device__ inline double3 &operator*=(double3 &lhs, const double3 &rhs) {
    lhs.x *= rhs.x;
    lhs.y *= rhs.y;
    lhs.z *= rhs.z;
    return lhs;
}

__device__ inline double3 &operator/=(double3 &lhs, const double3 &rhs) {
    lhs.x /= rhs.x;
    lhs.y /= rhs.y;
    lhs.z /= rhs.z;
    return lhs;
}

__device__ inline double3 &operator*=(double3 &lhs, double rhs) {
    lhs.x *= rhs;
    lhs.y *= rhs;
    lhs.z *= rhs;
    return lhs;
}

__device__ inline double3 &operator/=(double3 &lhs, double rhs) {
    lhs.x /= rhs;
    lhs.y /= rhs;
    lhs.z /= rhs;
    return lhs;
}
