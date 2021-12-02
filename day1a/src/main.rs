use std::{fmt::Display, fs, num::ParseIntError, ops::Index, str::FromStr};

fn main() {
    let raw_input = fs::read_to_string("day1A.txt").expect("Reading input file failed !!!");
    let input = Measurements::from_str(&raw_input).expect("Parsing raw input failed !!!");

    let mut inscreased = 0;
    for m in (&input)[..].windows(2) {
        if m[0] < m[1] {
            inscreased += 1;
        }
    }

    println!("Increased count: {}", inscreased);
}

struct Measurements {
    inner: Vec<u32>,
}

impl<'a, Idx> Index<Idx> for &'a Measurements
where
    Idx: std::slice::SliceIndex<[u32]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
    }
}

impl Display for Measurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl FromStr for Measurements {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measurements: Vec<&str> = s.split('\n').collect();

        let mut inner = Vec::new();

        for measure in measurements {
            let m: u32 = measure.parse()?;
            inner.push(m);
        }

        Ok(Self { inner })
    }
}

struct MeasurementsIntoIterator {
    index: usize,
    measurements: Measurements,
}

impl Iterator for MeasurementsIntoIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let inner_len = self.measurements.inner.len();
        if self.index < inner_len {
            let item = self.measurements.inner[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl IntoIterator for Measurements {
    type Item = u32;
    type IntoIter = MeasurementsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MeasurementsIntoIterator {
            index: 0,
            measurements: self,
        }
    }
}

struct MeasurementsIterator<'a> {
    index: usize,
    measurements: &'a Measurements,
}

impl<'a> Iterator for MeasurementsIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let inner_len = self.measurements.inner.len();
        if self.index < inner_len {
            let item = self.measurements.inner[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Measurements {
    type Item = u32;
    type IntoIter = MeasurementsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MeasurementsIterator {
            index: 0,
            measurements: self,
        }
    }
}
