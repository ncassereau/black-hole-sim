const float INVERSE_GAMMA = 1.0 / 2.2;

struct Color {
    float r;
    float g;
    float b;
    float a;

    __device__ Color(): r(0.0f), g(0.0f), b(0.0f), a(0.0f) {}

    __device__ Color(float r, float g, float b, float a): r(r), g(g), b(b), a(a) {}

    __device__ Color gamma_correct() const {
        return Color(
            powf(r, INVERSE_GAMMA),
            powf(g, INVERSE_GAMMA),
            powf(b, INVERSE_GAMMA),
            a
        );
    }

    __device__ float transmittance() const {
        return 1.0f - a;
    }

    __device__ void blend(const Color& sample_color) {
        r += sample_color.r *  sample_color.a * transmittance();
        g += sample_color.g *  sample_color.a * transmittance();
        b += sample_color.b *  sample_color.a * transmittance();
        a = 1.0f - transmittance() * sample_color.transmittance();
    }
};
