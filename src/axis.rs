#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    Market,
    Liberty,
    Progressivism,
    Transhumanism,
    Democracy,
    Delusionality,
    Revolutionism,
}

impl Axis {
    pub const ALL: [Self; 7] = [
        Self::Market,
        Self::Liberty,
        Self::Progressivism,
        Self::Transhumanism,
        Self::Democracy,
        Self::Delusionality,
        Self::Revolutionism,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Market => 0,
            Self::Liberty => 1,
            Self::Progressivism => 2,
            Self::Transhumanism => 3,
            Self::Democracy => 4,
            Self::Delusionality => 5,
            Self::Revolutionism => 6,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Market => "Market",
            Self::Liberty => "Liberty",
            Self::Progressivism => "Progressivism",
            Self::Transhumanism => "Transhumanism",
            Self::Democracy => "Democracy",
            Self::Delusionality => "Delusionality",
            Self::Revolutionism => "Revolutionism",
        }
    }

    pub const fn positive_pole(self) -> &'static str {
        match self {
            Self::Market => "free-market",
            Self::Liberty => "individual liberty",
            Self::Progressivism => "progressive",
            Self::Transhumanism => "enhancement",
            Self::Democracy => "democracy",
            Self::Delusionality => "delusionality",
            Self::Revolutionism => "revolution",
        }
    }

    pub const fn negative_pole(self) -> &'static str {
        match self {
            Self::Market => "state-directed",
            Self::Liberty => "authority",
            Self::Progressivism => "traditionalist",
            Self::Transhumanism => "naturalist",
            Self::Democracy => "autocracy",
            Self::Delusionality => "groundedness",
            Self::Revolutionism => "reformism",
        }
    }
}
