#pragma once

#include "color.cuh"
#include "tensor_ops.cuh"

__device__ Color kelvin_to_rgb(float temp_kelvin) {
    float temp = temp_kelvin / 100.0f;

    float red = (temp <= 66.0f)
                    ? 255.0f
                    : 329.698727446f * __powf(temp - 60.0f, -0.1332047592f);

    float green = (temp <= 66.0f)
                      ? 99.4708025861f * __logf(temp) - 161.1195681661f
                      : 288.1221695283f * __powf(temp - 60.0f, -0.0755148492f);

    float blue;
    if (temp <= 19.0f) {
        blue = 0.0f;
    } else if (temp >= 66.0f) {
        blue = 255.0f;
    } else {
        blue = 138.5177312231f * __logf(temp - 10.0f) - 305.0447927307f;
    }

    return Color(red, green, blue, 1.0f);
}

__device__ double brightness(double r_isco, double radius) {
    return (1.0 - __dsqrt_rn(r_isco / radius)) /
           (radius * radius * radius + 1e-10);
}

struct AccretionDisk {
    double r_isco;
    double accretion_r_max;
    double width;
    double max_temperature;
    double step_opacity;
    double doppler_factor;
    double fade_start_ratio;
    double peak_brightness;

    __device__ AccretionDisk()
        : r_isco(0.0), accretion_r_max(0.0), width(0.0), max_temperature(0.0),
          step_opacity(0.0), doppler_factor(0.0), fade_start_ratio(0.0),
          peak_brightness(0.0) {}

    __device__ AccretionDisk(double r_isco, double accretion_r_max,
                             double width, double max_temperature,
                             double step_opacity, double doppler_factor,
                             double fade_start_ratio, double peak_brightness)
        : r_isco(r_isco), accretion_r_max(accretion_r_max), width(width),
          max_temperature(max_temperature), step_opacity(step_opacity),
          doppler_factor(doppler_factor), fade_start_ratio(fade_start_ratio),
          peak_brightness(peak_brightness) {}

    __device__ double check_intersection(float3 position1,
                                         float3 position2) const {
        float dz = position2.z - position1.z;
        if (fabs(dz) < 1e-10) return -1.0;

        float t = -position1.z / dz;
        if (t < 0. || t > 1.) return -1.0;

        float3 equator_collision = position1 + (position2 - position1) * t;
        double r_plane = (double)length(equator_collision);
        if (r_plane < r_isco || r_plane > accretion_r_max) return -1;

        return r_plane;
    }

    __device__ Color get_color(double radius) const {
        if (radius < r_isco || radius > accretion_r_max) return Color();

        double normalized_brightness =
            min(brightness(r_isco, radius) / peak_brightness, 1.0);
        double normalized_radius = (radius - r_isco) / width;
        double geometric_falloff =
            1.0 - smoothstep(fade_start_ratio, 1.0, normalized_radius);

        // Stefan-Boltzmann's Law
        float temp_k =
            (float)max_temperature * __powf((float)normalized_brightness, 0.25);

        // Planck's Law
        Color rgb = kelvin_to_rgb(temp_k);

        // Doppler Effect (disabled for now)
        double doppler_boost = 1.0;

        float emitted_intensity =
            (float)(normalized_brightness * doppler_boost * geometric_falloff);

        float r = rgb.r * emitted_intensity;
        float g = rgb.g * emitted_intensity;
        float b = rgb.b * emitted_intensity;
        float a = (float)step_opacity * emitted_intensity;
        return Color(r, g, b, a);
    }
};
