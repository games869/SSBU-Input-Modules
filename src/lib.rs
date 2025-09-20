#![crate_name = "input_modules"]
#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
	non_camel_case_types,
    clippy::borrow_interior_mutable_const,
	ambiguous_glob_reexports,
    hidden_glob_reexports
)]

#[macro_use]
extern crate lazy_static;
use {
	smash::{
	  lua2cpp::*,
	  phx::*,
	  app::{sv_animcmd::*, lua_bind::*, *},
	  lib::{lua_const::*, L2CValue, L2CAgent},
	  hash40
	},
  smash_script::*,
  smashline::{*, Priority::*},
  std::any::type_name,
};



const FORWARD_HIGH:f32 = 15.0;
const FORWARD_LOW:f32 = -15.0;

const DOWN_FORWARD_HIGH:f32 = -15.0;
const DOWN_FORWARD_SUB_LOW:f32 = -37.9;
const DOWN_FORWARD_SUB_HIGH:f32 = -38.0;
const DOWN_FORWARD_LOW:f32 = -80.0;

const DOWN_HIGH:f32 = -80.0;
const DOWN_LOW:f32 = -100.0;

const DOWN_BACK_HIGH:f32 = -100.0;
const DOWN_BACK_SUB_LOW:f32 = -141.9;
const DOWN_BACK_SUB_HIGH:f32 = -142.0;
const DOWN_BACK_LOW:f32 = -165.9;

const BACK_HIGH:f32 = 165.0;
const BACK_SUB_HIGH:f32 = 180.0;
const BACK_SUB_LOW:f32 = -180.0;
const BACK_LOW:f32 = -165.9;

const UP_BACK_HIGH:f32 = 165.0;
const UP_BACK_SUB_LOW:f32 = 120.9;
const UP_BACK_SUB_HIGH:f32 = 119.0;
const UP_BACK_LOW:f32 = 102.0;

const UP_HIGH:f32 = 102.0;
const UP_LOW:f32 = 79.0;

const UP_FORWARD_HIGH:f32 = 79.0;
const UP_FORWARD_SUB_LOW:f32 = 60.0;
const UP_FORWARD_SUB_HIGH:f32 = 59.9;
const UP_FORWARD_LOW:f32 = 15.0;


//every global input releated to ChargeInputModule
    //i havent started this one

use core::fmt;
/// A general list of directions the control stick could be in formatted like 2d fighting game notation 
/// 
/// Forward and back are always alinged with the character
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InputDirection {

    ERROR,
    /// 1 in fighing game notation
    DOWN_BACK,
    /// 2 in fighing game notation
    DOWN,
    /// 3 in fighing game notation
    DOWN_FORWARD,
    /// 4 in fighting game notation
    BACK,
    /// No direction being held
    /// 
    /// 5 in fighting game notation
    NEUTRAL,
    /// 6 in fighting game notation
    FORWARD,
    /// 7 in fighting game notation
    UP_BACK,
    /// 8 in fighting game notation
    UP,
    /// 9 in fighting game notation
    UP_FORWARD,

    NULL
    
}
/// A general list of directions the control stick could be in
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InputDirectionRaw {

    ERROR,
    DOWN_LEFT,
    DOWN,
    DOWN_RIGHT,
    LEFT,
    NEUTRAL,
    RIGHT,
    UP_LEFT,
    UP,
    UP_RIGHT,

}


static mut down_charge_time:[i32; 8] = [0; 8];
static mut back_charge_time:[i32; 8] = [0; 8];
static mut up_charge_time:[i32; 8] = [0; 8];
static mut forward_charge_time:[i32; 8] = [0; 8];

static mut down_charge_buffer_time:[i32; 8] = [0; 8];
static mut back_charge_buffer_time:[i32; 8] = [0; 8];
static mut up_charge_buffer_time:[i32; 8] = [0; 8];
static mut forward_charge_buffer_time:[i32; 8] = [0; 8];

static mut down_back_specific_charge_time:[i32; 8] = [0; 8];
static mut down_specific_charge_time:[i32; 8] = [0; 8];
static mut down_forward_specific_charge_time:[i32; 8] = [0; 8];
static mut back_specific_charge_time:[i32; 8] = [0; 8];
static mut forward_specific_charge_time:[i32; 8] = [0; 8];
static mut up_back_specific_charge_time:[i32; 8] = [0; 8];
static mut up_specific_charge_time:[i32; 8] = [0; 8];
static mut up_forward_specific_charge_time:[i32; 8] = [0; 8];


