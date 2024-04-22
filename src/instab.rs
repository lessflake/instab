#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instability {
    AdrenalineRush,
    Afflicted,
    BoonOverload,
    FluxBomb,
    FractalVindicators,
    Frailty,
    Hamstrung,
    LastLaugh,
    MistsConvergence,
    NoPainNoGain,
    Outflanked,
    SocialAwkwardness,
    StickTogether,
    SugarRush,
    ToxicTrail,
    Vengeance,
    WeBleedFire,
    ToxicSickness,
}

impl Instability {
    pub const fn from_u8(n: u8) -> Option<Self> {
        use Instability::*;
        match n {
            1 => Some(AdrenalineRush),
            2 => Some(Afflicted),
            3 => Some(BoonOverload),
            4 => Some(FluxBomb),
            5 => Some(FractalVindicators),
            6 => Some(Frailty),
            7 => Some(Hamstrung),
            8 => Some(LastLaugh),
            9 => Some(MistsConvergence),
            10 => Some(NoPainNoGain),
            11 => Some(Outflanked),
            12 => Some(SocialAwkwardness),
            13 => Some(StickTogether),
            14 => Some(SugarRush),
            15 => Some(ToxicTrail),
            16 => Some(Vengeance),
            17 => Some(WeBleedFire),
            18 => Some(ToxicSickness),
            _ => None,
        }
    }
}

impl core::fmt::Display for Instability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Instability::*;

        let name = match self {
            AdrenalineRush => "Adrenaline Rush",
            Afflicted => "Afflicted",
            BoonOverload => "Boon Overload",
            FluxBomb => "Flux Bomb",
            FractalVindicators => "Fractal Vindicators",
            Frailty => "Frailty",
            Hamstrung => "Hamstrung",
            LastLaugh => "Last Laugh",
            MistsConvergence => "Mists Convergence",
            NoPainNoGain => "No Pain, No Gain",
            Outflanked => "Outflanked",
            SocialAwkwardness => "Social Awkwardness",
            StickTogether => "Stick Together",
            SugarRush => "Sugar Rush",
            ToxicTrail => "Toxic Trail",
            Vengeance => "Vengeance",
            WeBleedFire => "We Bleed Fire",
            ToxicSickness => "Toxic Sickness",
        };

        write!(f, "{}", name)
    }
}
