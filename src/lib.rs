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
	}, std::fmt
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

#[allow(unused)]
impl InputDirection {
    pub fn to_notation(&self) -> u8 {
        match self {
            Self::ERROR => 0,
            Self::DOWN_BACK => 1,
            Self::DOWN => 2,
            Self::DOWN_FORWARD => 3,
            Self::BACK => 4,
            Self::NEUTRAL => 5,
            Self::FORWARD => 6,
            Self::UP_BACK => 7,
            Self::UP => 8,
            Self::UP_FORWARD => 9,
            Self::NULL => 10
        }
    }
}

pub trait ToInputDir {
    fn to_inputdir(self) -> InputDirection;
}

impl ToInputDir for u32 {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for i32 {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for usize {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for isize {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for f32 {
    fn to_inputdir(self) -> InputDirection {
        match self.floor() {
            1.0 => InputDirection::DOWN_BACK,
            2.0 => InputDirection::DOWN,
            3.0 => InputDirection::DOWN_FORWARD,
            4.0 => InputDirection::BACK,
            5.0 => InputDirection::NEUTRAL,
            6.0 => InputDirection::FORWARD,
            7.0 => InputDirection::UP_BACK,
            8.0 => InputDirection::UP,
            9.0 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for u64 {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for i64 {
    fn to_inputdir(self) -> InputDirection {
        match self {
            1 => InputDirection::DOWN_BACK,
            2 => InputDirection::DOWN,
            3 => InputDirection::DOWN_FORWARD,
            4 => InputDirection::BACK,
            5 => InputDirection::NEUTRAL,
            6 => InputDirection::FORWARD,
            7 => InputDirection::UP_BACK,
            8 => InputDirection::UP,
            9 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for f64 {
    fn to_inputdir(self) -> InputDirection {
        match self.floor() {
            1.0 => InputDirection::DOWN_BACK,
            2.0 => InputDirection::DOWN,
            3.0 => InputDirection::DOWN_FORWARD,
            4.0 => InputDirection::BACK,
            5.0 => InputDirection::NEUTRAL,
            6.0 => InputDirection::FORWARD,
            7.0 => InputDirection::UP_BACK,
            8.0 => InputDirection::UP,
            9.0 => InputDirection::UP_FORWARD,
            _ => InputDirection::NULL
        }
    }
}

impl ToInputDir for InputDirectionRaw {
    fn to_inputdir(self) -> InputDirection {
        match self {
            Self::DOWN_RIGHT => InputDirection::DOWN_BACK,
            Self::DOWN => InputDirection::DOWN,
            Self::DOWN_LEFT => InputDirection::DOWN_FORWARD,
            Self::LEFT => InputDirection::BACK,
            Self::NEUTRAL => InputDirection::NEUTRAL,
            Self::RIGHT => InputDirection::FORWARD,
            Self::UP_LEFT => InputDirection::UP_BACK,
            Self::UP => InputDirection::UP,
            Self::UP_RIGHT => InputDirection::UP_FORWARD,
            Self::NULL => InputDirection::NULL,
            Self::ERROR => InputDirection::ERROR
        }
    }
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
    NULL

}

#[allow(unused)]
impl InputDirectionRaw {
    pub fn to_notation(&self) -> u8 {
        match self {
            Self::ERROR => 0,
            Self::DOWN_LEFT => 1,
            Self::DOWN => 2,
            Self::DOWN_RIGHT => 3,
            Self::LEFT => 4,
            Self::NEUTRAL => 5,
            Self::RIGHT => 6,
            Self::UP_LEFT => 7,
            Self::UP => 8,
            Self::UP_RIGHT => 9,
            Self::NULL => 10
        }
    }
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
    
pub trait ToInputDirRaw {
    fn to_inputdirraw(self) -> InputDirectionRaw;
}

impl ToInputDirRaw for u32 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for i32 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for usize {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for isize {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for f32 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self.floor() {
            1.0 => InputDirectionRaw::DOWN_LEFT,
            2.0 => InputDirectionRaw::DOWN,
            3.0 => InputDirectionRaw::DOWN_RIGHT,
            4.0 => InputDirectionRaw::LEFT,
            5.0 => InputDirectionRaw::NEUTRAL,
            6.0 => InputDirectionRaw::RIGHT,
            7.0 => InputDirectionRaw::UP_LEFT,
            8.0 => InputDirectionRaw::UP,
            9.0 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for u64 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for i64 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            1 => InputDirectionRaw::DOWN_LEFT,
            2 => InputDirectionRaw::DOWN,
            3 => InputDirectionRaw::DOWN_RIGHT,
            4 => InputDirectionRaw::LEFT,
            5 => InputDirectionRaw::NEUTRAL,
            6 => InputDirectionRaw::RIGHT,
            7 => InputDirectionRaw::UP_LEFT,
            8 => InputDirectionRaw::UP,
            9 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for f64 {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self.floor() {
            1.0 => InputDirectionRaw::DOWN_LEFT,
            2.0 => InputDirectionRaw::DOWN,
            3.0 => InputDirectionRaw::DOWN_RIGHT,
            4.0 => InputDirectionRaw::LEFT,
            5.0 => InputDirectionRaw::NEUTRAL,
            6.0 => InputDirectionRaw::RIGHT,
            7.0 => InputDirectionRaw::UP_LEFT,
            8.0 => InputDirectionRaw::UP,
            9.0 => InputDirectionRaw::UP_RIGHT,
            _ => InputDirectionRaw::NULL
        }
    }
}

impl ToInputDirRaw for InputDirection {
    fn to_inputdirraw(self) -> InputDirectionRaw {
        match self {
            Self::DOWN_BACK => InputDirectionRaw::DOWN_LEFT,
            Self::DOWN => InputDirectionRaw::DOWN,
            Self::DOWN_FORWARD => InputDirectionRaw::DOWN_RIGHT,
            Self::BACK => InputDirectionRaw::LEFT,
            Self::NEUTRAL => InputDirectionRaw::NEUTRAL,
            Self::FORWARD => InputDirectionRaw::RIGHT,
            Self::UP_BACK => InputDirectionRaw::UP_LEFT,
            Self::UP => InputDirectionRaw::UP,
            Self::UP_FORWARD => InputDirectionRaw::UP_RIGHT,
            Self::NULL => InputDirectionRaw::NULL,
            Self::ERROR => InputDirectionRaw::ERROR
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

impl std::fmt::Display for InputDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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



impl std::fmt::Display for InputDirectionRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            InputDirectionRaw::UP_RIGHT => write!(f, "UP_RIGHT"),
            InputDirectionRaw::NULL => write!(f, "NULL")
        }
    }
}

pub trait ToDirVector {
    fn into_vec(self) -> Vec<InputDirection>;
}

impl ToDirVector for u32 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for i32 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for usize {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for isize {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for f32 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for u64 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for i64 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for f64 {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for InputDirection {
    fn into_vec(self) -> Vec<InputDirection> { vec![self] }
}

impl ToDirVector for InputDirectionRaw {
    fn into_vec(self) -> Vec<InputDirection> { vec![self.to_inputdir()] }
}

impl ToDirVector for Vec<u32> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<i32> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<usize> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<isize> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<f32> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<u64> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<i64> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<f64> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<InputDirectionRaw> {
    fn into_vec(self) -> Vec<InputDirection> {
        
        let mut ret: Vec<InputDirection> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdir());
        }

        ret
    }
}

impl ToDirVector for Vec<InputDirection> {
    fn into_vec(self) -> Vec<InputDirection> { self }
}

pub trait ToRawDirVector {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw>;
}

impl ToRawDirVector for u32 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for i32 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for usize {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for isize {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for f32 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for u64 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for i64 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for f64 {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for InputDirection {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self.to_inputdirraw()] }
}

impl ToRawDirVector for InputDirectionRaw {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { vec![self] }
}

impl ToRawDirVector for Vec<u32> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<i32> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<usize> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<isize> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<f32> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<u64> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<i64> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<f64> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<InputDirection> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> {
        
        let mut ret: Vec<InputDirectionRaw> = vec![];
        
        for index in 0..self.len() {
            ret.push(self[index].to_inputdirraw());
        }

        ret
    }
}

impl ToRawDirVector for Vec<InputDirectionRaw> {
    fn into_raw_vec(self) -> Vec<InputDirectionRaw> { self }
}

pub trait ToVector {
    fn as_usize_vec(self) -> Vec<usize>;
    fn as_isize_vec(self) -> Vec<isize>;
    fn as_uint_vec(self) -> Vec<u32>;
    fn as_int_vec(self) -> Vec<i32>;
    fn as_float_vec(self) -> Vec<f32>;
    fn as_u64_vec(self) -> Vec<u64>;
    fn as_i64_vec(self) -> Vec<i64>;
    fn as_f64_vec(self) -> Vec<f64>;
}

impl ToVector for usize {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for isize {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for u32 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for i32 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for f32 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for u64 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for i64 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self as f64]
    }
}

impl ToVector for f64 {
    fn as_usize_vec(self) -> Vec<usize> {
        vec![self as usize]
    }
    fn as_isize_vec(self) -> Vec<isize> {
        vec![self as isize]
    }
    fn as_uint_vec(self) -> Vec<u32> {
        vec![self as u32]
    }
    fn as_int_vec(self) -> Vec<i32> {
        vec![self as i32]
    }
    fn as_float_vec(self) -> Vec<f32> {
        vec![self as f32]
    }
    fn as_u64_vec(self) -> Vec<u64> {
        vec![self as u64]
    }
    fn as_i64_vec(self) -> Vec<i64> {
        vec![self as i64]
    }
    fn as_f64_vec(self) -> Vec<f64> {
        vec![self]
    }
}

impl ToVector for Vec<usize> {
    fn as_usize_vec(self) -> Vec<usize> {
        self
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<isize> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        self
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<u32> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        self
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<i32> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        self
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<f32> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        self
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<u64> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        self
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}

impl ToVector for Vec<i64> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        self
    }
    fn as_f64_vec(self) -> Vec<f64> {
        let mut ret_vec: Vec<f64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f64);
        }

        ret_vec
    }
}
impl ToVector for Vec<f64> {
    fn as_usize_vec(self) -> Vec<usize> {
        let mut ret_vec: Vec<usize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as usize);
        }

        ret_vec
    }
    fn as_isize_vec(self) -> Vec<isize> {
        let mut ret_vec: Vec<isize> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as isize);
        }

        ret_vec
    }
    fn as_uint_vec(self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u32);
        }

        ret_vec
    }
    fn as_int_vec(self) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i32);
        }

        ret_vec
    }
    fn as_float_vec(self) -> Vec<f32> {
        let mut ret_vec: Vec<f32> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as f32);
        }

        ret_vec
    }
    fn as_u64_vec(self) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as u64);
        }

        ret_vec
    }
    fn as_i64_vec(self) -> Vec<i64> {
        let mut ret_vec: Vec<i64> = vec![];

        for i in 0..self.len() {
            ret_vec.push(self[i] as i64);
        }

        ret_vec
    }
    fn as_f64_vec(self) -> Vec<f64> {
        self
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