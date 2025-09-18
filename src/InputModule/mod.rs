pub use {
    smash::{
      lua2cpp::*,
      phx::*,
      app::{sv_animcmd::*, lua_bind::*, *},
      lib::{lua_const::*, L2CValue, L2CAgent},
      hash40
    },
  smash_script::*,
  smashline::{*, Priority::*},
  crate::InputModule::Command_Input_Module::InputDirection
};

pub mod Command_Input_Module;
pub mod Motion_Input_Module;
pub mod Charge_Input_Module;

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

pub fn install() {
	
	Command_Input_Module::install();
  Motion_Input_Module::install();
	Charge_Input_Module::install();
	
}