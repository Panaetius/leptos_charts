#![feature(iter_map_windows)]
use std::{f64::consts::TAU, iter};

use itertools::Itertools;
use leptos::{svg::*, *};
use leptos_use::*;
use num_traits::ToPrimitive;

const CATPPUCCIN_COLORS: &[&str] = &[
    "#dc8a78", //rosewater
    "#8839ef", //Mauve
    "#fe640b", //Peach
    "#40a02b", //green
    "#04a5e5", //Sky
    "#ea76cb", //Pink
    "#1e66f5", //Blue
    "#d20f39", //Red
    "#df8e1d", //yellow
    "#209fb5", //Sapphire
    "#7287fd", //lavender
    "#e64553", //maroon
];

#[derive(Debug, Clone)]
pub enum Color<'a> {
    Hex(&'a str),
    RGB(u8, u8, u8),
}

#[derive(Debug, Clone)]
pub enum ChartColor<'a, F>
where
    F: Fn(usize) -> Color<'a>,
{
    Palette(Vec<Color<'a>>),
    Gradient(Color<'a>, Color<'a>),
    Calcuated(F),
}

pub struct ChartOptions {
    pub max_ticks: u8,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self { max_ticks: 5u8 }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TickSpacing {
    min_point: f64,
    max_point: f64,
    spacing: f64,
    num_ticks: u8,
}

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

#[component]
pub fn BarChart<T>(
    values: MaybeSignal<Vec<T>>,
    options: ChartOptions,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    let vals = values.clone();
    let num_bars = create_memo(move |_| vals.get().len() as f64);
    let vals = values.clone();
    let max = create_memo(move |_| {
        vals.get()
            .iter()
            .map(|v| v.to_f64().unwrap())
            .fold(f64::NEG_INFINITY, f64::max)
    });
    let vals = values.clone();
    let min = create_memo(move |_| {
        let min = vals
            .get()
            .iter()
            .map(|v| v.to_f64().unwrap())
            .fold(f64::INFINITY, f64::min);
        if min < 0.0 {
            min
        } else {
            0.0
        }
    });
    let vals = values.clone();
    let values = create_memo(move |_| {
        vals.get()
            .into_iter()
            .map(|v| v.to_f64().unwrap())
            .zip(CATPPUCCIN_COLORS.into_iter().cycle())
            .enumerate()
            .collect::<Vec<(usize, (f64, &&str))>>()
    });
    let tick_config = create_memo(move |_| nice_ticks(min.get(), max.get(), options.max_ticks));
    let ticks = create_memo(move |_| {
        let ticks = tick_config.get();
        (0..ticks.num_ticks)
            .map(|i| ticks.min_point + i as f64 * ticks.spacing)
            .map(move |tick| {
                (
                    100.0 - (tick - ticks.min_point) / (ticks.max_point - ticks.min_point) * 100.0,
                    format!("{}", tick),
                )
            })
            .collect::<Vec<(f64, String)>>()
    });

    view! {
        <svg {..attrs}>
            <svg y="10%" height="80%" overflow="visible">
                <line
                    x1="9.8%"
                    y1="0%"
                    x2="9.8%"
                    y2="100%"
                    stroke="black"
                    stroke-width="1px"
                    vector-effect="non-scaling-stroke"
                ></line>
                {move || {
                    ticks
                        .get()
                        .into_iter()
                        .map(|(t, s)| {
                            view! {
                                <line
                                    x1="7%"
                                    y1=format!("{}%", t)
                                    x2="9.8%"
                                    y2=format!("{}%", t)
                                    stroke="black"
                                    strocke-width="1px"
                                    vector-effect="non-scaling-stroke"
                                ></line>
                                <text
                                    x="6.9%"
                                    y=format!("{}%", t)
                                    font-size="20px"
                                    dy="5px"
                                    text-anchor="end"
                                    vector-effect="non-scaling-stroke"
                                >
                                    {s}
                                </text>
                            }
                        })
                        .collect_view()
                }}

                {move || {
                    values
                        .get()
                        .into_iter()
                        .map(|(i, (v, color))| {
                            let el = create_node_ref::<Rect>();
                            let is_hovered = use_element_hover(el);
                            view! {
                                <svg
                                    x="10%"
                                    width="90%"
                                    height="100%"
                                    viewBox="0 0 100 100"
                                    preserveAspectRatio="none"
                                >
                                    <g transform="matrix(1 0 0 -1 0 100)">
                                        <rect
                                            node_ref=el
                                            x=move || (5.0 + 95.0 / num_bars.get() * i as f64)
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

                                            width=move || (80.0 / num_bars.get())
                                            height=move || {
                                                100.0 * v.abs()
                                                    / (tick_config.get().max_point
                                                        - tick_config.get().min_point)
                                            }

                                            fill=*color
                                            fill-opacity=move || {
                                                if is_hovered.get() { "0.8" } else { "0.6" }
                                            }

                                            stroke=*color
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
                                                "{}%", (15.0 + 85.0 / num_bars.get() * (i as f64 + 0.5))
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
        </svg>
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PieSegment {
    from: (f64, f64),
    to: (f64, f64),
    value: f64,
}
enum SegmentSize {
    LessThanHalf,
    Half,
    MoreThanHalf,
}
impl PieSegment {
    fn angle(&self) -> SegmentSize {
        let zcross = self.from.0 * self.to.1 - self.to.0 * self.from.1;
        if zcross == 0.0 {
            SegmentSize::Half
        } else if zcross > 0.0 {
            SegmentSize::LessThanHalf
        } else {
            SegmentSize::MoreThanHalf
        }
    }
    fn get_arc_path(&self) -> String {
        let angle = self.angle();

        let large_arc_flag = match angle {
            SegmentSize::LessThanHalf | SegmentSize::Half => 0,
            SegmentSize::MoreThanHalf => 1,
        };

        format!(
            "M0 0 {from_x} {from_y} A100 100 0 {arc_flag} 1 {to_x} {to_y}Z",
            from_x = self.from.0,
            from_y = self.from.1,
            to_x = self.to.0,
            to_y = self.to.1,
            arc_flag = large_arc_flag
        )
    }

    // Gets a middle vector for two vectors in a circle segment
    // This points in the direction of a circle segment's center
    // even if the angle of the segment is >= 180°
    // uses the cross product to figure out the angle and flips the vector
    // if it's larger than 180°. For the 180° case, it creates a new vector
    // 90° clockwise perpendicular to the from vector.
    fn get_center_unit_vector(&self) -> (f64, f64) {
        match self.angle() {
            SegmentSize::Half => {
                let magnitude = f64::sqrt(self.from.0.powi(2) + self.from.1.powi(2));
                (self.from.1 / magnitude, -self.from.0 / magnitude)
            }
            SegmentSize::LessThanHalf => {
                let new_x = (self.from.0 + self.to.0) / 2.0;
                let new_y = (self.from.1 + self.to.1) / 2.0;
                let magnitude = f64::sqrt(new_x.powi(2) + new_y.powi(2));

                (new_x / magnitude, new_y / magnitude)
            }
            SegmentSize::MoreThanHalf => {
                let new_x = (self.from.0 + self.to.0) / 2.0;
                let new_y = (self.from.1 + self.to.1) / 2.0;
                let magnitude = f64::sqrt(new_x.powi(2) + new_y.powi(2));

                (-new_x / magnitude, -new_y / magnitude)
            }
        }
    }
}

#[component]
pub fn PieChart<T>(
    values: MaybeSignal<Vec<T>>,
    options: ChartOptions,
    // colors: Option<&'chart [&'chart str]>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    let values = create_memo(move |_| {
        values
            .get()
            .iter()
            .map(|v| v.to_f64().unwrap())
            .collect::<Vec<f64>>()
    });
    let sum = create_memo(move |_| values.get().iter().sum::<f64>());
    let sorted_values = create_memo(move |_| {
        iter::once((0.0, 99.0, 0.0))
            .chain(
                values
                    .get()
                    .into_iter()
                    .sorted_by(|a, b| f64::partial_cmp(a, b).unwrap())
                    .map(|f| (f, f / sum.get()))
                    .scan((0.0, 0.0), |state, v| {
                        *state = (v.0, state.1 + v.1);
                        Some(*state)
                    })
                    .map(|(f, v)| (f, (v * TAU).cos() * 99.0, (v * TAU).sin() * 99.0)),
            )
            .map_windows(|[from, to]| PieSegment {
                from: (from.1, from.2),
                to: (to.1, to.2),
                value: to.0,
            })
            .zip(CATPPUCCIN_COLORS.into_iter().cycle())
            .collect::<Vec<(PieSegment, &&str)>>()
    });

    view! {
        <svg {..attrs}>
            {move || {
                sorted_values
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, (segment, color))| {
                        let el = create_node_ref::<Path>();
                        let is_hovered = use_element_hover(el);
                        let label_pos = segment.get_center_unit_vector();
                        view! {
                            <svg viewBox="0 0 200 200">
                                <g transform="translate(100,100)" stroke="#000" stroke-width="1">
                                    <mask id=format!("cut-path-{}", i)>
                                        <path
                                            d=segment.get_arc_path()
                                            fill="white"
                                            stroke="black"
                                            stroke-width="2"
                                            vector-effect="non-scaling-stroke"
                                        ></path>
                                    </mask>
                                    <path
                                        node_ref=el
                                        d=segment.get_arc_path()
                                        fill=*color
                                        fill-opacity=0.6
                                        stroke=*color
                                        stroke-width="2"
                                        vector-effect="non-scaling-stroke"
                                        mask=move || {
                                            if is_hovered.get() {
                                                "none".to_string()
                                            } else {
                                                format!("url(#cut-path-{})", i)
                                            }
                                        }
                                    >
                                    </path>
                                    <Show when=move || is_hovered.get() fallback=|| ()>
                                        <text
                                            font-size="15px"
                                            vector-effect="non-scaling-stroke"
                                            x=label_pos.0 * 85.0
                                            y=label_pos.1 * 85.0
                                        >
                                            <tspan
                                                text-anchor="middle"
                                                dominant-baseline="middle"
                                                color="#000"
                                            >
                                                {segment.value}
                                            </tspan>
                                        </text>
                                    </Show>
                                </g>
                            </svg>
                        }
                    })
                    .collect_view()
            }}

        </svg>
    }
}
