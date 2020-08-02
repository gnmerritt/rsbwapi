use bwapi_wrapper::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::c_char;

pub type CoordinateType = BWAPI_CoordinateType_Enum;
pub use bwapi_wrapper::BWAPI_Races_Enum_Enum as Race;
pub type UnitType = BWAPI_UnitTypes_Enum_Enum;
pub type TechType = BWAPI_TechTypes_Enum_Enum;
pub type UnitSizeType = BWAPI_UnitSizeTypes_Enum_Enum;
pub use bwapi_wrapper::BWAPI_UpgradeTypes_Enum_Enum as UpgradeType;
pub type Orders = BWAPI_Orders_Enum_Enum;
pub type WeaponType = BWAPI_WeaponTypes_Enum_Enum;
pub type Flag = BWAPI_Flag_Enum;
pub type UnitCommandType = BWAPI_UnitCommandTypes_Enum_Enum;
pub type Error = BWAPI_Errors_Enum_Enum;
pub type Key = BWAPI_Key;
pub type MouseButton = BWAPI_MouseButton;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum Color {
    /// <summary>The default color for Player 1.</summary>
    Red = 111,

    /// <summary>The default color for Player 2.</summary>
    Blue = 165,

    /// <summary>The default color for Player 3.</summary>
    Teal = 159,

    /// <summary>The default color for Player 4.</summary>
    Purple = 164,

    /// <summary>The default color for Player 5.</summary>
    Orange = 179,

    /// <summary>The default color for Player 6.</summary>
    Brown = 19,

    /// <summary>A bright white. Note that this is lighter than Player 7's white.</summary>
    White = 255,

    /// <summary>The default color for Player 8.</summary>
    Yellow = 135,

    /// <summary>The alternate color for Player 7 on Ice tilesets.</summary>
    Green = 117,

    /// <summary>The default color for Neutral (Player 12).</summary>
    Cyan = 128,

    /// <summary>The color black</summary>
    Black = 0,

    /// <summary>The color grey</summary>
    Grey = 74,
}

#[derive(Debug)]
pub enum TextSize {
    /// <summary>The smallest text size in the game.</summary>
    Small,

    /// <summary>The standard text size, used for most things in the game such as chat messages.</summary>
    Default,

    /// <summary>A larger text size. This size is used for the in-game countdown timer seen in @CTF and @UMS game types.</summary>
    Large,

    /// <summary>The largest text size in the game.</summary>
    Huge,
}

pub trait TypeFrom {
    fn new(i: i32) -> Self;
}

impl<T: FromPrimitive> TypeFrom for T {
    fn new(i: i32) -> Self {
        Self::from_i32(i).unwrap()
    }
}

pub(crate) fn c_str_to_str(i: &[c_char]) -> &str {
    unsafe {
        let i = &*(i as *const [c_char] as *const [u8]);
        CStr::from_bytes_with_nul_unchecked(&i[..=i.iter().position(|&c| c == 0).unwrap()])
            .to_str()
            .unwrap()
    }
}

pub trait WeaponTypeExt {
    fn damage_amount(&self) -> i32 {
        unimplemented!()
    }

    fn damage_bonus(&self) -> i32 {
        unimplemented!()
    }

    fn damage_cooldown(&self) -> i32 {
        unimplemented!()
    }

    fn damage_factor(&self) -> i32 {
        unimplemented!()
    }

    fn min_range(&self) -> i32 {
        unimplemented!()
    }
    fn max_range(&self) -> i32 {
        unimplemented!()
    }

    fn upgrade_type(&self) -> UpgradeType {
        unimplemented!()
    }
}

impl WeaponTypeExt for WeaponType {}
