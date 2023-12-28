#![feature(iter_map_windows)]

pub mod axis;
pub mod bar;
pub mod color;
pub mod legend;
pub mod pie;
pub mod point;

pub use bar::{BarChart, BarChartOptions};
pub use color::{CalculatedColor, ChartColor, Color, Gradient, Palette, CATPPUCCIN_COLORS};
pub use pie::{PieChart, PieChartOptions};
pub use point::{Point, Series};
