use num_traits::ToPrimitive;

#[derive(Clone, Debug, PartialEq)]
pub struct TickSpacing {
    pub min_point: f64,
    pub max_point: f64,
    pub spacing: f64,
    pub num_ticks: u8,
}

#[allow(clippy::collapsible_else_if)]
pub fn nice_num(num: f64, round: bool) -> f64 {
    let exponent = num.log10().floor();
    let fraction = num / 10.0f64.powf(exponent);
    let nice_fraction = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else {
        if fraction <= 1.0 {
            1.0
        } else if fraction <= 2.0 {
            2.0
        } else if fraction <= 5.0 {
            5.0
        } else {
            10.0
        }
    };
    nice_fraction * 10.0f64.powf(exponent)
}

pub fn nice_ticks(min: f64, max: f64, max_ticks: u8) -> TickSpacing {
    let range = nice_num(max - min, false);
    let spacing = nice_num(range / (max_ticks - 1) as f64, true);
    let min_point = (min / spacing).floor() * spacing;
    let max_point = (max / spacing).ceil() * spacing;
    let num_ticks = ((max_point - min_point) / spacing) as u8 + 1;
    TickSpacing {
        min_point,
        max_point,
        spacing,
        num_ticks,
    }
}

#[allow(clippy::ptr_arg)]
pub fn get_min_max<T>(values: &Vec<T>) -> (f64, f64)
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    let min_max = values
        .iter()
        .map(|v| v.to_f64().unwrap())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), v| {
            (f64::min(a, v), f64::max(b, v))
        });
    (
        if min_max.0 < 0.0 { min_max.0 } else { 0.0 },
        if min_max.1 > 0.0 { min_max.1 } else { 0.0 },
    )
}

pub fn get_ticks(ticks: &TickSpacing) -> Vec<(f64, String)> {
    (0..ticks.num_ticks)
        .map(|i| ticks.min_point + i as f64 * ticks.spacing)
        .map(move |tick| {
            (
                100.0 - (tick - ticks.min_point) / (ticks.max_point - ticks.min_point) * 100.0,
                format!("{}", tick),
            )
        })
        .collect::<Vec<(f64, String)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max() {
        let values = vec![-4, 10, 0, 50, 2, -6, 7];
        assert_eq!(get_min_max(&values), (-6.0, 50.0));
        let values = vec![-4.9, 10.0, 0.8, 50.2, 2.7, -6.3, 7.5];
        assert_eq!(get_min_max(&values), (-6.3, 50.2));
        let values = vec![-4, -10, -3, -50, -2, -6, -7];
        assert_eq!(get_min_max(&values), (-50.0, 0.0));
        let values = vec![4, 10, 2, 50, 2, 6, 7];
        assert_eq!(get_min_max(&values), (0.0, 50.0));
    }

    #[test]
    fn ticks() {
        let ticks = nice_ticks(-10.0, 10.0, 10);
        assert_eq!(ticks.min_point, -10.0);
        assert_eq!(ticks.max_point, 10.0);
        assert_eq!(ticks.spacing, 2.0);
        assert_eq!(ticks.num_ticks, 11);

        let ticks = get_ticks(&ticks);
        assert_eq!(ticks[0].0, 100.0);
        assert_eq!(ticks[0].1, "-10");
        assert_eq!(ticks[4].0, 60.0);
        assert_eq!(ticks[4].1, "-2");
        assert_eq!(ticks[10].0, 0.0);
        assert_eq!(ticks[10].1, "10");
    }
}