impl fmt::Display for InputDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputDirection::ERROR  => write!(f, "ERROR"),
            InputDirection::DOWN_BACK => write!(f, "DOWN_BACK"),
            InputDirection::DOWN => write!(f, "DOWN"),
            InputDirection::DOWN_FORWARD => write!(f, "DOWN_FORWARD"),
            InputDirection::BACK => write!(f, "BACK"),
            InputDirection::NEUTRAL => write!(f, "NEUTRAL"),
            InputDirection::FORWARD => write!(f, "FORWARD"),
            InputDirection::UP_BACK => write!(f, "UP_BACK"),
            InputDirection::UP => write!(f, "UP"),
            InputDirection::UP_FORWARD => write!(f, "UP_FORWARD"),
            InputDirection::NULL => write!(f, "NULL"),

        }
    }
}



impl fmt::Display for InputDirectionRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputDirectionRaw::ERROR  => write!(f, "ERROR"),
            InputDirectionRaw::DOWN_LEFT => write!(f, "DOWN_LEFT"),
            InputDirectionRaw::DOWN => write!(f, "DOWN"),
            InputDirectionRaw::DOWN_RIGHT => write!(f, "DOWN_RIGHT"),
            InputDirectionRaw::LEFT => write!(f, "LEFT"),
            InputDirectionRaw::NEUTRAL => write!(f, "NEUTRAL"),
            InputDirectionRaw::RIGHT => write!(f, "RIGHT"),
            InputDirectionRaw::UP_LEFT => write!(f, "UP_LEFT"),
            InputDirectionRaw::UP => write!(f, "UP"),
            InputDirectionRaw::UP_RIGHT => write!(f, "UP_RIGHT")
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    if type_name::<T>() != "&plugin::InputModule::Command_Input_Module::InputDirection" {
        type_name::<T>()
    }
    else {
        "InputDirection"
    }
}

/// A custom Module made to make the reading inputs easier
pub mod CommandInputModule {    

