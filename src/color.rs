use crate::{
    rating::Rateable,
    set::{Dailylike, Instabs},
    Fractal, Instability, Rater, Rating, Set,
};

pub trait CustomColors {
    fn map(rating: Rating) -> Color;
}

pub struct DefaultColors {}
impl CustomColors for DefaultColors {
    fn map(rating: Rating) -> Color {
        match rating {
            Rating::Perfect => Color::Yellow,
            Rating::Good => Color::Blue,
            Rating::Playable => Color::Normal,
            Rating::Bad => Color::BrightBlack,
            Rating::Unplayable => Color::Red,
        }
    }
}

pub struct StandaloneInstabColors {}
impl CustomColors for StandaloneInstabColors {
    fn map(rating: Rating) -> Color {
        match rating {
            Rating::Perfect => Color::Yellow,
            Rating::Unplayable => Color::BrightBlack,
            _ => Color::Normal,
        }
    }
}

pub trait ColorRating<T>
where
    T: CustomColors,
{
    fn color_by_rating(&self, rating: Rating) -> WithColor<'_, Self>
    where
        Self: Colored,
    {
        self.colored(T::map(rating))
    }
}

impl<T, U> ColorRating<U> for T
where
    T: Colored,
    U: CustomColors,
{
}

pub trait DailyColored<T>
where
    T: Rater,
{
    fn daily_colored(&self) -> ColoredByRater<'_, Self, T>;
}

impl<T> DailyColored<Fractal> for T
where
    T: Dailylike,
{
    fn daily_colored(&self) -> ColoredByRater<'_, Self, Fractal> {
        ColoredByRater {
            inner: self,
            rater: self.fractal().clone(),
        }
    }
}

pub struct ColoredByRater<'a, T, U>
where
    T: ?Sized,
    U: Rater,
{
    inner: &'a T,
    rater: U,
}

pub trait ColorByRater<T>
where
    T: Rater,
{
    fn color_by_rater(&self, rater: T) -> ColoredByRater<'_, Self, T> {
        ColoredByRater { inner: self, rater }
    }
}

impl<T> ColorByRater<T> for Instability where T: Rater {}
impl<T> ColorByRater<T> for Instabs where T: Rater {}
impl<T, U> ColorByRater<T> for U
where
    T: Rater,
    U: Dailylike,
{
}

pub trait SetColored {
    fn colored(&self) -> ColoredSet;
}

pub struct ColoredSet<'a>(&'a Set);

impl SetColored for Set {
    fn colored(&self) -> ColoredSet<'_> {
        ColoredSet(self)
    }
}

impl core::fmt::Display for ColoredSet<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "{}\n{}\n{}",
            self.0.nightmare().daily_colored(),
            self.0.shattered_observatory().daily_colored(),
            self.0.sunqua_peak().daily_colored(),
        )?;
        for daily in self.0.dailies_iter() {
            writeln!(f, "{}", daily.daily_colored())?;
        }

        Ok(())
    }
}

impl<T, U> core::fmt::Display for ColoredByRater<'_, U, T>
where
    T: Rater + Copy,
    U: Dailylike,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rating = self.inner.rate(&self.rater);
        write!(
            f,
            "{}: {}",
            ColorRating::<DefaultColors>::color_by_rating(&self.inner.fractal(), rating),
            self.inner.instabs().color_by_rater(self.rater)
        )
    }
}

impl<T> core::fmt::Display for ColoredByRater<'_, Instabs, T>
where
    T: Rater + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let array = self.inner.raw();
        write!(
            f,
            "{}; {}; {}",
            array[0].color_by_rater(self.rater),
            array[1].color_by_rater(self.rater),
            array[2].color_by_rater(self.rater)
        )
    }
}

impl<T> core::fmt::Display for ColoredByRater<'_, Instability, T>
where
    T: Rater,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rating = self.rater.rate_one(self.inner);
        let thing = ColorRating::<StandaloneInstabColors>::color_by_rating(&self.inner, rating);
        write!(f, "{}", thing)
    }
}

pub enum Color {
    Normal,
    Bold,
    Faint,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    fn code(&self) -> u8 {
        use Color::*;

        match self {
            Normal => 0,
            Bold => 1,
            Faint => 2,
            Black => 30,
            Red => 31,
            Green => 32,
            Yellow => 33,
            Blue => 34,
            Magenta => 35,
            Cyan => 36,
            White => 37,
            BrightBlack => 90,
            BrightRed => 91,
            BrightGreen => 92,
            BrightYellow => 93,
            BrightBlue => 94,
            BrightMagenta => 95,
            BrightCyan => 96,
            BrightWhite => 97,
        }
    }
}

pub struct WithColor<'a, T: ?Sized> {
    inner: &'a T,
    color: Color,
}

pub trait Colored {
    fn colored(&self, color: Color) -> WithColor<'_, Self> {
        WithColor { inner: self, color }
    }
}

impl<T> Colored for T {}

impl<T> core::fmt::Display for WithColor<'_, T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}m{}[0m", self.color.code(), self.inner)
    }
}
