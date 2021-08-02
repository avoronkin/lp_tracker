
pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.is_empty() {
        return None
    }

    let first = series.first()?;
    let last = series.last()?;
    let percentage = (first / last ) * 100_f64;
    let absolute_difference = first - last;

    Some((percentage, absolute_difference))

}

pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.is_empty() || n < 1 {
        return None
    }

    let mut sma: Vec<f64> = vec!();
    let len = series.len();

    for i in 0..len {
        let it = series.iter().rev().skip(i).take(n);
        let count = it.len();
        let sum: f64= it.sum();
        sma.push(sum / count as f64);
    }

    Some(sma.iter().rev().cloned().collect())
}

pub fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None
    }

    let mut min = f64::MAX;

    for num in series.iter() {
        min = num.min(min);
    };

    Some(min)
}

pub fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None
    }

    let mut max = f64::MIN;

    for num in series.iter() {
        max = num.max(max);
    };

    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_diff() {
        let series: Vec<f64> = vec![2.0, 3.0, 10.0];
        assert_eq!(price_diff(&series), Some((20.0, -8.0)));
    }

    #[test]
    fn test_price_diff_on_empty() {
        assert_eq!(price_diff(&[]), None);
    }

    #[test]
    fn test_n_window_sma() {
        let series: Vec<f64> = vec![2.0, 3.0, 10.0];
        assert_eq!(n_window_sma(2, &series), Some(vec![2.0, 2.5, 6.5]));
    }

    #[test]
    fn test_n_window_sma_on_empty() {
        assert_eq!(n_window_sma(2, &[]), None);
    }

    #[test]
    fn test_min() {
        let series: Vec<f64> = vec![2.0, 3.0, 10.0];
        assert_eq!(min(&series), Some(2.0));
    }

    #[test]
    fn test_min_on_empty() {
        assert_eq!(min(&[]), None);
    }

    #[test]
    fn test_max() {
        let series: Vec<f64> = vec![2.0, 3.0, 10.0];
        assert_eq!(max(&series), Some(10.0));
    }

    #[test]
    fn test_max_on_empty() {
        assert_eq!(max(&[]), None);
    }
}
