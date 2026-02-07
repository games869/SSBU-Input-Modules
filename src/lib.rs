#![crate_name = "inputmodule"]
#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
	static_mut_refs,
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	//unused,
	non_upper_case_globals,
	non_snake_case,
	non_camel_case_types,
    clippy::borrow_interior_mutable_const,
	ambiguous_glob_reexports,
    hidden_glob_reexports,
    dead_code
)]
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
  std::{any::type_name, fmt}
};

#[derive(PartialEq, Debug, Clone, Copy)]

pub enum InputType {
    
    none,
    on,
    off,
    on_trigger,
    on_release,
    trigger,
    release,
    perfect

}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StickType {

    both,
    control_stick_only,
    c_stick_only
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            
            InputType::none => write!(f, "none"),
            InputType::on  => write!(f, "on"),
            InputType::off => write!(f, "off"),
            InputType::on_trigger => write!(f, "on_trigger"),
            InputType::on_release => write!(f, "on_release"),
            InputType::trigger => write!(f, "trigger"),
            InputType::release => write!(f, "release"),
            InputType::perfect => write!(f, "perfect")
            
        }
    }
}

/// A custom module made to make checking the current input a little bit easier
pub mod CommandInputModule;
/// A custom module made to make the creation of custom motion intputs easy and simple
pub mod MotionInputModule;

// A custom module that ... YOU STILL NEED TO CODE @games 
pub mod ChargeInputModule;

/*
+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_

	CommandInputModule
		-move charge stuff to charge input module
	
	ChargeInputModule
		-code the module

+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_
*/