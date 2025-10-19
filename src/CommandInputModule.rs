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
    core::fmt
}; 

//a shit tone of stick angles for use in the CommandInputModule
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

/// A custom Module made to make the reading inputs easier

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
        if get_stick_dir(module_accessor) == get_prev_stick_dir(module_accessor) && get_stick_dir(module_accessor) == dir {
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
        
        let stick_dir = get_stick_dir(module_accessor);

        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && !allow_cstick_perfect {
            
            println!("i see a cstick");
            return false;

        }

        if dir == stick_dir && get_prev_stick_dir(module_accessor) != dir && ControlModule::check_button_on_trriger(module_accessor, button) {
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
            if dir == stick_dir && get_prev_stick_dir(module_accessor) != dir && ControlModule::check_button_on_release(module_accessor, button) {
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


pub unsafe fn update_is_perfect(fighter: &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let dir = get_stick_dir(fighter.module_accessor);

    if !StatusModule::is_changing(fighter.module_accessor) { 
        if dir == InputDirection::DOWN || dir == InputDirection::DOWN_BACK || dir == InputDirection::DOWN_FORWARD {
            
            down_charge_time[entry_id] += 1;
            down_charge_buffer_time[entry_id] = 9;

        }
        else {
            if down_charge_buffer_time[entry_id] > 0 {
                down_charge_buffer_time[entry_id] -= 1;
            }
            else if down_charge_time[entry_id] != 0 {
                
                down_charge_time[entry_id] = 0;

            }
        } 
        if dir == InputDirection::BACK || dir == InputDirection::UP_BACK || dir == InputDirection::DOWN_BACK {
            back_charge_time[entry_id] += 1;
            back_charge_buffer_time[entry_id] = 9;
        }
        else {
            if back_charge_buffer_time[entry_id] > 0 {
                back_charge_buffer_time[entry_id] -= 1;
            }
            else if back_charge_time[entry_id] != 0 {
                back_charge_time[entry_id] = 0;
            }
        }

        if dir == InputDirection::UP || dir == InputDirection::UP_BACK || dir == InputDirection::UP_FORWARD {
            up_charge_time[entry_id] += 1;
            up_charge_buffer_time[entry_id] = 9;

        }
        else {
            if up_charge_buffer_time[entry_id] > 0 {
                up_charge_buffer_time[entry_id] -= 1;
            }
            else if up_charge_time[entry_id] != 0 {
        
                up_charge_time[entry_id] = 0;
        
            }
        }

        if dir == InputDirection::FORWARD || dir == InputDirection::UP_FORWARD || dir == InputDirection::DOWN_FORWARD {
            forward_charge_time[entry_id] += 1;
            forward_charge_buffer_time[entry_id] = 9;
        }
        else {
            if forward_charge_buffer_time[entry_id] > 0 {
            forward_charge_buffer_time[entry_id] -= 1;
            }
            else if forward_charge_time[entry_id] != 0 {
                forward_charge_time[entry_id] = 0;
            }
        }

        inc_specific_charge_time(fighter.module_accessor, dir);
        
    }
}

//todo move charge input fn's to charge input module

//todo add a perfect input varient for when the button is released
