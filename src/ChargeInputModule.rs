use {
    smash::{
        lua2cpp::*, 
        phx::*,
        app::{ lua_bind::*, sv_animcmd::*, * }, 
        lib::{ lua_const::*, L2CAgent, L2CValue },
        hash40
    }, 
    smash_script::*, 
    smashline::{ Priority::*, * }, 
    std::{ any::type_name, usize },
    super::{
        *,
        CommandInputModule::{
            *,
            InputDirection::*
        }
    }
}; 

const on: u8 = 1;
const off: u8 = 2;
const on_trigger: u8 = 3;
const on_release: u8 = 4;
const trigger: u8 = 5;
const release: u8 = 6;
const perfect: u8 = 7;

#[derive(PartialEq, Debug, Clone)]
struct inputs {
  has_input_atached: bool,
  input_type: Option<u8>,
  button: Option<i32>, 
  strict: bool,
  dir: Option<InputDirection>,
  allow_extra_frame: Option<bool>,
  allow_negative_edge: Option<bool>,
  allow_c_stick_input: Option<bool>
}


static mut test_charge_bool:bool = false;

const default_charge_time: u8 = 24;
const default_buffer:u8 = 10;

static mut player_1_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_2_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_3_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_4_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_5_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_6_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_7_charge_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_8_charge_vec: Vec<Vec<InputDirection>> = Vec::new();

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

static mut player_1_use_charge_input_module: bool = false;
static mut player_2_use_charge_input_module: bool = false;
static mut player_3_use_charge_input_module: bool = false;
static mut player_4_use_charge_input_module: bool = false;
static mut player_5_use_charge_input_module: bool = false;
static mut player_6_use_charge_input_module: bool = false;
static mut player_7_use_charge_input_module: bool = false;
static mut player_8_use_charge_input_module: bool = false;

/// A custom Module mad for making custom charge inputs eaiser
pub mod ChargeInputModule {
    
    use super::*;

    pub unsafe fn add_button_off(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

    }
    pub unsafe fn add_button_on(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
    }
    pub unsafe fn add_button_on_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
    }
    pub unsafe fn add_button_on_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
    }
    pub unsafe fn add_button_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
    }
    pub unsafe fn add_button_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
    }
    pub unsafe fn add_perfect_input(module_accessor:*mut BattleObjectModuleAccessor, button: i32, dir: InputDirection, allow_extra_frame: bool, allow_negative_edge: bool, allow_c_stick_perfect: bool, strict: bool, input: usize) {
        
    }

}


//todo ... make the f-ing module @games