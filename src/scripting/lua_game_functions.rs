use std::collections::HashMap;

use rose_game_common::components::CharacterGender;

use crate::scripting::{
    lua4::Lua4Value,
    lua_game_constants::{
        SV_BIRTH, SV_CHA, SV_CLASS, SV_CON, SV_DEX, SV_EXP, SV_FAME, SV_INT, SV_LEVEL, SV_RANK,
        SV_SEN, SV_SEX, SV_STR, SV_UNION,
    },
    ScriptFunctionContext,
};

pub struct LuaGameFunctions {
    pub closures: HashMap<String, fn(&mut ScriptFunctionContext, Vec<Lua4Value>) -> Vec<Lua4Value>>,
}

impl Default for LuaGameFunctions {
    fn default() -> Self {
        let mut closures: HashMap<
            String,
            fn(&mut ScriptFunctionContext, Vec<Lua4Value>) -> Vec<Lua4Value>,
        > = HashMap::new();

        closures.insert("GF_getVariable".into(), GF_getVariable);

        /*
        GF_addUserMoney
        GF_appraisal
        GF_ChangeState
        GF_checkNumOfInvItem
        GF_checkTownItem
        GF_checkUserMoney
        GF_DeleteEffectFromObject
        GF_disorganizeClan
        GF_EffectOnObject
        GF_error
        GF_getDate
        GF_GetEffectUseFile
        GF_GetEffectUseIndex
        GF_getGameVersion
        GF_getIDXOfInvItem
        GF_getItemRate
        GF_GetMotionUseFile
        GF_getName
        GF_getReviveZoneName
        GF_GetTarget
        GF_getTownRate
        GF_getTownVar
        GF_getWorldRate
        GF_getZone
        GF_giveEquipItemIntoInv
        GF_giveUsableItemIntoInv
        GF_log
        GF_LogString
        GF_movableXY
        GF_moveEvent
        GF_moveXY
        GF_openBank
        GF_openDeliveryStore
        GF_openSeparate
        GF_openStore
        GF_openUpgrade
        GF_organizeClan
        GF_playEffect
        GF_playSound
        GF_putoffItem
        GF_putonItem
        GF_Random
        GF_repair
        GF_rotateCamera
        GF_setEquipedItem
        GF_SetMotion
        GF_setRevivePosition
        GF_setTownRate
        GF_setVariable
        GF_setWorldRate
        GF_spawnMonAtEvent
        GF_spawnMonXY
        GF_takeItemFromInv
        GF_takeUserMoney
        GF_warp
        GF_WeatherEffectOnObject
        GF_zoomCamera
        */

        Self { closures }
    }
}

#[allow(non_snake_case)]
fn GF_getVariable(
    context: &mut ScriptFunctionContext,
    parameters: Vec<Lua4Value>,
) -> Vec<Lua4Value> {
    let variable_id = parameters[0].to_i32().unwrap();
    let (character_info, basic_stats, experience_points, level, union_membership) =
        context.query_character.single();

    let value = match variable_id {
        SV_SEX => match character_info.gender {
            CharacterGender::Male => 0,
            CharacterGender::Female => 1,
        },
        SV_BIRTH => character_info.birth_stone as i32,
        SV_CLASS => character_info.job as i32,
        SV_UNION => union_membership
            .current_union
            .map(|x| x.get() as i32)
            .unwrap_or(0),
        SV_RANK => character_info.rank as i32,
        SV_FAME => character_info.fame as i32,
        SV_STR => basic_stats.strength,
        SV_DEX => basic_stats.dexterity,
        SV_INT => basic_stats.intelligence,
        SV_CON => basic_stats.concentration,
        SV_CHA => basic_stats.charm,
        SV_SEN => basic_stats.sense,
        SV_EXP => experience_points.xp as i32,
        SV_LEVEL => level.level as i32,
        _ => 0,
    };

    vec![value.into()]
}
