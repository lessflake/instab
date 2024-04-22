use crate::{Date, Fractal, Instability};

#[derive(Debug, Clone)]
pub struct Set {
    dailies: [DailyOrCm; 3],
    nm: Nightmare,
    so: ShatteredObservatory,
    sp: SunquaPeak,
}

#[test]
fn set_size() {
    assert_eq!(core::mem::size_of::<Set>(), 30);
}

impl Set {
    pub const fn new(
        d1: DailyOrCm,
        d2: DailyOrCm,
        d3: DailyOrCm,
        nm: Instabs,
        so: Instabs,
        sp: Instabs,
    ) -> Self {
        Self {
            dailies: [d1, d2, d3],
            nm: Nightmare::from_instabs(nm),
            so: ShatteredObservatory::from_instabs(so),
            sp: SunquaPeak::from_instabs(sp),
        }
    }

    pub fn dailies_iter(&self) -> impl Iterator<Item = &Daily> {
        self.dailies.iter().filter_map(|d| {
            if let DailyOrCm::Daily(ref daily) = d {
                Some(daily)
            } else {
                None
            }
        })
    }

    pub const fn nightmare(&self) -> &Nightmare {
        &self.nm
    }

    pub const fn shattered_observatory(&self) -> &ShatteredObservatory {
        &self.so
    }

    pub const fn sunqua_peak(&self) -> &SunquaPeak {
        &self.sp
    }
}

impl core::fmt::Display for Set {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}\n{}\n{}", self.nm, self.so, self.sp)?;
        if let DailyOrCm::Daily(daily) = &self.dailies[0] {
            writeln!(f, "{}", daily)?;
        } else if let DailyOrCm::Daily(daily) = &self.dailies[1] {
            writeln!(f, "{}", daily)?;
        } else if let DailyOrCm::Daily(daily) = &self.dailies[2] {
            writeln!(f, "{}", daily)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum DailyOrCm {
    Daily(Daily),
    Nightmare,
    ShatteredObservatory,
    SunquaPeak,
}

pub trait HasInstabs {
    fn instabs(&self) -> &Instabs;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instabs([Instability; 3]);

impl Instabs {
    pub const fn new(one: Instability, two: Instability, three: Instability) -> Self {
        Self([one, two, three])
    }

    pub fn as_slice(&self) -> &[Instability] {
        &self.0[..]
    }

    pub fn raw(&self) -> &[Instability; 3] {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &Instability> {
        self.0.iter()
    }
}

impl core::fmt::Display for Instabs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}; {}; {}", self.0[0], self.0[1], self.0[2])
    }
}

impl HasInstabs for Instabs {
    fn instabs(&self) -> &Instabs {
        &self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Daily {
    fractal: Fractal,
    instabs: Instabs,
    alt: Option<Instabs>,
}

impl Daily {
    pub const fn new(fractal: Fractal, instabs: Instabs) -> Self {
        Self {
            fractal,
            instabs,
            alt: None,
        }
    }

    pub const fn new_with_alt(fractal: Fractal, instabs: Instabs, alt: Option<Instabs>) -> Self {
        Self {
            fractal,
            instabs,
            alt,
        }
    }

    pub const fn fractal(&self) -> &Fractal {
        &self.fractal
    }

    pub fn borrow(&self) -> DailyBorrow<'_> {
        DailyBorrow::new(self.fractal, &self.instabs)
    }

    pub fn with_alt(mut self, alt: impl Into<Option<Instabs>>) -> Self {
        self.alt = alt.into();
        self
    }
}

impl core::fmt::Display for Daily {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.fractal, self.instabs)?;
        if let Some(alt) = self.alt.as_ref() {
            write!(f, " | {}", alt)?;
        }
        Ok(())
    }
}

impl HasInstabs for Daily {
    fn instabs(&self) -> &Instabs {
        &self.instabs
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nightmare(pub(crate) Instabs);

#[derive(Debug, Clone, PartialEq)]
pub struct ShatteredObservatory(pub(crate) Instabs);

#[derive(Debug, Clone, PartialEq)]
pub struct SunquaPeak(pub(crate) Instabs);

impl Nightmare {
    pub fn new(one: Instability, two: Instability, three: Instability) -> Self {
        Self(Instabs::new(one, two, three))
    }

    pub const fn from_instabs(instabs: Instabs) -> Self {
        Self(instabs)
    }

    pub fn as_daily(&self) -> DailyBorrow<'_> {
        DailyBorrow::new(Fractal::Nightmare, &self.0)
    }

    pub fn to_daily(&self) -> Daily {
        Daily::new(Fractal::Nightmare, self.0.clone())
    }

    pub fn into_daily(self) -> Daily {
        let Self(instabs) = self;
        Daily::new(Fractal::Nightmare, instabs)
    }
}

impl ShatteredObservatory {
    pub fn new(one: Instability, two: Instability, three: Instability) -> Self {
        Self(Instabs::new(one, two, three))
    }

    pub const fn from_instabs(instabs: Instabs) -> Self {
        Self(instabs)
    }

    pub fn as_daily(&self) -> DailyBorrow<'_> {
        DailyBorrow::new(Fractal::ShatteredObservatory, &self.0)
    }

