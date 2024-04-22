#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Fractal {
    Aetherblade,
    AquaticRuins,
    CaptainMaiTrinBoss,
    ChaosIsles,
    Cliffside,
    Deepstone,
    MoltenBoss,
    MoltenFurnace,
    Nightmare,
    ShatteredObservatory,
    SirensReef,
    Snowblind,
    SolidOcean,
    Swampland,
    ThaumanovaReactor,
    TwilightOasis,
    Uncategorized,
    UndergroundFacility,
    UrbanBattleground,
    Volcanic,
    SunquaPeak,
}

impl Fractal {
    pub const fn from_u8(n: u8) -> Option<Self> {
        use Fractal::*;
        match n {
            b'A' => Some(Aetherblade),
            b'B' => Some(AquaticRuins),
            b'C' => Some(CaptainMaiTrinBoss),
            b'D' => Some(ChaosIsles),
            b'E' => Some(Cliffside),
            b'F' => Some(Deepstone),
            b'G' => Some(MoltenBoss),
            b'H' => Some(MoltenFurnace),
            b'I' => Some(Nightmare),
            b'J' => Some(ShatteredObservatory),
            b'K' => Some(SirensReef),
            b'L' => Some(Snowblind),
            b'M' => Some(SolidOcean),
            b'N' => Some(Swampland),
            b'O' => Some(ThaumanovaReactor),
            b'P' => Some(TwilightOasis),
            b'Q' => Some(Uncategorized),
            b'R' => Some(UndergroundFacility),
            b'S' => Some(UrbanBattleground),
            b'T' => Some(Volcanic),
            b'U' => Some(SunquaPeak),

            _ => None,
        }
    }
}

impl core::str::FromStr for Fractal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Fractal::*;

        Ok(match s {
            "aetherblade" => Aetherblade,
            "aquatic" => AquaticRuins,
            "mai" => CaptainMaiTrinBoss,
            "chaos" => ChaosIsles,
            "cliff" => Cliffside,
            "deepstone" => Deepstone,
            "molten boss" => MoltenBoss,
            "furnace" => MoltenFurnace,
            "nightmare" => Nightmare,
            "shattered" => ShatteredObservatory,
            "reef" => SirensReef,
            "snowblind" => Snowblind,
            "swamp" => Swampland,
            "thauma" => ThaumanovaReactor,
            "twilight" => TwilightOasis,
            "uncat" => Uncategorized,
            "dredge" => UndergroundFacility,
            "urban" => UrbanBattleground,
            "volcanic" => Volcanic,
            "sunqua" => SunquaPeak,

            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Fractal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Fractal::*;

        let name = match self {
            Aetherblade => "Aetherblade",
            AquaticRuins => "Aquatic Ruins",
            CaptainMaiTrinBoss => "Captain Mai Trin Boss",
            ChaosIsles => "Chaos Isles",
            Cliffside => "Cliffside",
            Deepstone => "Deepstone",
            MoltenBoss => "Molten Boss",
            MoltenFurnace => "Molten Furnace",
            Nightmare => "Nightmare",
            ShatteredObservatory => "Shattered Observatory",
            SirensReef => "Sirens Reef",
            Snowblind => "Snowblind",
            SolidOcean => "Solid Ocean",
            Swampland => "Swampland",
            ThaumanovaReactor => "Thaumanova Reactor",
            TwilightOasis => "Twilight Oasis",
            Uncategorized => "Uncategorized",
            UndergroundFacility => "Underground Facility",
            UrbanBattleground => "Urban Battleground",
            Volcanic => "Volcanic",
            SunquaPeak => "Sunqua Peak",
        };

        write!(f, "{}", name)
    }
}

// impl core::fmt::Display for Fractal {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         use Fractal::*;

//         match self {
//             Aetherblade => write!(f, "{}", "Aetherblade"),
//             AquaticRuins => write!(f, "{}", "Aquatic Ruins"),
//             CaptainMaiTrinBoss => write!(f, "{}", "Captain Mai Trin Boss"),
//             ChaosIsles => write!(f, "{}", "Chaos Isles"),
//             Cliffside => write!(f, "{}", "Cliffside"),
//             Deepstone => write!(f, "{}", "Deepstone"),
//             MoltenBoss => write!(f, "{}", "Molten Boss"),
//             MoltenFurnace => write!(f, "{}", "Molten Furnace"),
//             Nightmare => write!(f, "{}", "Nightmare"),
//             ShatteredObservatory => write!(f, "{}", "Shattered Observatory"),
//             SirensReef => write!(f, "{}", "Sirens Reef"),
//             Snowblind => write!(f, "{}", "Snowblind"),
//             SolidOcean => write!(f, "{}", "Solid Ocean"),
//             Swampland => write!(f, "{}", "Swampland"),
//             ThaumanovaReactor => write!(f, "{}", "Thaumanova Reactor"),
//             TwilightOasis => write!(f, "{}", "Twilight Oasis"),
//             Uncategorized => write!(f, "{}", "Uncategorized"),
//             UndergroundFacility => write!(f, "{}", "Underground Facility"),
//             UrbanBattleground => write!(f, "{}", "Urban Battleground"),
//             Volcanic => write!(f, "{}", "Volcanic"),
//             SunquaPeak => write!(f, "{}", "Sunqua Peak"),
//         }
//     }
// }
