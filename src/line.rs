use std::cmp;

use crate::{axis::YAxis, utils, ChartColor, Color, Palette, CATPPUCCIN_COLORS};
use itertools::Itertools;
use leptos::{svg::*, *};
use leptos_use::*;
use num_traits::ToPrimitive;

pub struct LineChartOptions {
    pub max_ticks: u8,
    pub color: Box<dyn ChartColor>,
}

impl Default for LineChartOptions {
    fn default() -> Self {
        Self {
            max_ticks: 5u8,
            color: Box::new(Palette(vec![Color::Hex("#dd3333")])),
        }
    }
}

#[component]
pub fn LineChart<T, U>(
    values: MaybeSignal<Vec<(T, U)>>,
    options: Box<LineChartOptions>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
    U: ToPrimitive + Clone + PartialOrd + 'static,
{
    let values = create_memo(move |_| {
        values
            .get()
            .into_iter()
            .map(|(x, y)| (x.to_f64().unwrap(), y.to_f64().unwrap()))
            .collect::<Vec<(f64, f64)>>()
    });
    let min_max = create_memo(move |_| {
        values.get().iter().fold(
            (
                (f64::INFINITY, f64::NEG_INFINITY),
                (f64::INFINITY, f64::NEG_INFINITY),
            ),
            |((acc_min_x, acc_max_x), (acc_min_y, acc_max_y)), (x, y)| {
                (
                    (f64::min(acc_min_x, *x), f64::max(acc_max_x, *x)),
                    (f64::min(acc_min_y, *y), f64::max(acc_max_y, *y)),
                )
            },
        )
    });
    let max_ticks = options.max_ticks;
    let tick_config =
        create_memo(move |_| utils::nice_ticks(min_max.get().1 .0, min_max.get().1 .1, max_ticks));
    let ticks = create_memo(move |_| tick_config.with(utils::get_ticks));
    view! {
        <svg {..attrs}>
            <YAxis ticks=ticks/>
            <svg
                x="10%"
                y="10%"
                width="90%"
                height="80%"
                viewBox="0 0 100 100"
                preserveAspectRatio="none"
            >
                <g transform="matrix(1 0 0 -1 0 100)">
                    <defs>
                        <linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop
                                offset="0%"
                                stop-color=String::from(options.color.color_for_index(0, 2))
                            ></stop>
                            <stop
                                offset="100%"
                                stop-color=String::from(options.color.color_for_index(1, 2))
                            ></stop>
                        </linearGradient>
                    </defs>
                    <polyline
                        fill="none"
                        style="stroke:url(#gradient)"
                        stroke-width="1"
                        vector-effect="non-scaling-stroke"
                        stroke-linejoin="round"
                        points=move || {
                            values
                                .get()
                                .into_iter()
                                .map(|(x, y)| (
                                    100.0 * (x - min_max.get().0.0)
                                        / (min_max.get().0.1 - min_max.get().0.0),
                                    100.0 * (y - tick_config.get().min_point)
                                        / (tick_config.get().max_point
                                            - tick_config.get().min_point),
                                ))
                                .map(|(x, y)| format!("{},{}", x, y))
                                .intersperse(" ".to_string())
                                .collect::<String>()
                        }
                    >
                    </polyline>

                </g>
            </svg>
        </svg>
    }
}
