struct AccretionDisk {
    double r_isco;
    double accretion_r_max;
    double width;
    double max_temperature;
    double step_opacity;
    double doppler_factor;
    double fade_start_ratio;
    double peak_brightness;

    __device__
    AccretionDisk() : 
        r_isco(0.0), 
        accretion_r_max(0.0), 
        width(0.0), 
        max_temperature(0.0),
        step_opacity(0.0), 
        doppler_factor(0.0), 
        fade_start_ratio(0.0), 
        peak_brightness(0.0) {}

    __device__
    AccretionDisk(double r_isco, double accretion_r_max, double width, double max_temperature,
                  double step_opacity, double doppler_factor, double fade_start_ratio, double peak_brightness) :
        r_isco(r_isco),
        accretion_r_max(accretion_r_max),
        width(width),
        max_temperature(max_temperature),
        step_opacity(step_opacity),
        doppler_factor(doppler_factor),
        fade_start_ratio(fade_start_ratio),
        peak_brightness(peak_brightness) {}

};
