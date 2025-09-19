#pragma once

struct Hyperparameters {
    double dλ0;
    double bounding_box_radius;
    unsigned int num_integration_steps;
    unsigned int normalization_interval;
    double integration_error_tolerance;
    double min_dλ;
    double max_dλ;
    unsigned int max_retries;
};
