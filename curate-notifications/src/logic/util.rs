pub(crate) fn get_multiplier(rate: f64) -> i64 {
    let mut multiplier = 1;

    if rate == 0.0 {
        return multiplier;
    }

    for m in 0..9 {
        multiplier = 10_i64.pow(m);
        if rate * (multiplier as f64) >= 1.0 {
            break;
        }
    };
    multiplier
}