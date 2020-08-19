use crate::prelude::*;
pub(crate) struct UpgradeTypeData {
pub(crate) what_uses: &'static [UnitType],
pub(crate) gas_price_factor: i32,
pub(crate) max_repeats: i32,
pub(crate) race: Race,
pub(crate) mineral_price_factor: i32,
pub(crate) what_upgrades: UnitType,
pub(crate) upgrade_time_factor: i32,
pub(crate) name: &'static str
}
pub(crate) static UPGRADE_TYPE_DATA : [UpgradeTypeData; 53] = [
UpgradeTypeData {what_uses: &[UnitType::Terran_Marine, UnitType::Terran_Ghost, UnitType::Terran_SCV, UnitType::Hero_Gui_Montag, UnitType::Terran_Civilian, UnitType::Hero_Sarah_Kerrigan, UnitType::Hero_Jim_Raynor_Marine, UnitType::Terran_Firebat, UnitType::Terran_Medic, UnitType::Hero_Samir_Duran, UnitType::Hero_Alexei_Stukov],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 75,
what_upgrades: UnitType::Terran_Engineering_Bay,
upgrade_time_factor: 480,
name: "Terran_Infantry_Armor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Vulture, UnitType::Terran_Goliath, UnitType::Terran_Siege_Tank_Tank_Mode, UnitType::Hero_Alan_Schezar, UnitType::Hero_Jim_Raynor_Vulture, UnitType::Hero_Edmund_Duke_Tank_Mode, UnitType::Hero_Edmund_Duke_Siege_Mode, UnitType::Terran_Siege_Tank_Siege_Mode],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 75,
what_upgrades: UnitType::Terran_Armory,
upgrade_time_factor: 480,
name: "Terran_Vehicle_Plating"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Wraith, UnitType::Terran_Science_Vessel, UnitType::Terran_Dropship, UnitType::Terran_Battlecruiser, UnitType::Hero_Tom_Kazansky, UnitType::Hero_Magellan, UnitType::Hero_Arcturus_Mengsk, UnitType::Hero_Hyperion, UnitType::Hero_Norad_II, UnitType::Terran_Valkyrie, UnitType::Hero_Gerard_DuGalle],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 75,
what_upgrades: UnitType::Terran_Armory,
upgrade_time_factor: 480,
name: "Terran_Ship_Plating"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Larva, UnitType::Zerg_Egg, UnitType::Zerg_Zergling, UnitType::Zerg_Hydralisk, UnitType::Zerg_Ultralisk, UnitType::Zerg_Broodling, UnitType::Zerg_Drone, UnitType::Zerg_Defiler, UnitType::Hero_Torrasque, UnitType::Zerg_Infested_Terran, UnitType::Hero_Infested_Kerrigan, UnitType::Hero_Unclean_One, UnitType::Hero_Hunter_Killer, UnitType::Hero_Devouring_One, UnitType::Zerg_Cocoon, UnitType::Zerg_Lurker_Egg, UnitType::Zerg_Lurker, UnitType::Hero_Infested_Duran],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Zerg,
mineral_price_factor: 75,
what_upgrades: UnitType::Zerg_Evolution_Chamber,
upgrade_time_factor: 480,
name: "Zerg_Carapace"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Overlord, UnitType::Zerg_Mutalisk, UnitType::Zerg_Guardian, UnitType::Zerg_Queen, UnitType::Zerg_Scourge, UnitType::Hero_Matriarch, UnitType::Hero_Kukulza_Mutalisk, UnitType::Hero_Kukulza_Guardian, UnitType::Hero_Yggdrasill, UnitType::Zerg_Devourer],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Zerg,
mineral_price_factor: 75,
what_upgrades: UnitType::Zerg_Spire,
upgrade_time_factor: 480,
name: "Zerg_Flyer_Carapace"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Dark_Templar, UnitType::Protoss_Dark_Archon, UnitType::Protoss_Probe, UnitType::Protoss_Zealot, UnitType::Protoss_Dragoon, UnitType::Protoss_High_Templar, UnitType::Protoss_Archon, UnitType::Hero_Dark_Templar, UnitType::Hero_Zeratul, UnitType::Hero_Tassadar_Zeratul_Archon, UnitType::Hero_Fenix_Zealot, UnitType::Hero_Fenix_Dragoon, UnitType::Hero_Tassadar, UnitType::Hero_Warbringer, UnitType::Protoss_Reaver, UnitType::Hero_Aldaris],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Protoss,
mineral_price_factor: 75,
what_upgrades: UnitType::Protoss_Forge,
upgrade_time_factor: 480,
name: "Protoss_Ground_Armor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Corsair, UnitType::Protoss_Shuttle, UnitType::Protoss_Scout, UnitType::Protoss_Arbiter, UnitType::Protoss_Carrier, UnitType::Protoss_Interceptor, UnitType::Hero_Mojo, UnitType::Hero_Gantrithor, UnitType::Protoss_Observer, UnitType::Hero_Danimoth, UnitType::Hero_Artanis, UnitType::Hero_Raszagal],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Protoss,
mineral_price_factor: 75,
what_upgrades: UnitType::Protoss_Cybernetics_Core,
upgrade_time_factor: 480,
name: "Protoss_Air_Armor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Marine, UnitType::Hero_Jim_Raynor_Marine, UnitType::Terran_Ghost, UnitType::Hero_Sarah_Kerrigan, UnitType::Terran_Firebat, UnitType::Hero_Gui_Montag, UnitType::Special_Wall_Flame_Trap, UnitType::Special_Right_Wall_Flame_Trap, UnitType::Hero_Samir_Duran, UnitType::Hero_Alexei_Stukov, UnitType::Hero_Infested_Duran],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 75,
what_upgrades: UnitType::Terran_Engineering_Bay,
upgrade_time_factor: 480,
name: "Terran_Infantry_Weapons"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Vulture, UnitType::Hero_Jim_Raynor_Vulture, UnitType::Terran_Goliath, UnitType::Hero_Alan_Schezar, UnitType::Terran_Siege_Tank_Tank_Mode, UnitType::Terran_Siege_Tank_Siege_Mode, UnitType::Hero_Edmund_Duke_Tank_Mode, UnitType::Hero_Edmund_Duke_Siege_Mode, UnitType::Special_Floor_Missile_Trap, UnitType::Special_Floor_Gun_Trap, UnitType::Special_Wall_Missile_Trap, UnitType::Special_Right_Wall_Missile_Trap],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 75,
what_upgrades: UnitType::Terran_Armory,
upgrade_time_factor: 480,
name: "Terran_Vehicle_Weapons"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Wraith, UnitType::Hero_Tom_Kazansky, UnitType::Terran_Battlecruiser, UnitType::Hero_Hyperion, UnitType::Hero_Norad_II, UnitType::Hero_Arcturus_Mengsk, UnitType::Hero_Gerard_DuGalle, UnitType::Terran_Valkyrie],
gas_price_factor: 50,
max_repeats: 3,
race: Race::Terran,
mineral_price_factor: 50,
what_upgrades: UnitType::Terran_Armory,
upgrade_time_factor: 480,
name: "Terran_Ship_Weapons"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Zergling, UnitType::Hero_Devouring_One, UnitType::Hero_Infested_Kerrigan, UnitType::Zerg_Ultralisk, UnitType::Hero_Torrasque, UnitType::Zerg_Broodling],
gas_price_factor: 50,
max_repeats: 3,
race: Race::Zerg,
mineral_price_factor: 50,
what_upgrades: UnitType::Zerg_Evolution_Chamber,
upgrade_time_factor: 480,
name: "Zerg_Melee_Attacks"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Hydralisk, UnitType::Hero_Hunter_Killer, UnitType::Zerg_Lurker],
gas_price_factor: 50,
max_repeats: 3,
race: Race::Zerg,
mineral_price_factor: 50,
what_upgrades: UnitType::Zerg_Evolution_Chamber,
upgrade_time_factor: 480,
name: "Zerg_Missile_Attacks"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Mutalisk, UnitType::Hero_Kukulza_Mutalisk, UnitType::Hero_Kukulza_Guardian, UnitType::Zerg_Guardian, UnitType::Zerg_Devourer],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Zerg,
mineral_price_factor: 75,
what_upgrades: UnitType::Zerg_Spire,
upgrade_time_factor: 480,
name: "Zerg_Flyer_Attacks"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Zealot, UnitType::Hero_Fenix_Zealot, UnitType::Protoss_Dragoon, UnitType::Hero_Fenix_Dragoon, UnitType::Hero_Tassadar, UnitType::Hero_Aldaris, UnitType::Protoss_Archon, UnitType::Hero_Tassadar_Zeratul_Archon, UnitType::Hero_Dark_Templar, UnitType::Hero_Zeratul, UnitType::Protoss_Dark_Templar],
gas_price_factor: 50,
max_repeats: 3,
race: Race::Protoss,
mineral_price_factor: 50,
what_upgrades: UnitType::Protoss_Forge,
upgrade_time_factor: 480,
name: "Protoss_Ground_Weapons"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Scout, UnitType::Hero_Mojo, UnitType::Protoss_Arbiter, UnitType::Hero_Danimoth, UnitType::Protoss_Interceptor, UnitType::Protoss_Carrier, UnitType::Protoss_Corsair, UnitType::Hero_Artanis],
gas_price_factor: 75,
max_repeats: 3,
race: Race::Protoss,
mineral_price_factor: 75,
what_upgrades: UnitType::Protoss_Cybernetics_Core,
upgrade_time_factor: 480,
name: "Protoss_Air_Weapons"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Corsair, UnitType::Protoss_Dark_Templar, UnitType::Protoss_Dark_Archon, UnitType::Protoss_Probe, UnitType::Protoss_Zealot, UnitType::Protoss_Dragoon, UnitType::Protoss_High_Templar, UnitType::Protoss_Archon, UnitType::Protoss_Shuttle, UnitType::Protoss_Scout, UnitType::Protoss_Arbiter, UnitType::Protoss_Carrier, UnitType::Protoss_Interceptor, UnitType::Hero_Dark_Templar, UnitType::Hero_Zeratul, UnitType::Hero_Tassadar_Zeratul_Archon, UnitType::Hero_Fenix_Zealot, UnitType::Hero_Fenix_Dragoon, UnitType::Hero_Tassadar, UnitType::Hero_Mojo, UnitType::Hero_Warbringer, UnitType::Hero_Gantrithor, UnitType::Protoss_Reaver, UnitType::Protoss_Observer, UnitType::Hero_Danimoth, UnitType::Hero_Aldaris, UnitType::Hero_Artanis, UnitType::Hero_Raszagal],
gas_price_factor: 100,
max_repeats: 3,
race: Race::Protoss,
mineral_price_factor: 100,
what_upgrades: UnitType::Protoss_Forge,
upgrade_time_factor: 480,
name: "Protoss_Plasma_Shields"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Marine],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Academy,
upgrade_time_factor: 0,
name: "U_238_Shells"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Vulture],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Machine_Shop,
upgrade_time_factor: 0,
name: "Ion_Thrusters"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Science_Vessel],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Science_Facility,
upgrade_time_factor: 0,
name: "Titan_Reactor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Ghost],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Covert_Ops,
upgrade_time_factor: 0,
name: "Ocular_Implants"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Ghost],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Covert_Ops,
upgrade_time_factor: 0,
name: "Moebius_Reactor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Wraith],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Control_Tower,
upgrade_time_factor: 0,
name: "Apollo_Reactor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Battlecruiser],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Physics_Lab,
upgrade_time_factor: 0,
name: "Colossus_Reactor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Overlord],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Lair,
upgrade_time_factor: 0,
name: "Ventral_Sacs"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Overlord],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Lair,
upgrade_time_factor: 0,
name: "Antennae"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Overlord],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Lair,
upgrade_time_factor: 0,
name: "Pneumatized_Carapace"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Zergling],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Spawning_Pool,
upgrade_time_factor: 0,
name: "Metabolic_Boost"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Zergling],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Spawning_Pool,
upgrade_time_factor: 0,
name: "Adrenal_Glands"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Hydralisk],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Hydralisk_Den,
upgrade_time_factor: 0,
name: "Muscular_Augments"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Hydralisk],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Hydralisk_Den,
upgrade_time_factor: 0,
name: "Grooved_Spines"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Queen],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Queens_Nest,
upgrade_time_factor: 0,
name: "Gamete_Meiosis"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Defiler],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Defiler_Mound,
upgrade_time_factor: 0,
name: "Metasynaptic_Node"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Dragoon],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Cybernetics_Core,
upgrade_time_factor: 0,
name: "Singularity_Charge"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Zealot],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Citadel_of_Adun,
upgrade_time_factor: 0,
name: "Leg_Enhancements"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Reaver],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Robotics_Support_Bay,
upgrade_time_factor: 0,
name: "Scarab_Damage"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Reaver],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Robotics_Support_Bay,
upgrade_time_factor: 0,
name: "Reaver_Capacity"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Shuttle],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Robotics_Support_Bay,
upgrade_time_factor: 0,
name: "Gravitic_Drive"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Observer],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Observatory,
upgrade_time_factor: 0,
name: "Sensor_Array"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Observer],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Observatory,
upgrade_time_factor: 0,
name: "Gravitic_Boosters"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_High_Templar],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Templar_Archives,
upgrade_time_factor: 0,
name: "Khaydarin_Amulet"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Scout],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Fleet_Beacon,
upgrade_time_factor: 0,
name: "Apial_Sensors"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Scout],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Fleet_Beacon,
upgrade_time_factor: 0,
name: "Gravitic_Thrusters"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Carrier],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Fleet_Beacon,
upgrade_time_factor: 0,
name: "Carrier_Capacity"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Arbiter],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Arbiter_Tribunal,
upgrade_time_factor: 0,
name: "Khaydarin_Core"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Corsair],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Fleet_Beacon,
upgrade_time_factor: 0,
name: "Argus_Jewel"
}, 
UpgradeTypeData {what_uses: &[UnitType::Protoss_Dark_Archon],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Protoss,
mineral_price_factor: 0,
what_upgrades: UnitType::Protoss_Templar_Archives,
upgrade_time_factor: 0,
name: "Argus_Talisman"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Medic],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Academy,
upgrade_time_factor: 0,
name: "Caduceus_Reactor"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Ultralisk],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Ultralisk_Cavern,
upgrade_time_factor: 0,
name: "Chitinous_Plating"
}, 
UpgradeTypeData {what_uses: &[UnitType::Zerg_Ultralisk],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Zerg,
mineral_price_factor: 0,
what_upgrades: UnitType::Zerg_Ultralisk_Cavern,
upgrade_time_factor: 0,
name: "Anabolic_Synthesis"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Goliath],
gas_price_factor: 0,
max_repeats: 1,
race: Race::Terran,
mineral_price_factor: 0,
what_upgrades: UnitType::Terran_Machine_Shop,
upgrade_time_factor: 0,
name: "Charon_Boosters"
}, 
UpgradeTypeData {what_uses: &[UnitType::Terran_Vulture_Spider_Mine, UnitType::Critter_Ursadon, UnitType::Critter_Scantid, UnitType::Critter_Rhynadon, UnitType::Critter_Ragnasaur, UnitType::Critter_Kakaru, UnitType::Critter_Bengalaas, UnitType::Special_Cargo_Ship, UnitType::Special_Mercenary_Gunship, UnitType::Terran_SCV, UnitType::Protoss_Probe, UnitType::Zerg_Drone, UnitType::Zerg_Infested_Terran, UnitType::Zerg_Scourge],
gas_price_factor: 0,
max_repeats: 0,
race: Race::None,
mineral_price_factor: 0,
what_upgrades: UnitType::None,
upgrade_time_factor: 0,
name: "Upgrade_60"
}, 
UpgradeTypeData {what_uses: &[],
gas_price_factor: 0,
max_repeats: 0,
race: Race::None,
mineral_price_factor: 0,
what_upgrades: UnitType::None,
upgrade_time_factor: 0,
name: "None"
}, 
UpgradeTypeData {what_uses: &[],
gas_price_factor: 0,
max_repeats: 0,
race: Race::Unknown,
mineral_price_factor: 0,
what_upgrades: UnitType::None,
upgrade_time_factor: 0,
name: "Unknown"
}
];
impl UpgradeType {
fn d(&self) -> &UpgradeTypeData { &UPGRADE_TYPE_DATA[*self as usize] }
pub fn what_uses(&self) -> &'static [UnitType] {
  self.d().what_uses
}
pub fn gas_price_factor(&self) -> i32 {
  self.d().gas_price_factor
}
pub fn max_repeats(&self) -> i32 {
  self.d().max_repeats
}
pub fn get_race(&self) -> Race {
  self.d().race
}
pub fn mineral_price_factor(&self) -> i32 {
  self.d().mineral_price_factor
}
pub fn what_upgrades(&self) -> UnitType {
  self.d().what_upgrades
}
pub fn upgrade_time_factor(&self) -> i32 {
  self.d().upgrade_time_factor
}
pub fn name(&self) -> &'static str {
  self.d().name
}
}