    use super::*;
    /// returns the general direction of where the control stick is in with fighting game style inputs
    /// 
    pub unsafe fn get_stick_dir(module_accessor:*mut BattleObjectModuleAccessor) -> InputDirection {
        let stick_ang = ControlModule::get_stick_angle(module_accessor).to_degrees();
        let stick_x = ControlModule::get_stick_x(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);

        if stick_x == 0.0 && stick_y == 0.0 {
            return InputDirection::NEUTRAL;
        }

        else if stick_ang > FORWARD_LOW && stick_ang < FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::FORWARD;
            }
            else {
                return InputDirection::BACK;
            }
        }

        else if stick_ang > DOWN_FORWARD_LOW && stick_ang <= DOWN_FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::DOWN_FORWARD;
            }
            else {
                return InputDirection::DOWN_BACK;
            }
        }
        else if stick_ang > DOWN_LOW && stick_ang <= DOWN_HIGH {

              return InputDirection::DOWN;

        }
        else if stick_ang > DOWN_BACK_LOW && stick_ang <= DOWN_BACK_HIGH {
            if lr > 0.0 {
                return InputDirection::DOWN_BACK;
            }
            else {
                return InputDirection::DOWN_FORWARD;
            }
        }
        else if 
            stick_ang >= BACK_SUB_LOW && stick_ang <= BACK_LOW
            ||
            stick_ang >= BACK_HIGH && stick_ang <= BACK_SUB_HIGH
        {
            if lr > 0.0 {
                return InputDirection::BACK;
            }
            else {
                return InputDirection::FORWARD;
            }
        }

        else if stick_ang >= UP_BACK_LOW && stick_ang <= UP_BACK_HIGH {
            if lr > 0.0 {
                return InputDirection::UP_BACK;
            }
            else {
                return InputDirection::UP_FORWARD;
            }
        }

        else if stick_ang >= UP_LOW && stick_ang <= UP_HIGH {

                return InputDirection::UP;

        }

        else if stick_ang >= UP_FORWARD_LOW && stick_ang <= UP_FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::UP_FORWARD;
            }
            else {
                return InputDirection::UP_BACK;
            }        }

        else {
            return InputDirection::ERROR;
        }

        
    }
    /// returns the general direction of where the control stick was LAST FRAME with fighting game style inputs
    pub unsafe fn get_prev_stick_dir(module_accessor:*mut BattleObjectModuleAccessor) -> InputDirection {
        
        let stick_x = ControlModule::get_stick_prev_x(module_accessor);
        let stick_y = ControlModule::get_stick_prev_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);

        let rad_ang = stick_y.atan2(stick_x);

        let stick_ang = rad_ang.to_degrees();
        
        if stick_x == 0.0 && stick_y == 0.0 {
            return InputDirection::NEUTRAL;
        }

        else if stick_ang > FORWARD_LOW && stick_ang < FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::FORWARD;
            }
            else {
                return InputDirection::BACK;
            }
        }

        else if stick_ang > DOWN_FORWARD_LOW && stick_ang <= DOWN_FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::DOWN_FORWARD;
            }
            else {
                return InputDirection::DOWN_BACK;
            }
        }

        else if stick_ang > DOWN_LOW && stick_ang <= DOWN_HIGH {
            return InputDirection::DOWN
        }

        else if stick_ang > DOWN_BACK_LOW && stick_ang <= DOWN_BACK_HIGH {
            if lr > 0.0 {
                return InputDirection::DOWN_BACK;
            }
            else {
                return InputDirection::DOWN_FORWARD;
            }
        }

        else if 
            stick_ang >= BACK_SUB_LOW && stick_ang <= BACK_LOW
            ||
            stick_ang >= BACK_HIGH && stick_ang <= BACK_SUB_HIGH
        {
            if lr > 0.0 {
                return InputDirection::BACK;
            }
            else {
                return InputDirection::FORWARD;
            }
        }

        else if stick_ang >= UP_BACK_LOW && stick_ang <= UP_BACK_HIGH {
            if lr > 0.0 {
                return InputDirection::UP_BACK;
            }
            else {
                return InputDirection::UP_FORWARD;
            }
        }

        else if stick_ang >= UP_LOW && stick_ang <= UP_HIGH {
            return InputDirection::UP;
        }

        else if stick_ang >= UP_FORWARD_LOW && stick_ang <= UP_FORWARD_HIGH {
            if lr > 0.0 {
                return InputDirection::UP_FORWARD;
            }
            else {
                return InputDirection::UP_BACK;
            }        }

        else {
            return InputDirection::ERROR;
        }

    }
    /// returns the general direction of where the control stick is in with general controller inputs
    pub unsafe fn get_stick_dir_raw(module_accessor:*mut BattleObjectModuleAccessor) -> InputDirectionRaw {
        let stick_ang = ControlModule::get_stick_angle(module_accessor).to_degrees();
        let stick_x = ControlModule::get_stick_x(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);

        if stick_x == 0.0 && stick_y == 0.0 {
            return InputDirectionRaw::NEUTRAL;
        }

        else if stick_ang > FORWARD_LOW && stick_ang < FORWARD_HIGH {
            return InputDirectionRaw::RIGHT;
        }

        else if stick_ang > DOWN_FORWARD_LOW && stick_ang <= DOWN_FORWARD_HIGH {
            return InputDirectionRaw::DOWN_RIGHT;
        }

        else if stick_ang > DOWN_LOW && stick_ang <= DOWN_HIGH {
            return InputDirectionRaw::DOWN
        }

        else if stick_ang > DOWN_BACK_LOW && stick_ang <= DOWN_BACK_HIGH {
            return InputDirectionRaw::DOWN_LEFT;
        }

        else if 
            stick_ang >= BACK_SUB_LOW && stick_ang <= BACK_LOW
            ||
            stick_ang >= BACK_HIGH && stick_ang <= BACK_SUB_HIGH
        {
            return InputDirectionRaw::LEFT;
        }

        else if stick_ang >= UP_BACK_LOW && stick_ang <= UP_BACK_HIGH {
            return InputDirectionRaw::UP_LEFT
        }

        else if stick_ang >= UP_LOW && stick_ang <= UP_HIGH {
            return InputDirectionRaw::UP;
        }

        else if stick_ang >= UP_FORWARD_LOW && stick_ang <= UP_FORWARD_HIGH {
            return InputDirectionRaw::UP_RIGHT;
        }

        else {
            return InputDirectionRaw::ERROR;
        }

        
    }
    /// returns the general direction of where the control stick was LAST FRAME with general controller inputs
    pub unsafe fn get_prev_stick_dir_raw(module_accessor:*mut BattleObjectModuleAccessor) -> InputDirectionRaw {
        
        let stick_x = ControlModule::get_stick_prev_x(module_accessor);
        let stick_y = ControlModule::get_stick_prev_y(module_accessor);

        let rad_ang = stick_y.atan2(stick_x);

        let stick_ang = rad_ang.to_degrees();
        
        if stick_x == 0.0 && stick_y == 0.0 {
            return InputDirectionRaw::NEUTRAL;
        }

        else if stick_ang > FORWARD_LOW && stick_ang < FORWARD_HIGH {
            return InputDirectionRaw::RIGHT;
        }

        else if stick_ang > DOWN_FORWARD_LOW && stick_ang <= DOWN_FORWARD_HIGH {
            return InputDirectionRaw::DOWN_RIGHT;
        }

        else if stick_ang > DOWN_LOW && stick_ang <= DOWN_HIGH {
            return InputDirectionRaw::DOWN
        }

        else if stick_ang > DOWN_BACK_LOW && stick_ang <= DOWN_BACK_HIGH {
            return InputDirectionRaw::DOWN_LEFT;
        }

        else if 
            stick_ang >= BACK_SUB_LOW && stick_ang <= BACK_LOW
            ||
            stick_ang >= BACK_HIGH && stick_ang <= BACK_SUB_HIGH
        {
            return InputDirectionRaw::LEFT;
        }

        else if stick_ang >= UP_BACK_LOW && stick_ang <= UP_BACK_HIGH {
            return InputDirectionRaw::UP_LEFT
        }

        else if stick_ang >= UP_LOW && stick_ang <= UP_HIGH {
            return InputDirectionRaw::UP;
        }

        else if stick_ang >= UP_FORWARD_LOW && stick_ang <= UP_FORWARD_HIGH {
            return InputDirectionRaw::UP_RIGHT;
        }

        else {
            return InputDirectionRaw::ERROR;
        }

    }
    /// Returns whether or not module_accessor has held a stick direction for 2 consecutive frames
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to BattleObjectModuleAccessor
    /// 
    /// * `dir` - the direction that is being checked InputDirection
    /// 
    pub unsafe fn is_charging(module_accessor:*mut BattleObjectModuleAccessor, dir: InputDirection) -> bool {
        if CommandInputModule::get_stick_dir(module_accessor) == CommandInputModule::get_prev_stick_dir(module_accessor) && CommandInputModule::get_stick_dir(module_accessor) == dir {
            return true;
        }
        else {
            return false;
        }
        
    }

    //buffer betwean motion inputs is 9 frames
    //charge time for hold inputs is 24 frames

    /// Returns whether or not module_accessor has held a cardinal InputDirection for a desired number of frames
    /// 
    /// terrys rising tackle needs 24 frames for the charge input
    ///
    /// # Arguments
    ///
    /// * `moudule_accessor` - a pointer to BattleObjectModuleAccessor
    /// 
    /// * `dir` - the direction that needs to be charged InputDirection
    ///
    /// * `length` - the number of frames the input must be held for i32 
    ///
    /// # Example
    ///
    /// ``` if the player has charged down on the control stick for 24 frames multiply the attack power by 1.5
    /// if CommandInputModule::is_charged(agent.module_accessor, InputDirection::DOWN, 24) {
    ///    AttackModule::set_power_up(agent.module_accessor, 1.5);    
    /// }
    /// ```
    pub unsafe fn is_charged(module_accessor:*mut BattleObjectModuleAccessor, dir: InputDirection, length: i32) -> bool {
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if dir == InputDirection::DOWN && down_charge_time[entry_id] >= length {
            return true;
        }
        else if dir == InputDirection::BACK && back_charge_time[entry_id] >= length {
            return true;
        }
        else if dir == InputDirection::UP && up_charge_time[entry_id] >= length {
            return true;
        }
        else if dir == InputDirection::FORWARD && forward_charge_time[entry_id] >= length {
            return true;
        }
        else {
            return false;
        }
    
    }

    ///returns weather or not a control_pad_button was hit on the same frame as a stick direction
    /// 
    /// # Arguments
    /// 
    /// *`module_accessor` a pointer to `BattleObjectModuleAccessor`
    /// 
    /// *`dir` the direction being checked `InputDirection`
    /// 
    /// *`use_easy_df` toggles if easy_df is used when checking for InputDirection::DOWN_FORWARD
    /// 
    /// *`button` the CONTROL_PAD_BUTTON being checked `i32`
    /// 
    /// *`allow_extra_frame` determines if the perfect input can be done 1 frame late simmaler to kazuya's ewgf `bool`
    /// 
    /// *`allow_negative_edge` determines if releaseing the input on a perfect frame counts as a perfect input `bool`
    /// 
    /// *`allow_cstick_perfect` determines if using the cstick can do perfect inputs `bool`
    /// 
    /// # Example
    ///
    /// ``` what kazuyas ewgf check would look like 
    /// if ewgf_step == 2 && CommandInputModule::is_perfect_input(fighter.module_accessor, InputDirection::DOWN_FORWARD, true, *CONTROL_PAD_BUTTON_ATTACK, true, true, false) {
    ///    FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT = true;
    /// }
    /// ```
    pub unsafe fn is_perfect_input(module_accessor:*mut BattleObjectModuleAccessor, button: i32, dir: InputDirection, allow_extra_frame: bool, allow_negative_edge: bool, allow_cstick_perfect: bool) -> bool {
        
        let stick_dir = CommandInputModule::get_stick_dir(module_accessor);

        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && !allow_cstick_perfect {
            
            println!("i see a cstick");
            return false;

        }

        if dir == stick_dir && CommandInputModule::get_prev_stick_dir(module_accessor) != dir && ControlModule::check_button_on_trriger(module_accessor, button) {
            println!("perfect");
            //println!("dir is a {}", type_of(&dir));
            return true;
            }

        if allow_extra_frame {
            if dir == stick_dir && get_specific_charge_time(module_accessor, &dir) <= 2 && ControlModule::check_button_on_trriger(module_accessor, button) {
                println!("lienient frame");
                return true;
            }
        }

        if allow_negative_edge {
            if dir == stick_dir && CommandInputModule::get_prev_stick_dir(module_accessor) != dir && ControlModule::check_button_on_release(module_accessor, button) {
                println!("negative edge perfect");
                return true;
            }
        }

        if allow_extra_frame && allow_negative_edge {
            if dir == stick_dir && get_specific_charge_time(module_accessor, &dir) <= 2 && ControlModule::check_button_on_release(module_accessor, button) {
                println!("negative lienient frame");
                return true;
            }
        }
        
        
        return false;
        
    }
}

