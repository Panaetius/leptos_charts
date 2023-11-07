use num_traits::ToPrimitive;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point<T> {
    pub value: T,
    pub label: String,
}

impl<T> From<(T, String)> for Point<T>
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    fn from(point: (T, String)) -> Point<T> {
        Point {
            value: point.0,
            label: point.1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Series<T>(Vec<Point<T>>);

impl<T> From<Vec<(T, String)>> for Series<T>
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    fn from(values: Vec<(T, String)>) -> Series<T> {
        Series(values.into_iter().map(|p| p.into()).collect())
    }
}

impl<T> IntoIterator for Series<T> {
    type Item = Point<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
