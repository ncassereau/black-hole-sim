__device__ inline double4 operator*(const double4& lhs, const double4& rhs) {
    return make_double4(
        lhs.x * rhs.x,
        lhs.y * rhs.y,
        lhs.z * rhs.z,
        lhs.w * rhs.w
    );
}

__device__ inline double4 operator+(const double4& lhs, const double4& rhs) {
    return make_double4(
        lhs.x + rhs.x,
        lhs.y + rhs.y,
        lhs.z + rhs.z,
        lhs.w + rhs.w
    );
}

__device__ inline double4 operator-(const double4& lhs, const double4& rhs) {
    return make_double4(
        lhs.x - rhs.x,
        lhs.y - rhs.y,
        lhs.z - rhs.z,
        lhs.w - rhs.w
    );
}

__device__ inline double4 operator/(const double4& lhs, const double4& rhs) {
    return make_double4(
        lhs.x / rhs.x,
        lhs.y / rhs.y,
        lhs.z / rhs.z,
        lhs.w / rhs.w
    );
}

__device__ inline double4 operator*(const double4& lhs, double rhs) {
    return make_double4(
        lhs.x * rhs,
        lhs.y * rhs,
        lhs.z * rhs,
        lhs.w * rhs
    );
}

__device__ inline double4 operator*(double lhs, const double4& rhs) {
    return make_double4(
        lhs * rhs.x,
        lhs * rhs.y,
        lhs * rhs.z,
        lhs * rhs.w
    );
}

__device__ inline double4 operator+(const double4& lhs, double rhs) {
    return make_double4(
        lhs.x + rhs,
        lhs.y + rhs,
        lhs.z + rhs,
        lhs.w + rhs
    );
}

__device__ inline double4 operator+(double lhs, const double4& rhs) {
    return make_double4(
        lhs + rhs.x,
        lhs + rhs.y,
        lhs + rhs.z,
        lhs + rhs.w
    );
}

__device__ inline double4 operator-(const double4& lhs, double rhs) {
    return make_double4(
        lhs.x - rhs,
        lhs.y - rhs,
        lhs.z - rhs,
        lhs.w - rhs
    );
}

__device__ inline double4 operator-(double lhs, const double4& rhs) {
    return make_double4(
        lhs - rhs.x,
        lhs - rhs.y,
        lhs - rhs.z,
        lhs - rhs.w
    );
}

__device__ inline double4 operator/(const double4& lhs, double rhs) {
    return make_double4(
        lhs.x / rhs,
        lhs.y / rhs,
        lhs.z / rhs,
        lhs.w / rhs
    );
}

__device__ inline double4 operator/(double lhs, const double4& rhs) {
    return make_double4(
        lhs / rhs.x,
        lhs / rhs.y,
        lhs / rhs.z,
        lhs / rhs.w
    );
}

__device__ inline double4 operator-(const double4& v) {
    return make_double4(-v.x, -v.y, -v.z, -v.w);
}

__device__ inline double4& operator+=(double4& lhs, const double4& rhs) {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
    lhs.w += rhs.w;
    return lhs;
}

__device__ inline double4& operator-=(double4& lhs, const double4& rhs) {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    lhs.z -= rhs.z;
    lhs.w -= rhs.w;
    return lhs;
}

__device__ inline double4& operator*=(double4& lhs, const double4& rhs) {
    lhs.x *= rhs.x;
    lhs.y *= rhs.y;
    lhs.z *= rhs.z;
    lhs.w *= rhs.w;
    return lhs;
}

__device__ inline double4& operator/=(double4& lhs, const double4& rhs) {
    lhs.x /= rhs.x;
    lhs.y /= rhs.y;
    lhs.z /= rhs.z;
    lhs.w /= rhs.w;
    return lhs;
}

__device__ inline double4& operator*=(double4& lhs, double rhs) {
    lhs.x *= rhs;
    lhs.y *= rhs;
    lhs.z *= rhs;
    lhs.w *= rhs;
    return lhs;
}

__device__ inline double4& operator/=(double4& lhs, double rhs) {
    lhs.x /= rhs;
    lhs.y /= rhs;
    lhs.z /= rhs;
    lhs.w /= rhs;
    return lhs;
}
