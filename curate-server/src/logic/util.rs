use crate::data::models::Rate;

pub(crate) fn get_min_rate(rates: &[Rate]) -> f64 {
    rates.iter()
        .map(|r| r.rate)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0)
}

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