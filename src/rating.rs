use crate::{Fractal, HasInstabs, Instability};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rating {
    Unplayable,
    Bad,
    Playable,
    Good,
    Perfect,
}

impl core::fmt::Display for Rating {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Rating::*;

        let name = match self {
            Perfect => "Perfect",
            Good => "Good",
            Playable => "Playable",
            Bad => "Bad",
            Unplayable => "Unplayable",
        };

        write!(f, "{}", name)
    }
}

impl core::str::FromStr for Rating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rating::*;

        Ok(match s {
            "perfect" => Perfect,
            "good" => Good,
            "playable" => Playable,
            "bad" => Bad,
            "unplayable" => Unplayable,
            _ => return Err(()),
        })
    }
}

impl core::ops::Add for Rating {
    type Output = Rating;

    fn add(self, rhs: Self) -> Self::Output {
        use core::cmp::Ordering;
        use Rating::*;

        if matches!((self, rhs), (Perfect, Good) | (Good, Perfect)) {
            return Perfect;
        }

        match self.cmp(&rhs) {
            Ordering::Less => self,
            Ordering::Equal => rhs,
            Ordering::Greater => rhs,
        }
    }
}

impl core::iter::Sum for Rating {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Rating::Good, core::ops::Add::add)
    }
}

pub trait Rateable: HasInstabs {
    fn rate(&self, rateable: &impl Rater) -> Rating {
        self.instabs().iter().map(|i| rateable.rate_one(i)).sum()
    }
}

impl<T> Rateable for T where T: HasInstabs {}

pub trait Rater {
    fn rate_one(&self, instab: &Instability) -> Rating;
}

impl Instability {
    fn rate(&self) -> Rating {
        use Instability::*;
        use Rating::*;

        match self {
            AdrenalineRush => Good,
            Afflicted => Good,
            BoonOverload => Perfect,
            FluxBomb => Unplayable,
            FractalVindicators => Good,
            Frailty => Good,
            Hamstrung => Playable,
            LastLaugh => Bad,
            MistsConvergence => Playable,
            NoPainNoGain => Unplayable,
            Outflanked => Good,
            SocialAwkwardness => Unplayable,
            StickTogether => Good,
            SugarRush => Good,
            ToxicTrail => Bad,
            Vengeance => Bad,
            WeBleedFire => Unplayable,
            ToxicSickness => Playable,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Boss {
    Mama,
    Siax,
    Ensolyss,
    Skorvald,
    Artsariiv,
    Arkk,
    Ai,
}

impl Boss {
    pub fn home(&self) -> Fractal {
        use Boss::*;
        use Fractal::*;

        match &self {
            Mama => Nightmare,
            Siax => Nightmare,
            Ensolyss => Nightmare,
            Skorvald => ShatteredObservatory,
            Artsariiv => ShatteredObservatory,
            Arkk => ShatteredObservatory,
            Ai => SunquaPeak,
        }
    }
}

impl core::str::FromStr for Boss {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Boss::*;

        Ok(match s {
            "mama" => Mama,
            "siax" => Siax,
            "enso" => Ensolyss,
            "skorv" => Skorvald,
            "arts" => Artsariiv,
            "arkk" => Arkk,
            "ai" => Ai,
            _ => return Err(()),
        })
    }
}

impl Rater for Boss {
    fn rate_one(&self, instab: &Instability) -> Rating {
        use Boss::*;
        use Instability::*;
        use Rating::*;

        match self {
            Mama => match instab {
                LastLaugh => return Unplayable,
                _ => {}
            },
            Siax => match instab {
                // LastLaugh => return Bad,
                _ => {}
            },
            Ensolyss => match instab {
                LastLaugh => return Good,
                _ => {}
            },
            Skorvald => match instab {
                LastLaugh => return Good,
                _ => {}
            },
            Artsariiv => match instab {
                LastLaugh => return Good,
                _ => {}
            },
            Arkk => match instab {
                Hamstrung => return Bad,
                // LastLaugh => return Bad,
                _ => {}
            },
            Ai => match instab {
                _ => {}
            },
        }
        instab.rate()
    }
}

impl Rater for Fractal {
    fn rate_one(&self, instab: &Instability) -> Rating {
        use Boss::*;
        use Fractal::*;

        match self {
            Nightmare => [Mama, Siax, Ensolyss]
                .iter()
                .map(|b| b.rate_one(instab))
                .sum(),
            ShatteredObservatory => [Skorvald, Artsariiv, Arkk]
                .iter()
                .map(|b| b.rate_one(instab))
                .sum(),
            _ => instab.rate(),
        }
    }
}

impl Rater for crate::set::Nightmare {
    fn rate_one(&self, instab: &Instability) -> Rating {
        use Boss::*;

        [Mama, Siax, Ensolyss]
            .iter()
            .map(|b| b.rate_one(instab))
            .sum()
    }
}

impl Rater for crate::set::ShatteredObservatory {
    fn rate_one(&self, instab: &Instability) -> Rating {
        use Boss::*;

        [Skorvald, Artsariiv, Arkk]
            .iter()
            .map(|b| b.rate_one(instab))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_addition() {
        use Rating::*;
        assert_eq!(Unplayable + Good, Unplayable);
        assert_eq!(Good + Good, Good);
        assert_eq!(Bad + Good, Bad);
        assert_eq!(Playable + Good, Playable);
    }

    #[test]
    fn rating_summation() {
        use crate::set::Instabs;
        use Boss::*;
        use Instability::*;
        use Rating::*;

        let sets = [
            (
                Siax,
                Instabs::new(ToxicTrail, BoonOverload, SocialAwkwardness),
                Unplayable,
            ),
            (
                Arkk,
                Instabs::new(BoonOverload, Afflicted, Frailty),
                Perfect,
            ),
            (Arkk, Instabs::new(BoonOverload, Afflicted, Vengeance), Bad),
        ];

        for (boss, instabs, expected) in &sets {
            assert_eq!(instabs.rate(boss), *expected);
        }
    }
}
