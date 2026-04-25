#![crate_name = "inputmodule"]
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
    

impl InputType {
    pub fn get_negative_instance(&self) -> InputType {
        match self {

            InputType::None => InputType::None,
            InputType::On  => InputType::Off,
            InputType::Off => InputType::On,
            InputType::On_Trigger => InputType::On_Release,
            InputType::On_Release => InputType::On_Trigger,
            InputType::Trigger => InputType::Release,
            InputType::Release => InputType::Trigger,
            InputType::Perfect => InputType::Perfect

        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StickType {

    Both,
    Control_Stick_Only,
    C_Stick_Only
    
}


/// A custom module made to make checking the current input a little bit easier
pub mod CommandInputModule;
/// A custom module made to make the creation of custom motion inputs easy and simple
pub mod MotionInputModule;

/// A custom module made to make the creation of custom charge inputs easy and simple
pub mod ChargeInputModule;

/*
+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_

	CommandInputModule
		-move charge stuff to charge input module
	
	ChargeInputModule
		-code the module

+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_
*/