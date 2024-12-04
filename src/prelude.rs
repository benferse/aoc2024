use num_traits::Num;

pub fn gcd<T: Num + Copy>(lhs: T, rhs: T) -> T {
    if rhs.is_zero() {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

pub fn lcm<T: Num + Copy>(lhs: T, rhs: T) -> T {
    (lhs * rhs) as T / gcd(lhs, rhs) as T
}

pub struct TrimmedLines<'a>(std::str::Lines<'a>);

impl<'a> Iterator for TrimmedLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::trim)
    }
}

pub struct MapLines<'a, F>(std::iter::Map<std::str::Lines<'a>, F>);

impl<F, B> Iterator for MapLines<'_, F>
where
    F: FnMut(&str) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub trait StrExt<'a> {
    fn trimmed_lines(&self) -> TrimmedLines;
    fn map_lines<F, B>(&self, f: F) -> MapLines<F>
    where
        F: FnMut(&str) -> B;
}

impl<T: AsRef<str>> StrExt<'_> for T {
    fn trimmed_lines(&self) -> TrimmedLines {
        TrimmedLines(self.as_ref().lines())
    }

    fn map_lines<F, B>(&self, f: F) -> MapLines<F>
    where
        F: FnMut(&str) -> B,
    {
        MapLines(self.as_ref().lines().map(f))
    }
}

pub trait Join<'a> {
    fn join(self, sep: &'a str) -> String;
}

impl<'a, T> Join<'a> for T
where
    T: Iterator<Item = &'a str>,
{
    fn join(self, sep: &'a str) -> String {
        self.intersperse(sep).collect()
    }
}
