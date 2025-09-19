struct Camera {
    double4 position;
    double4 right;
    double4 up;
    double4 forward;
    double scale; // This is the tangent of FOV
    double aspect_ratio; // Width / height
    double screen_width;
    double screen_height;
};