unsafe fn inc_specific_charge_time(module_accessor:*mut BattleObjectModuleAccessor, dir: InputDirection) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if dir == InputDirection::NEUTRAL || dir == InputDirection::ERROR {
        
        down_back_specific_charge_time[entry_id] = 0;
        down_specific_charge_time[entry_id] = 0;
        down_forward_specific_charge_time[entry_id] = 0;

        back_specific_charge_time[entry_id] = 0;
        forward_specific_charge_time[entry_id] = 0;

        up_forward_specific_charge_time[entry_id] = 0;
        up_back_specific_charge_time[entry_id] = 0;
        up_specific_charge_time[entry_id] = 0;
            
        return;        

    }
        
    if dir != InputDirection::DOWN_BACK && down_back_specific_charge_time[entry_id] != 0 {

        down_back_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::DOWN_BACK {

        down_back_specific_charge_time[entry_id] += 1;

    }

    if dir != InputDirection::DOWN && down_specific_charge_time[entry_id] != 0 {

        down_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::DOWN {

        down_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::DOWN_FORWARD && down_forward_specific_charge_time[entry_id] != 0 {

        down_forward_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::DOWN_FORWARD {

        down_forward_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::BACK && back_specific_charge_time[entry_id] != 0 {

        back_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::BACK {

        back_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::FORWARD && forward_specific_charge_time[entry_id] != 0 {

        forward_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::FORWARD {

        forward_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::UP_BACK && up_back_specific_charge_time[entry_id] != 0 {

        up_back_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::UP_BACK {

        up_back_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::UP && up_specific_charge_time[entry_id] != 0 {

        up_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::UP {

        up_specific_charge_time[entry_id] += 1;
            
    }

    if dir != InputDirection::UP_FORWARD && up_forward_specific_charge_time[entry_id] != 0 {

        up_forward_specific_charge_time[entry_id] = 0;

    }
    else if dir == InputDirection::UP_FORWARD {

        up_forward_specific_charge_time[entry_id] += 1;
            
    }
}
///helper fn for is_perfect_input
unsafe fn get_specific_charge_time(module_accessor:*mut BattleObjectModuleAccessor, dir: &InputDirection) -> i32{
        
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if *dir == InputDirection::DOWN_BACK {

        return down_back_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::DOWN {

        return down_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::DOWN_FORWARD {

        return down_forward_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::BACK {

        return back_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::FORWARD {

        return forward_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::UP_BACK {

        return up_back_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::UP {

        return up_specific_charge_time[entry_id];

    }
    else if *dir == InputDirection::UP_FORWARD {

        return up_forward_specific_charge_time[entry_id];

    }

    return i32::MAX;

}











































pub const on: u8 = 1;
pub const off: u8 = 2;
pub const on_trigger: u8 = 3;
pub const on_release: u8 = 4;
pub const trigger: u8 = 5;
pub const release: u8 = 6;
pub const perfect: u8 = 7;

#[derive(PartialEq, Debug, Clone)]
pub struct inputs {
  has_input_atached: bool,
  input_type: Option<u8>,
  button: Option<i32>, 
  strict: bool,
  dir: Option<InputDirection>,
  allow_extra_frame: Option<bool>,
  allow_negative_edge: Option<bool>,
  allow_c_stick_input: Option<bool>
}


const default_buffer: u8 = 7;

static mut player_1_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_2_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_3_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_4_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_5_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_6_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_7_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_8_motion_vec: Vec<Vec<InputDirection>> = Vec::new();

static mut player_1_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_2_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_3_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_4_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_5_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_6_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_7_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_8_input_vec: Vec<Vec<inputs>> = Vec::new();

static mut player_1_step: Vec<usize> = Vec::new();
static mut player_2_step: Vec<usize> = Vec::new();
static mut player_3_step: Vec<usize> = Vec::new();
static mut player_4_step: Vec<usize> = Vec::new();
static mut player_5_step: Vec<usize> = Vec::new();
static mut player_6_step: Vec<usize> = Vec::new();
static mut player_7_step: Vec<usize> = Vec::new();
static mut player_8_step: Vec<usize> = Vec::new();

static mut player_1_buffer: Vec<u8> = Vec::new();
static mut player_2_buffer: Vec<u8> = Vec::new();
static mut player_3_buffer: Vec<u8> = Vec::new();
static mut player_4_buffer: Vec<u8> = Vec::new();
static mut player_5_buffer: Vec<u8> = Vec::new();
static mut player_6_buffer: Vec<u8> = Vec::new();
static mut player_7_buffer: Vec<u8> = Vec::new();
static mut player_8_buffer: Vec<u8> = Vec::new();

static mut player_1_buffer_window: u8 = default_buffer;
static mut player_2_buffer_window: u8 = default_buffer;
static mut player_3_buffer_window: u8 = default_buffer;
static mut player_4_buffer_window: u8 = default_buffer;
static mut player_5_buffer_window: u8 = default_buffer;
static mut player_6_buffer_window: u8 = default_buffer;
static mut player_7_buffer_window: u8 = default_buffer;
static mut player_8_buffer_window: u8 = default_buffer;

static mut player_1_use_motion_input_module: bool = false;
static mut player_2_use_motion_input_module: bool = false;
static mut player_3_use_motion_input_module: bool = false;
static mut player_4_use_motion_input_module: bool = false;
static mut player_5_use_motion_input_module: bool = false;
static mut player_6_use_motion_input_module: bool = false;
static mut player_7_use_motion_input_module: bool = false;
static mut player_8_use_motion_input_module: bool = false;

/// A custom Module made for making custom motion inputs eaiser to make 
pub mod MotionInputModule {

    use super::*;
	use super::InputDirection::*;
    /// Adds a custom motion to the fighter, this should be done on init or start
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `vec` - a `Vec` containing the desired `InputDirections` for the input
    /// 
    /// # Example
    /// 
    /// ```unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    /// 
    ///     //adds a down down motion to all fighters
    ///     MotionInputModule::add_motion(fighter.module_accessor, Vec::From(InputDirection::DOWN, InputDirection::NEUTRAL, InputDirection::DOWN))
    /// 
    /// }
    /// pub fn install() {
    /// 
    ///     Agent::new("fighter")
    ///     .on_start(on_start)
    ///     .install();
    /// 
    /// }```
    pub unsafe fn add_motion(module_accessor:*mut BattleObjectModuleAccessor, vec: Vec<InputDirection>) {

        let mut motion = vec;
        motion.push(NULL);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_vec = get_motion_vec(&entry_id);
        let player_input_vec = get_input_vec(&entry_id);
        let mut new_vec: Vec<inputs> = Vec::new();
        let use_motion_input_module = get_use_motion_input_module(&entry_id);
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = *get_buffer_window(&entry_id);

        player_vec.push(motion.clone());

        if !*use_motion_input_module { *use_motion_input_module = true; }

        let len = motion.len();
        let blank_inputs:inputs = inputs {

            has_input_atached: (false), 
            input_type: (None),
            button: (None), 
            strict: (false), 
            dir: (None), 
            allow_extra_frame: (None), 
            allow_negative_edge: (None), 
            allow_c_stick_input: (None) 

        };

        for i in 0..len { new_vec.push(blank_inputs.clone()); }
        player_input_vec.push(new_vec);
        step.push(0);
        buffer.push(buffer_window);
        
        dbg!(&player_vec);
        dbg!(&player_input_vec); 
        dbg!(&step);
        dbg!(&buffer);


    }

    /// Sets a custom buffer ammout for all custom inputs 
    /// 
    /// default is 7 which translates to 8 frames of time between inputs
    pub unsafe fn set_custom_input_buffer(module_accessor:*mut BattleObjectModuleAccessor, new_buffer: u8) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_buffer_window = get_buffer_window(&entry_id);

        *player_buffer_window = new_buffer;

    }

    /// Adds the ControlModule::check_button_off check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(DOWN, NEUTRAL, DOWN));
    /// MotionInputModule::add_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_off(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(off),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };

    }
    /// Adds the ControlModule::check_button_on check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a qcf motion and is pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(DOWN, DOWN_FORWARD, FORWARD));
    /// MotionInputModule::add_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, false, 2);
    /// ```
    pub unsafe fn add_button_on(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_on_release check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    pub unsafe fn add_button_on_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on_release),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_on_trigger check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    pub unsafe fn add_button_on_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on_trigger),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_trigger check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(BACK, FORWARD ));
    /// MotionInputModule::add_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(trigger),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_release check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a  motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(UP, UP_FORWARD, FORWARD));
    /// MotionInputModule::add_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(release),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the CommandInputModule::is_perfect_input check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `dir` - the `InputDirection` being checked
    /// 
    /// * `allow_extra_frame` - a `bool` that determines if you can do the input 1 frame late simmaler to kazuyas ewgf
    /// 
    /// * `allow_negative_edge` - a `bool` that determines if you can use negative edge to trigger the input
    /// 
    /// * `allow_c_stick_perfect` - a `bool` that determines if the cstick can be used to do the input
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(FORWARD, DOWN, DOWN_FORWARD));
    /// MotionInputModule::add_perfect_input(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, DOWN_FORWARD, false, false, false, true, 2);
    /// ```
    pub unsafe fn add_perfect_input(module_accessor:*mut BattleObjectModuleAccessor, button: i32, dir: InputDirection, allow_extra_frame: bool, allow_negative_edge: bool, allow_c_stick_perfect: bool, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(perfect),
            button: Some(button),
            strict: strict,
            dir: Some(dir),
            allow_extra_frame: Some(allow_extra_frame),
            allow_negative_edge: Some(allow_negative_edge),
            allow_c_stick_input: Some(allow_c_stick_perfect)
        };

        dbg!(&player_input_vec);

    }
    /// Gets what step a given input is on as a 'u8'
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `motion_input` - a `usize` of the input you are checking coresponding to the order it was set in the init or start
    pub unsafe fn get_input_step(module_accessor:*mut BattleObjectModuleAccessor, motion_input: usize) -> u8 {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let step = get_step(&entry_id);

        step[motion_input] as u8
    }
    /// Forces a given input to reset back to the first step [step 0]
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `motion_input` - a `usize` of the input you are checking coresponding to the order it was set in the init or start
    pub unsafe fn reset_input_step(module_accessor:*mut BattleObjectModuleAccessor, motion_input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = get_buffer_window(&entry_id);

        step[motion_input] = 0;
        buffer[motion_input] = 0;

    }
}

unsafe fn get_motion_vec(entry_id: &usize) -> &mut Vec<Vec<InputDirection>> {

    let motion_vec = if *entry_id == 0 { &mut player_1_motion_vec }
        else if *entry_id == 1 { &mut player_2_motion_vec } else if *entry_id == 2 { &mut player_3_motion_vec }
        else if *entry_id == 3 { &mut player_4_motion_vec } else if *entry_id == 4 { &mut player_5_motion_vec }
        else if *entry_id == 5 { &mut player_6_motion_vec } else if *entry_id == 6 { &mut player_7_motion_vec }
    else { &mut player_8_motion_vec };

    motion_vec

}
unsafe fn get_input_vec(entry_id: &usize) -> &mut Vec<Vec<inputs>> {
    
    let input_vec = if *entry_id == 0 { &mut player_1_input_vec }
        else if *entry_id == 1 { &mut player_2_input_vec } else if *entry_id == 2 { &mut player_3_input_vec }
        else if *entry_id == 3 { &mut player_4_input_vec } else if *entry_id == 4 { &mut player_5_input_vec }
        else if *entry_id == 5 { &mut player_6_input_vec } else if *entry_id == 6 { &mut player_7_input_vec }
    else { &mut player_8_input_vec };

    input_vec
}
unsafe fn get_step(entry_id: &usize) -> &mut Vec<usize> {

    let step = if *entry_id == 0 { &mut player_1_step }
        else if *entry_id == 1 { &mut player_2_step } else if *entry_id == 2 { &mut player_3_step }
        else if *entry_id == 3 { &mut player_4_step } else if *entry_id == 4 { &mut player_5_step }
        else if *entry_id == 5 { &mut player_6_step } else if *entry_id == 6 { &mut player_7_step }
    else { &mut player_8_step };

    step

}
unsafe fn get_buffer(entry_id: &usize) -> &mut Vec<u8> {
    
    let buffer = if *entry_id == 0 { &mut player_1_buffer }
        else if *entry_id == 1 { &mut player_2_buffer } else if *entry_id == 2 { &mut player_3_buffer }
        else if *entry_id == 3 { &mut player_4_buffer } else if *entry_id == 4 { &mut player_5_buffer }
        else if *entry_id == 5 { &mut player_6_buffer } else if *entry_id == 6 { &mut player_7_buffer }
    else { &mut player_8_buffer };

    buffer
}
unsafe fn get_buffer_window(entry_id: &usize) -> *mut u8 {
    
    let buffer_window = if *entry_id == 0 { &raw mut player_1_buffer_window }
        else if *entry_id == 1 { &raw mut player_2_buffer_window } else if *entry_id == 2 { &raw mut player_3_buffer_window }
        else if *entry_id == 3 { &raw mut player_4_buffer_window } else if *entry_id == 4 { &raw mut player_5_buffer_window }
        else if *entry_id == 5 { &raw mut player_6_buffer_window } else if *entry_id == 6 { &raw mut player_7_buffer_window }
    else { &raw mut player_8_buffer_window };

    buffer_window
}
unsafe fn get_use_motion_input_module(entry_id: &usize) -> *mut bool {

    let is_using_motion_input_module = if *entry_id == 0 { &raw mut player_1_use_motion_input_module }
        else if *entry_id == 1 { &raw mut player_2_use_motion_input_module } else if *entry_id == 2 { &raw mut player_3_use_motion_input_module }
        else if *entry_id == 3 { &raw mut player_4_use_motion_input_module } else if *entry_id == 4 { &raw mut player_5_use_motion_input_module }
        else if *entry_id == 5 { &raw mut player_6_use_motion_input_module } else if *entry_id == 6 { &raw mut player_7_use_motion_input_module }
    else { &raw mut player_8_use_motion_input_module };

    is_using_motion_input_module

}

unsafe fn get_inputs_for_current_step(module_accessor:*mut BattleObjectModuleAccessor, input: &inputs) -> bool {

    if !input.has_input_atached {

        return true

    }
    else if input.input_type == Some(on) {
        
        return ControlModule::check_button_on(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(off) {

        return ControlModule::check_button_off(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(on_trigger) {

        return ControlModule::check_button_on_trriger(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(on_release) {

        return ControlModule::check_button_on_release(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(trigger) {

        return ControlModule::check_button_trigger(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(release) {

        return ControlModule::check_button_release(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(perfect) {

        return CommandInputModule::is_perfect_input(module_accessor, input.button.expect("no input found"), input.dir.expect("no input found"), input.allow_extra_frame.expect("no input found"), input.allow_negative_edge.expect("no input found"), input.allow_c_stick_input.expect("no input found"))

    }
    else {

        false
    }
}

unsafe extern "C" fn motion_input_frame(fighter: &mut L2CFighterCommon) {
        
    if !StatusModule::is_changing(fighter.module_accessor) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if *get_use_motion_input_module(&entry_id) {

            let motion_vec = get_motion_vec(&entry_id);
            let motion_input_count = motion_vec.len();
            let input_vec = get_input_vec(&entry_id);
            let step = get_step(&entry_id);
            let prev_step = step.clone();
            let buffer = get_buffer(&entry_id);
            let buffer_window = *get_buffer_window(&entry_id);
            let input_dir = CommandInputModule::get_stick_dir(fighter.module_accessor);
            for current_input in 0..motion_input_count {

                let input = get_inputs_for_current_step(fighter.module_accessor, &input_vec[current_input][step[current_input]]);
                let input_compare_against = if input_vec[current_input][step[current_input]].input_type == Some(perfect) { ControlModule::check_button_on_trriger(fighter.module_accessor, input_vec[current_input][step[current_input]].button.expect("no input found"))} 
                    else {get_inputs_for_current_step(fighter.module_accessor, &input_vec[current_input][step[current_input]])};
                let max = motion_vec[current_input].len() - 1;

                if step[current_input] == prev_step[current_input] && buffer[current_input] > 0 {

                    buffer[current_input] -= 1;

                }
                else if buffer[current_input] == 0 {

                    step[current_input] = 0;

                }

                if input_dir == motion_vec[current_input][step[current_input]] && step[current_input] != max && input {

                    step[current_input] += 1;
                    buffer[current_input] = buffer_window;

                }
                else if input_dir == motion_vec[current_input][step[current_input]] && step[current_input] != max && input_vec[current_input][step[current_input]].strict 
                || input_compare_against && step[current_input] != max && input_vec[current_input][step[current_input]].strict {

                    step[current_input] = 0;
                    buffer[current_input] = buffer_window;

                }
            }
            dbg!(&step);
        }
    }
}

unsafe extern "C" fn reset_motion_input_module(fighter: &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if *get_use_motion_input_module(&entry_id) {

        let motion_vec = get_motion_vec(&entry_id);
        let input_vec = get_input_vec(&entry_id);
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = get_buffer_window(&entry_id);
        let use_motion_input_module = get_use_motion_input_module(&entry_id);

        *motion_vec = Vec::new();
        *input_vec = Vec::new();
        *step = Vec::new();
        *buffer = Vec::new();
        *buffer_window = default_buffer;
        *use_motion_input_module = false;

    }
}

#[no_mangle]
pub fn test(i: i32) {
	println!("i == {i}");
}

#[skyline::main(name = "input_module")]
pub fn main() {

	Agent::new("fighter")
        .on_line(Main, motion_input_frame)
        .on_end(reset_motion_input_module)
    .install();

}



