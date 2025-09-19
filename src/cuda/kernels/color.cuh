const float INVERSE_GAMMA = 1.0 / 2.2;

struct Color {
    float r;
    float g;
    float b;
    float a;

    __device__
    Color(): r(0.0f), g(0.0f), b(0.0f), a(0.0f) {}

    __device__
    Color(float r, float g, float b, float a): r(r), g(g), b(b), a(a) {}

    __device__
    Color gamma_correct() const {
        return Color(
            powf(r, INVERSE_GAMMA),
            powf(g, INVERSE_GAMMA),
            powf(b, INVERSE_GAMMA),
            a
        );
    }
};

struct ColorState {
    Color color;
    float transmittance;

    __device__
    ColorState(Color color, float transmittance): color(color), transmittance(transmittance) {}

    __device__
    ColorState(): color(Color()), transmittance(0.0f) {}

    __device__
    void blend(Color sample_color) {
        color.r += sample_color.r *  sample_color.a * transmittance;
        color.g += sample_color.g *  sample_color.a * transmittance;
        color.b += sample_color.b *  sample_color.a * transmittance;
        float new_transmittance = transmittance * (1.0 - sample_color.a);
        color.a = 1. - new_transmittance;
        transmittance = new_transmittance;
    }
};
