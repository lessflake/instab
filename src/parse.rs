use crate::{
    set::{Daily, DailyOrCm, Instabs},
    Date, Fractal, Instability, Set,
};

const fn parse_data() -> [Set; 366] {
    Parser::new(include_str!("../instabilities.csv")).run()
}

const PARSED_DATA: [Set; 366] = parse_data();

pub trait Parsable {
    fn parse(date: Date) -> &'static Self;
}

impl Parsable for Set {
    fn parse(date: Date) -> &'static Self {
        &PARSED_DATA[date.day() as usize]
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            bytes: data.as_bytes(),
            pos: 30,
        }
    }

    #[rustfmt::skip]
    const fn run(&mut self) -> [Set; 366] {
        [
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
            self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(), self.parse_set(),
        ]
    }

    const fn parse_set(&mut self) -> Set {
        self.pos += 6;
        let d1 = self.parse_daily_or_cm();
        let d2 = self.parse_daily_or_cm();
        let d3 = self.parse_daily_or_cm();
        let nm = self.parse_cm();
        let so = self.parse_cm();
        let sp = self.parse_cm();
        self.pos += 1;

        Set::new(d1, d2, d3, nm, so, sp)
    }

    const fn parse_daily_or_cm(&mut self) -> DailyOrCm {
        let res = match self.bytes[self.pos] {
            b'"' => DailyOrCm::Daily(self.parse_daily()),
            b'I' => DailyOrCm::Nightmare,
            b'J' => DailyOrCm::ShatteredObservatory,
            b'U' => DailyOrCm::SunquaPeak,
            _ => unreachable!(),
        };
        self.pos += 2;
        res
    }

    const fn parse_daily(&mut self) -> Daily {
        self.pos += 1;
        let fractal = Fractal::from_u8(self.bytes[self.pos]).unwrap();
        self.pos += 2;
        let instabs = self.parse_instabs();
        let mut alt = None;
        if self.bytes[self.pos] == b'/' {
            self.pos += 3;
            alt = Some(self.parse_instabs());
        }
        Daily::new_with_alt(fractal, instabs, alt)
    }

    const fn parse_cm(&mut self) -> Instabs {
        self.pos += 3;
        let instabs = self.parse_instabs();
        self.pos += 2;
        instabs
    }

    #[inline]
    const fn parse_instabs(&mut self) -> Instabs {
        let one = Instability::from_u8(self.parse_u8()).unwrap();
        self.pos += 1;
        let two = Instability::from_u8(self.parse_u8()).unwrap();
        self.pos += 1;
        let three = Instability::from_u8(self.parse_u8()).unwrap();
        Instabs::new(one, two, three)
    }

    #[inline]
    const fn parse_u8(&mut self) -> u8 {
        let mut n = self.bytes[self.pos].wrapping_sub(b'0');
        self.pos += 1;
        let next = self.bytes[self.pos].wrapping_sub(b'0');

        if next < 10 {
            self.pos += 1;
            n = n * 10 + next;
        }

        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_size() {
        assert_eq!(
            core::mem::size_of::<crate::set::Nightmare>(),
            core::mem::size_of::<Instabs>()
        );

        assert_eq!(
            core::mem::size_of::<Instabs>(),
            core::mem::size_of::<[Instability; 3]>()
        );

        assert_eq!(
            core::mem::size_of::<Instability>(),
            core::mem::size_of::<u8>()
        );
    }
}
