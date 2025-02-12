//! Contain type for extension of sets

pub mod aug {
    //! Extension type for Augmented
    use serde::{Deserialize, Serialize};

    use crate::{self_upgrade, Card, MoxCount};

    /// Augmented's [`Card`] extensions.
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct AugExt {
        /// Artist credit.
        pub artist: String,
    }

    /// Augmented's [`Costs`][crate::Costs] extensions.
    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    pub struct AugCosts {
        /// Shattered mox cost count.
        pub shattered_count: Option<MoxCount>,
        /// Max energy cell cost.
        pub max: isize,
    }

    self_upgrade!(AugExt, AugCosts);

    /// Type alias for a Augmented [`Card`]
    pub type AugCard = Card<AugCosts, AugExt>;
}

pub mod desc {
    //! Extension type for Descyption

    use serde::{Deserialize, Serialize};

    use crate::{self_upgrade, Card};

    /// Descryption's [`Costs`][crate::Costs] extension.
    #[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
    pub struct DescCosts {
        /// Links cost.
        pub link: isize,
        /// Gold cost.
        pub gold: isize,
    }

    self_upgrade!((), DescCosts);

    /// Type alias for a Augmented [`Card`]
    pub type DescCard = Card<DescCosts, ()>;
}