    pub fn to_daily(&self) -> Daily {
        Daily::new(Fractal::ShatteredObservatory, self.0.clone())
    }

    pub fn into_daily(self) -> Daily {
        let Self(instabs) = self;
        Daily::new(Fractal::ShatteredObservatory, instabs)
    }
}

impl SunquaPeak {
    pub fn new(one: Instability, two: Instability, three: Instability) -> Self {
        Self(Instabs::new(one, two, three))
    }

    pub const fn from_instabs(instabs: Instabs) -> Self {
        Self(instabs)
    }

    pub fn to_daily(&self) -> Daily {
        Daily::new(Fractal::SunquaPeak, self.0.clone())
    }

    pub fn as_daily(&self) -> DailyBorrow<'_> {
        DailyBorrow::new(Fractal::SunquaPeak, &self.0)
    }

    pub fn into_daily(self) -> Daily {
        let Self(instabs) = self;
        Daily::new(Fractal::SunquaPeak, instabs)
    }
}

impl core::fmt::Display for Nightmare {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", Fractal::Nightmare, self.0)
    }
}

impl HasInstabs for Nightmare {
    fn instabs(&self) -> &Instabs {
        &self.0
    }
}

impl core::fmt::Display for ShatteredObservatory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", Fractal::ShatteredObservatory, self.0)
    }
}

impl HasInstabs for ShatteredObservatory {
    fn instabs(&self) -> &Instabs {
        &self.0
    }
}

impl core::fmt::Display for SunquaPeak {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", Fractal::SunquaPeak, self.0)
    }
}

impl HasInstabs for SunquaPeak {
    fn instabs(&self) -> &Instabs {
        &self.0
    }
}

pub struct DailyBorrow<'a> {
    fractal: Fractal,
    instabs: &'a Instabs,
    alt: Option<&'a Instabs>,
}

impl<'a> DailyBorrow<'a> {
    fn new(fractal: Fractal, instabs: &'a Instabs) -> Self {
        Self {
            fractal,
            instabs,
            alt: None,
        }
    }

    pub fn fractal(&self) -> &Fractal {
        &self.fractal
    }

    pub fn with_alt(mut self, alt: impl Into<Option<&'a Instabs>>) -> Self {
        self.alt = alt.into();
        self
    }

    pub fn into_owned(self) -> Daily {
        let Self {
            fractal,
            instabs,
            alt,
        } = self;
        Daily {
            fractal,
            instabs: instabs.clone(),
            alt: alt.map(Instabs::clone),
        }
    }
}

pub enum SearchResult {
    Nightmare,
    ShatteredObservatory,
    SunquaPeak,
    Daily(usize),
}

pub trait Searchable {
    fn date(&self) -> &Date;
    fn get(&self, search_result: &SearchResult) -> Option<DailyBorrow<'_>>;
    fn find_fractal(&self, fractal: &Fractal) -> Option<SearchResult>;
}

impl Searchable for Set {
    fn date(&self) -> &Date {
        todo!();
    }

    fn get(&self, search_result: &SearchResult) -> Option<DailyBorrow<'_>> {
        use SearchResult::*;

        match search_result {
            Nightmare => Some(self.nightmare().as_daily()),
            ShatteredObservatory => Some(self.shattered_observatory().as_daily()),
            SunquaPeak => Some(self.sunqua_peak().as_daily()),
            Daily(idx) => self.dailies.get(*idx).and_then(|d| {
                if let DailyOrCm::Daily(daily) = d {
                    Some(daily.borrow())
                } else {
                    None
                }
            }),
        }
    }

    fn find_fractal(&self, fractal: &Fractal) -> Option<SearchResult> {
        use Fractal::*;

        match fractal {
            Nightmare => Some(SearchResult::Nightmare),
            ShatteredObservatory => Some(SearchResult::ShatteredObservatory),
            SunquaPeak => Some(SearchResult::SunquaPeak),
            _ => self
                .dailies
                .iter()
                .filter_map(|d| {
                    if let DailyOrCm::Daily(daily) = d {
                        Some(daily)
                    } else {
                        None
                    }
                })
                .position(|d| d.fractal == *fractal)
                .map(SearchResult::Daily),
        }
    }
}

impl core::fmt::Display for DailyBorrow<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.fractal, self.instabs)?;
        if let Some(alt) = self.alt.as_ref() {
            write!(f, " | {}", alt)?;
        }
        Ok(())
    }
}

impl HasInstabs for DailyBorrow<'_> {
    fn instabs(&self) -> &Instabs {
        self.instabs
    }
}

pub trait Dailylike: HasInstabs {
    fn fractal(&self) -> Fractal;
}

impl Dailylike for Daily {
    fn fractal(&self) -> Fractal {
        self.fractal
    }
}

impl Dailylike for DailyBorrow<'_> {
    fn fractal(&self) -> Fractal {
        self.fractal
    }
}

impl Dailylike for Nightmare {
    fn fractal(&self) -> Fractal {
        Fractal::Nightmare
    }
}

impl Dailylike for ShatteredObservatory {
    fn fractal(&self) -> Fractal {
        Fractal::ShatteredObservatory
    }
}

impl Dailylike for SunquaPeak {
    fn fractal(&self) -> Fractal {
        Fractal::SunquaPeak
    }
}
