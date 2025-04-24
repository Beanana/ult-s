use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("inkling")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, ink)
	.install();
}

unsafe extern "C" fn ink(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if is_default(boma) {
			let cat1 = ControlModule::get_command_flag_cat(boma, 0);
			if status_kind == *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK {
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_DASH && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0{
					StatusModule::change_status_request_from_script(boma, *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_END, true);
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DASH);;
				};
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_DASH && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0{
					StatusModule::change_status_request_from_script(boma, *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_END, true);
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DASH_BACK);
				};
			};
			if [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL].contains(&status_kind) || (status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP && frame > 8.0) {
					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) && motion_kind != hash40("special_hi_down") {
						MotionModule::change_motion(boma, Hash40::new("special_hi_down"), -1.0, 1.0, false, 0.0, false, false);
						macros::PLAY_SE(fighter, Hash40::new("se_inkling_special_h01"));
						ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID,smash::phx::Hash40::new("special_hi_2"),false,0.0);
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					}
			};
			if motion_kind == hash40("special_hi_down") {
				if GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_DOWN as u64 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_superjump_jet"), Hash40::new("hip"), -2, 0, 0, 0, 0, -90, 1, true);
					EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
					macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
				
					StatusModule::change_status_request_from_script(boma, 25, true);
				}
			};
			if motion_kind == hash40("special_hi_landing_r") || motion_kind == hash40("special_hi_landing_l"){
				let slideoff = 1.5 - (frame*WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0));
				if slideoff > 0.0 {
					macros::SET_SPEED_EX(fighter, slideoff, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				}
			};
		};
	};
}