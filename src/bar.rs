use crate::{axis::YAxis, ChartColor, Palette, CATPPUCCIN_COLORS};
use leptos::{svg::*, *};
use leptos_use::*;
use num_traits::ToPrimitive;

pub struct BarChartOptions {
    pub max_ticks: u8,
    pub color: Box<dyn ChartColor>,
}

impl Default for BarChartOptions {
    fn default() -> Self {
        Self {
            max_ticks: 5u8,
            color: Box::new(Palette(CATPPUCCIN_COLORS.clone())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TickSpacing {
    min_point: f64,
    max_point: f64,
    spacing: f64,
    num_ticks: u8,
}

#[allow(clippy::collapsible_else_if)]
fn nice_num(num: f64, round: bool) -> f64 {
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

fn nice_ticks(min: f64, max: f64, max_ticks: u8) -> TickSpacing {
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
fn get_min_max<T>(values: &Vec<T>) -> (f64, f64)
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

fn get_ticks(ticks: &TickSpacing) -> Vec<(f64, String)> {
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

/// Simple responsive bar chart
///
/// Example:
/// ```rust
/// use leptos_charts::*;
/// use leptos::*;
///
/// let data: Vec<f64> = vec![2.0, 3.0, 1.5, 7.0, 1.0, 2.5, 9.9];
/// let options = Box::new(BarChartOptions {
///     max_ticks: 4,
///     color: Box::new(Palette(CATPPUCCIN_COLORS.clone())),
/// });
/// # #[cfg(hydrate)]
/// # {
/// view!{
/// <BarChart
///     values=data.into()
///     options=options
///     attr:style="margin-top:5px"
///     attr:preserveAspectRatio="none"
///     attr:width="300"
///     attr:height="200"
/// />
/// }
/// # }
/// # ;
/// ```
#[component]
pub fn BarChart<T>(
    values: MaybeSignal<Vec<T>>,
    options: Box<BarChartOptions>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    let vals = values.clone();
    let num_bars = create_memo(move |_| vals.get().len());
    let vals = values.clone();
    let min_max = create_memo(move |_| vals.with(get_min_max));
    let values = create_memo(move |_| {
        values
            .get()
            .into_iter()
            .map(|v| v.to_f64().unwrap())
            .enumerate()
            .collect::<Vec<(usize, f64)>>()
    });
    let max_ticks = options.max_ticks;
    let tick_config = create_memo(move |_| nice_ticks(min_max.get().0, min_max.get().1, max_ticks));
    let ticks = create_memo(move |_| tick_config.with(get_ticks));

    view! {
        <svg {..attrs}>
            <YAxis ticks=ticks/>

            {move || {
                values
                    .get()
                    .into_iter()
                    .map(|(i, v)| {
                        let el = create_node_ref::<Rect>();
                        let is_hovered = use_element_hover(el);
                        let color = String::from(options.color.color_for_index(i, num_bars.get()));
                        view! {
                            <svg
                                x="10%"
                                y="10%"
                                width="90%"
                                height="80%"
                                viewBox="0 0 100 100"
                                preserveAspectRatio="none"
                            >
                                <g transform="matrix(1 0 0 -1 0 100)">
                                    <rect
                                        node_ref=el
                                        x=move || (5.0 + 95.0 / num_bars.get() as f64 * i as f64)
                                        y=move || {
                                            if v > 0.0 {
                                                100.0 * -tick_config.get().min_point
                                                    / (tick_config.get().max_point
                                                        - tick_config.get().min_point)
                                            } else {
                                                100.0 * (v - tick_config.get().min_point)
                                                    / (tick_config.get().max_point
                                                        - tick_config.get().min_point)
                                            }
                                        }

                                        width=move || (80.0 / num_bars.get() as f64)
                                        height=move || {
                                            100.0 * v.abs()
                                                / (tick_config.get().max_point
                                                    - tick_config.get().min_point)
                                        }

                                        fill=color.clone()
                                        fill-opacity=move || {
                                            if is_hovered.get() { "0.8" } else { "0.6" }
                                        }

                                        stroke=color
                                        stroke-width=move || {
                                            if is_hovered.get() { "3px" } else { "1px" }
                                        }

                                        vector-effect="non-scaling-stroke"
                                    ></rect>
                                </g>
                            </svg>
                            <Show when=move || is_hovered.get() fallback=|| ()>
                                <text
                                    font-size="15px"
                                    vector-effect="non-scaling-stroke"
                                    x=move || {
                                        format!(
                                            "{}%", (15.0 + 85.0 / num_bars.get() as f64 * (i as f64 +
                                            0.5))
                                        )
                                    }

                                    y=move || {
                                        format!(
                                            "{}%", (100.0 - 100.0 * (v - tick_config.get().min_point) /
                                            (tick_config.get().max_point - tick_config.get().min_point))
                                        )
                                    }

                                    dy=move || { if v > 0.0 { "-5" } else { "15" } }
                                    dx="-9"
                                >
                                    {v}
                                </text>
                            </Show>
                        }
                    })
                    .collect_view()
            }}

        </svg>
    }
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
