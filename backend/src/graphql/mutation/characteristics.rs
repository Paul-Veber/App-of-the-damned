use entity::async_graphql::{self, InputObject};
use entity::unit::Characteristics;
use sea_orm::ActiveValue;

use crate::utils::update_json_value::update_json_value;

#[derive(InputObject,Clone, Debug)]
pub struct CharacteristicsUpdate {
   pub movement: Option<u8>,
   pub weapon_skill: Option<u8>,
   pub ballistic_skill: Option<u8>,
   pub strength: Option<u8>,
   pub toughness: Option<u8>,
   pub wounds: Option<u8>,
   pub initiative: Option<u8>,
   pub attacks: Option<u8>,
   pub leadership: Option<u8>,
   pub armor_save: Option<u8>,
}

pub fn update_characteristics(input: CharacteristicsUpdate, default_value:  ActiveValue<Characteristics>) -> Characteristics {
    let mut updated_charac: Characteristics = default_value.unwrap().into();

    updated_charac.movement = update_json_value(input.movement, updated_charac.movement);
    updated_charac.weapon_skill = update_json_value(input.weapon_skill, updated_charac.weapon_skill);
    updated_charac.ballistic_skill = update_json_value(input.ballistic_skill, updated_charac.ballistic_skill);
    updated_charac.strength = update_json_value(input.strength, updated_charac.strength);
    updated_charac.toughness = update_json_value(input.toughness, updated_charac.toughness);
    updated_charac.wounds = update_json_value(input.wounds, updated_charac.wounds);
    updated_charac.initiative = update_json_value(input.initiative, updated_charac.initiative);
    updated_charac.attacks = update_json_value(input.attacks, updated_charac.attacks);
    updated_charac.leadership = update_json_value(input.leadership, updated_charac.leadership);
    updated_charac.armor_save = update_json_value(input.armor_save, updated_charac.armor_save);

    updated_charac
}
