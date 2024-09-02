#[derive(Debug, PartialEq)]
pub enum Fu {
    BasePoints,
    BasePointsChitoi,
    ClosedRon,
    Tsumo,
    NonSimpleClosedTriplet,
    SimpleClosedTriplet,
    NonSimpleOpenTriplet,
    SimpleOpenTriplet,
    NonSimpleClosedKan,
    SimpleClosedKan,
    NonSimpleOpenKan,
    SimpleOpenKan,
    Toitsu,
    SingleWait,
}

impl std::fmt::Display for Fu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BasePoints => write!(f, "BasePoints: 20"),
            Self::BasePointsChitoi => write!(f, "BasePoints: 25"),
            Self::ClosedRon => write!(f, "ClosedRon: 10"),
            Self::Tsumo => write!(f, "Tsumo: 2"),
            Self::NonSimpleClosedTriplet => write!(f, "NonSimpleClosedTriplet: 8"),
            Self::SimpleClosedTriplet => write!(f, "ClosedTriplet: 4"),
            Self::NonSimpleOpenTriplet => write!(f, "NonSimpleOpenTriplet: 4"),
            Self::SimpleOpenTriplet => write!(f, "OpenTriplet: 2"),
            Self::NonSimpleClosedKan => write!(f, "NonSimpleClosedKan: 32"),
            Self::SimpleClosedKan => write!(f, "ClosedKan: 16"),
            Self::NonSimpleOpenKan => write!(f, "NonSimpleOpenKan: 16"),
            Self::SimpleOpenKan => write!(f, "OpenKan: 8"),
            Self::Toitsu => write!(f, "Toitsu: 2"),
            Self::SingleWait => write!(f, "SingleWait: 2"),
        }
    }
}

impl Fu {
    /// Get the minipoint value.
    pub fn value(&self) -> u16 {
        match self {
            Self::BasePoints => 20,
            Self::BasePointsChitoi => 25,
            Self::ClosedRon => 10,
            Self::Tsumo => 2,
            Self::NonSimpleClosedTriplet => 8,
            Self::SimpleClosedTriplet => 4,
            Self::NonSimpleOpenTriplet => 4,
            Self::SimpleOpenTriplet => 2,
            Self::NonSimpleClosedKan => 32,
            Self::SimpleClosedKan => 16,
            Self::NonSimpleOpenKan => 16,
            Self::SimpleOpenKan => 8,
            Self::Toitsu => 2,
            Self::SingleWait => 2,
        }
    }
}

/// Sum up all of the fu, rounding to the nearest 10.
pub fn calculate_total_fu_value(fu: &[Fu]) -> u16 {
    ((fu.iter().map(|f| f.value()).sum::<u16>() + 9) / 10) * 10
}
