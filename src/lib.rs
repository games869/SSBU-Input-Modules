#![crate_name = "inputmodule"]
#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    non_camel_case_types
)]
// todo fix all these hidden warnings
use {
	smash::{
	  app::{lua_bind::*, *},
	  lib::lua_const::*
	},
  std::fmt
};

#[derive(PartialEq, Debug, Clone, Copy)]

pub enum InputType {
    
    None,
    On,
    Off,
    On_Trigger,
    On_Release,
    Trigger,
    Release,
    Perfect

}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StickType {

    Both,
    Control_Stick_Only,
    C_Stick_Only
    
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            
            InputType::None => write!(f, "none"),
            InputType::On  => write!(f, "on"),
            InputType::Off => write!(f, "off"),
            InputType::On_Trigger => write!(f, "on_trigger"),
            InputType::On_Release => write!(f, "on_release"),
            InputType::Trigger => write!(f, "trigger"),
            InputType::Release => write!(f, "release"),
            InputType::Perfect => write!(f, "perfect")
            
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