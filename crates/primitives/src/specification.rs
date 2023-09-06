#![allow(non_camel_case_types)]

/// SpecId and their activation block
/// Information was obtained from: <https://github.com/ethereum/execution-specs>
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, enumn::N)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecId {
    FRONTIER = 0,         // Frontier	            0
    FRONTIER_THAWING = 1, // Frontier Thawing       200000
    HOMESTEAD = 2,        // Homestead	            1150000
    DAO_FORK = 3,         // DAO Fork	            1920000
    TANGERINE = 4,        // Tangerine Whistle	    2463000
    SPURIOUS_DRAGON = 5,  // Spurious Dragon        2675000
    BYZANTIUM = 6,        // Byzantium	            4370000
    CONSTANTINOPLE = 7,   // Constantinople         7280000 is overwritten with PETERSBURG
    PETERSBURG = 8,       // Petersburg             7280000
    ISTANBUL = 9,         // Istanbul	            9069000
    MUIR_GLACIER = 10,    // Muir Glacier	        9200000
    BERLIN = 11,          // Berlin	                12244000
    LONDON = 12,          // London	                12965000
    ARROW_GLACIER = 13,   // Arrow Glacier	        13773000
    GRAY_GLACIER = 14,    // Gray Glacier	        15050000
    MERGE = 15,           // Paris/Merge	        TBD (Depends on difficulty)
    SHANGHAI = 16,
    CANCUN = 17,
    LATEST = 18,
    #[cfg(feature = "optimism")]
    BEDROCK = 128,
    #[cfg(feature = "optimism")]
    REGOLITH = 129,
}

impl SpecId {
    pub fn try_from_u8(spec_id: u8) -> Option<Self> {
        Self::n(spec_id)
    }
}

pub use SpecId::*;

impl From<&str> for SpecId {
    fn from(name: &str) -> Self {
        match name {
            "Frontier" => SpecId::FRONTIER,
            "Homestead" => SpecId::HOMESTEAD,
            "Tangerine" => SpecId::TANGERINE,
            "Spurious" => SpecId::SPURIOUS_DRAGON,
            "Byzantium" => SpecId::BYZANTIUM,
            "Constantinople" => SpecId::CONSTANTINOPLE,
            "Petersburg" => SpecId::PETERSBURG,
            "Istanbul" => SpecId::ISTANBUL,
            "MuirGlacier" => SpecId::MUIR_GLACIER,
            "Berlin" => SpecId::BERLIN,
            "London" => SpecId::LONDON,
            "Merge" => SpecId::MERGE,
            "Shanghai" => SpecId::SHANGHAI,
            "Cancun" => SpecId::CANCUN,
            #[cfg(feature = "optimism")]
            "Bedrock" => SpecId::BEDROCK,
            #[cfg(feature = "optimism")]
            "Regolith" => SpecId::REGOLITH,
            _ => SpecId::LATEST,
        }
    }
}

impl SpecId {
    #[inline]
    pub const fn enabled(our: SpecId, other: SpecId) -> bool {
        our as u8 >= other as u8
    }
}

pub trait Spec: Sized {
    const SPEC_ID: SpecId;

    #[inline(always)]
    fn enabled(spec_id: SpecId) -> bool {
        // If the Spec is Bedrock or Regolith, and the input is not Bedrock or Regolith,
        // then no hardforks should be enabled after the merge.
        let is_self_optimism =
            Self::SPEC_ID == SpecId::BEDROCK || Self::SPEC_ID == SpecId::REGOLITH;
        let input_not_optimism = spec_id != SpecId::BEDROCK && spec_id != SpecId::REGOLITH;
        let after_merge = spec_id > SpecId::MERGE;

        // Optimism's Bedrock and Regolith hardforks implement changes on top of the Merge
        // hardfork. This function is modified to preserve the original behavior of the
        // spec IDs without having to put hardforks past Merge under
        // `#[cfg(not(feature = "optimism"))]`.
        #[cfg(feature = "optimism")]
        if is_self_optimism && input_not_optimism && after_merge {
            return false;
        }

        Self::SPEC_ID as u8 >= spec_id as u8
    }
}

macro_rules! spec {
    ($spec_id:tt, $spec_name:tt) => {
        pub struct $spec_name;

        impl Spec for $spec_name {
            //specification id
            const SPEC_ID: SpecId = $spec_id;
        }
    };
}

spec!(FRONTIER, FrontierSpec);
// FRONTIER_THAWING no EVM spec change
spec!(HOMESTEAD, HomesteadSpec);
// DAO_FORK no EVM spec change
spec!(TANGERINE, TangerineSpec);
spec!(SPURIOUS_DRAGON, SpuriousDragonSpec);
spec!(BYZANTIUM, ByzantiumSpec);
// CONSTANTINOPLE was overridden with PETERSBURG
spec!(PETERSBURG, PetersburgSpec);
spec!(ISTANBUL, IstanbulSpec);
// MUIR_GLACIER no EVM spec change
spec!(BERLIN, BerlinSpec);
spec!(LONDON, LondonSpec);
// ARROW_GLACIER no EVM spec change
// GRAY_GLACIER no EVM spec change
spec!(MERGE, MergeSpec);
spec!(SHANGHAI, ShanghaiSpec);
spec!(CANCUN, CancunSpec);
spec!(LATEST, LatestSpec);

// Optimism Hardforks
#[cfg(feature = "optimism")]
spec!(BEDROCK, BedrockSpec);
#[cfg(feature = "optimism")]
spec!(REGOLITH, RegolithSpec);

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "optimism")]
    #[test]
    fn test_bedrock_post_merge_hardforks() {
        assert!(BedrockSpec::enabled(SpecId::MERGE));
        assert!(!BedrockSpec::enabled(SpecId::SHANGHAI));
        assert!(!BedrockSpec::enabled(SpecId::CANCUN));
        assert!(!BedrockSpec::enabled(SpecId::LATEST));
        assert!(BedrockSpec::enabled(SpecId::BEDROCK));
        assert!(!BedrockSpec::enabled(SpecId::REGOLITH));
    }

    #[cfg(feature = "optimism")]
    #[test]
    fn test_regolith_post_merge_hardforks() {
        assert!(RegolithSpec::enabled(SpecId::MERGE));
        assert!(!RegolithSpec::enabled(SpecId::SHANGHAI));
        assert!(!RegolithSpec::enabled(SpecId::CANCUN));
        assert!(!RegolithSpec::enabled(SpecId::LATEST));
        assert!(RegolithSpec::enabled(SpecId::BEDROCK));
        assert!(RegolithSpec::enabled(SpecId::REGOLITH));
    }
}
