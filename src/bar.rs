use crate::{axis::YAxis, utils, ChartColor, Palette, CATPPUCCIN_COLORS};
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
    let min_max = create_memo(move |_| vals.with(utils::get_min_max));
    let values = create_memo(move |_| {
        values
            .get()
            .into_iter()
            .map(|v| v.to_f64().unwrap())
            .enumerate()
            .collect::<Vec<(usize, f64)>>()
    });
    let max_ticks = options.max_ticks;
    let tick_config =
        create_memo(move |_| utils::nice_ticks(min_max.get().0, min_max.get().1, max_ticks));
    let ticks = create_memo(move |_| tick_config.with(utils::get_ticks));

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
                                            "{}%",
                                            (15.0 + 85.0 / num_bars.get() as f64 * (i as f64 + 0.5)),
                                        )
                                    }

                                    y=move || {
                                        format!(
                                            "{}%",
                                            (100.0
                                                - 100.0 * (v - tick_config.get().min_point)
                                                    / (tick_config.get().max_point
                                                        - tick_config.get().min_point)),
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
