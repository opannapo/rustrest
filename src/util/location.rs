use rand::Rng;

pub fn generate_random_lat_lon() -> (f64, f64) {
    let mut rng = rand::thread_rng();

    // Rentang koordinat masing negara
    let indonesia_bounds = (-11.0, 6.0, 95.0, 141.0); // (min_lat, max_lat, min_lon, max_lon)
    let malaysia_bounds = (0.85, 7.4, 99.6, 119.3);
    let singapore_bounds = (1.2, 1.5, 103.6, 104.0);

    // Random select negara
    let country = rng.gen_range(0..3);

    // Tentukan batas sesuai selected negara
    let (min_lat, max_lat, min_lon, max_lon) = match country {
        0 => indonesia_bounds,
        1 => malaysia_bounds,
        _ => singapore_bounds,
    };

    // Generate random lat lon
    let latitude = rng.gen_range(min_lat..max_lat);
    let longitude = rng.gen_range(min_lon..max_lon);

    (latitude, longitude)
}
