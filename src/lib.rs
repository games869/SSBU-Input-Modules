#![crate_name = "input_module"]
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
use {
	smash::{
	  lua2cpp::*,
	  phx::*,
	  app::{sv_animcmd::*, lua_bind::*, *},
	  lib::{lua_const::*, L2CValue, L2CAgent},
	  hash40
	},
  smash_script::*,
  smashline::{*, Priority::*}
};

pub mod InputModule;

pub fn test (i: i32) {
	println!("\n{i}\n);
}


#[skyline::main(name = "input_module")]
pub fn main() {

    InputModule::install();
	

}





