use crate::player::Player;


use crate::*;
use bwapi_wrapper::*;

#[derive(Clone, Copy)]
pub struct Unit<'a> {
    pub id: usize,
    game: &'a Game,
    data: &'a BWAPI_UnitData,
}

impl<'a> Unit<'a> {
    pub(crate) fn new(id: usize, game: &'a Game, data: &'a BWAPI_UnitData) -> Self {
        Unit { id, game, data }
    }

    pub fn get_type(&self) -> UnitType {
        types::unit_type_from(self.data.type_)
    }

    pub fn exists(&self) -> bool {
        self.data.exists
    }

    pub fn is_visible(&self, player: &Player) -> bool {
        self.data.isVisible[player.id as usize]
    }

    pub fn get_player(&self) -> Option<Player<'a>> {
        self.game.get_player(self.data.player as usize)
    }

    pub fn gather(&self, target: &Unit) -> UnitCommand {
        UnitCommand {
            targetIndex: target.id as i32,
            ..self.command_type(BWAPI_UnitCommandTypes_Enum_Enum::Gather)
        }
    }

    pub fn attack(&self, target: &Unit) -> UnitCommand {
        UnitCommand {
            targetIndex: target.id as i32,
            ..self.command_type(BWAPI_UnitCommandTypes_Enum_Enum::Attack_Unit)
        }
    }

    fn command_type(&self, cmd: BWAPI_UnitCommandTypes_Enum_Enum) -> UnitCommand {
        UnitCommand {
            type_: BWAPI_UnitCommandType { _base: cmd as u32 },
            extra: 0,
            x: 0,
            y: 0,
            unitIndex: self.id as i32,
            targetIndex: -1,
        }
    }
}

impl<'a> PartialEq for Unit<'a> {
    fn eq(&self, other: &Unit) -> bool {
        self.id == other.id
    }
}

pub type UnitCommand = BWAPIC_UnitCommand;
