use {
    smash::{
        lua2cpp::*, 
        phx::*,
        app::{ lua_bind::*, sv_animcmd::*, * }, 
        lib::{ lua_const::*, L2CAgent, L2CValue },
        hash40
    },
    super::{ *, InputType, StickType, CommandInputModule::{ *, InputDirection::*  }}, 
    smash_script::*, 
    smashline::{ locks::*, Priority::*, * }, 
    std::{ any::type_name, usize },
}; 

#[derive(PartialEq, Debug, Clone)]

struct per_input {

    step: u8,
    life: u8,
    charge_time: u8,
    requier_manual_input_kill: bool,
    regress_with_failed_input: bool,
    stick_type: StickType

}
#[derive(PartialEq, Debug, Clone)]

struct per_dir {

    direction: Vec<InputDirection>,
    button: Option<Vec<i32>>,
    input_type: InputType,
    allow_extra_frame: Option<bool>,
    allow_negative_edge: Option<bool>,
    allow_c_stick_input: Option<bool>,
    require_multiple_pressed_inputs: bool,
    strict: bool,
    defualt_life: u8,
    required_charge_time: u8

}

const defualt_life: u8 = 9;
const defualt_charge_time: u8 = 24;

static mut player_1_per_input_vec: Vec<per_input> = Vec::new();
static mut player_2_per_input_vec: Vec<per_input> = Vec::new();
static mut player_3_per_input_vec: Vec<per_input> = Vec::new();
static mut player_4_per_input_vec: Vec<per_input> = Vec::new();
static mut player_5_per_input_vec: Vec<per_input> = Vec::new();
static mut player_6_per_input_vec: Vec<per_input> = Vec::new();
static mut player_7_per_input_vec: Vec<per_input> = Vec::new();
static mut player_8_per_input_vec: Vec<per_input> = Vec::new();
    

static mut player_1_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_2_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_3_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_4_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_5_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_6_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_7_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();
static mut player_8_per_direction_vec: Vec<Vec<per_dir>> = Vec::new();

static mut player_1_last_frame: i32 = 0;
static mut player_2_last_frame: i32 = 0;
static mut player_3_last_frame: i32 = 0;
static mut player_4_last_frame: i32 = 0;
static mut player_5_last_frame: i32 = 0;
static mut player_6_last_frame: i32 = 0;
static mut player_7_last_frame: i32 = 0;
static mut player_8_last_frame: i32 = 0;

/// Adds a charge input to the Moveset
/// 
/// # Arguments
/// 
/// * `entry_id` - a pointer to what fighter you are using `usize`
/// 
/// * `vec` - a `Vec` containing a `Vec` of `InputDirections` to check for each step of the input (Null can be used for inputs where you dont need to check a direction)
/// 
/// ```
///     use inputmodule::{*, CommandInputModule::{*, InputDirection::*}};
/// 
///     unsafe extern "C" fighter_init(fighter: &mut L2CFighterCommon) {
///         
///         //adds the sonic boom input ([4]6+( attack || special ))
///         ChargeInputModule::add_charge(fighter.entry_id, [[DOWN_BACK, BACK, UP_BACK].to_vec(), [FORWARD].to_vec()].to_vec());
///         ChargeInputModule::add_button(fighter.entry_id, 0, 0, [*CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_SPECIAL].to_vec(), trigger, None, None, None);
///         
///     }
/// ```
pub unsafe fn add_charge(entry_id: usize, mut vec: Vec<Vec<InputDirection>>) {
    
    vec.push([NULL].to_vec());
    let per_dir_v = get_per_dir_vec(&entry_id);
    let per_input_v = get_per_input_vec(&entry_id);
    let index = per_dir_v.len();

    per_dir_v.push(Vec::new());

    for i in 0..vec.len() {

        let charge = if i == vec.len() - 2 { 0 } else { defualt_charge_time };
        let blank_dir: per_dir = per_dir { 
            direction: vec[i].clone(), 
            button: None, 
            input_type: InputType::none, 
            allow_extra_frame: None, 
            allow_negative_edge: None, 
            allow_c_stick_input: None, 
            require_multiple_pressed_inputs: false, 
            strict: false, 
            defualt_life: defualt_life, 
            required_charge_time: charge 
        };

        per_dir_v[index].push(blank_dir);


    }

    let blank_input:per_input = per_input { 
        step: 0, 
        life: defualt_life, 
        charge_time: defualt_charge_time,
        requier_manual_input_kill: false,
        regress_with_failed_input: false,
        stick_type: StickType::control_stick_only
    };

    //so fun fact to anyone reading my repo i forgot this line of code durring the first test of the module cuasing a fun bit of debugging
    per_input_v.push(blank_input);



}
/// Adds a button to a specific step of the input 
/// 
/// # Arguments
/// 
/// * `entry_id` - a pointer to what fighter you are using `usize`
/// 
/// * `input` - an index that points to the motion input being edited `usize`
/// 
/// * `step` - an index that points to the step of the motion input being edited `usize`
/// 
/// * `buttons` - a `Vec` of `CONTROL_PAD_BUTTON`s to check on that step
/// 
/// * `input_type` - a `InputType` that tells the module how it needs to check the buttons
/// 
/// * `allow_extra_frame` - a `Option<bool>` used for `InputType::perfect` that determines if the input can be delayed by a frame 
/// 
/// * `allow_negative_edge` - a `Option<bool>` used for `InputType::perfect` that determines if the input can be triggered by releasing the input on a perfect frame 
///
/// * `allow_c_stick_input` - a `Option<bool>` used for `InputType::perfect` that determines if the input can use the c stick as a perfect input macro
/// 
/// # Example
/// 
/// ```
///  use inputmodule::{*, InputType::*, CommandInputModule::{*, InputDirection::*}}
///
///  //adds a input simmaler to x's buster charge from the mvc games
///  ChargeInputModule::add_charge(fighter.entry_id, [[NULL].to_vec(), [NULL].to_vec(), [NULL].to_vec()].to_vec());
///  ChargeInputModule::add_button(fighter.entry_id, 0, 0, [*CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_SPECIAL], on, None, None, None);
///  ChargeInputModule::set_charge_time(fighter.entry_id, 0, 0, 30);
///  ChargeInputModule::add_button(fighter.entry_id, 0, 1, [*CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_SPECIAL], on, None, None, None);
///  ChargeInputModule::set_charge_time(fighter.entry_id, 0, 1, 40);
///  ChargeInputModule::add_button(fighter.entry_id, 0, 2, [*CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_SPECIAL], on, None, None, None);
///  ChargeInputModule::set_charge_time(fighter.entry_id, 0, 2, 55);
///  ChargeInputModule::requier_manual_input_kill(fighter.entry_id, 0);
///  ChargeInputModule::regress_with_failed_input(fighter.entry_id, 0);
///  
/// ```
pub unsafe fn add_button(entry_id: usize, input: usize, step: usize, buttons: Vec<i32>, input_type: InputType, allow_extra_frame: Option<bool>, allow_negative_edge: Option<bool>, allow_c_stick_input: Option<bool>) {

    let per_dir = get_per_dir_vec(&entry_id);

    
    per_dir[input][step].button = Some(buttons);
    per_dir[input][step].input_type = input_type;
    per_dir[input][step].allow_extra_frame = allow_extra_frame;
    per_dir[input][step].allow_negative_edge = allow_negative_edge;
    per_dir[input][step].allow_c_stick_input = allow_c_stick_input;


}

/// Makes it so if the stick or the buttons are not correct but the other is the input resets
pub unsafe fn add_strict(entry_id: usize, input: usize, step: usize) {

    let per_dir_vec = get_per_dir_vec(&entry_id);

    per_dir_vec[input][step].strict = true;
}

/// Makes it so you need to hit all the buttons on a given step before you can progress to the next step
pub unsafe fn require_simultaneously_buttons(entry_id: usize, input: usize, step: usize) {

    let per_dir_vec = get_per_dir_vec(&entry_id);

    per_dir_vec[input][step].require_multiple_pressed_inputs = true;
}

/// Returns the current step of an input
pub unsafe fn get_step(entry_id: usize, input: usize) -> u8 {
    
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].step

}

/// Returns the current life of an input
pub unsafe fn get_life(entry_id: usize, input: usize) -> u8 {

    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].life

}

/// Returns the current charge time of an input
pub unsafe fn get_charge_time(entry_id: usize, input: usize) -> u8 {

    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].charge_time

}

/// Checks if the charge input is on the final step
pub unsafe fn is_complete(entry_id: usize, input: usize) -> bool {
    
    let per_input = get_per_input_vec(&entry_id);
    let per_dir = get_per_dir_vec(&entry_id);
    let step = per_input[input].step as usize;
    let final_step = per_dir[input].len() - 2;
    if step == final_step {

        return true

    }

    false
    
}
/// Sets a custom value for how long an input can exist before being reset
pub unsafe fn set_life(entry_id: usize, input: usize, new_life: u8) {

    let per_dir = get_per_dir_vec(&entry_id);

    for i in 0..per_dir[input].len() {

        per_dir[input][i].defualt_life = new_life;

    }
}

/// Sets a custom value for a specific step for how long an input can exist before being reset
pub unsafe fn set_specific_life(entry_id: usize, input: usize, step: usize, new_life: u8) {

    let per_dir = get_per_dir_vec(&entry_id);

    per_dir[input][step].defualt_life = new_life;

}

/// Sets a custom value for how long you need to hold an input before you can progress to the next
pub unsafe fn set_charge_time(entry_id: usize, input: usize, step: usize, new_charge_time: u8) {

    let per_dir = get_per_dir_vec(&entry_id);

    per_dir[input][step].required_charge_time = new_charge_time;

}

/// Makes it so when an input is not being held the charge time decrements by 1
pub unsafe fn regress_with_failed_input(entry_id: usize, input: usize) {
    
    let per_input = get_per_input_vec(&entry_id);

    per_input[input].regress_with_failed_input = true;
}

/// Resets the input manually
pub unsafe fn reset_input_step(module_accessor:*mut BattleObjectModuleAccessor, input: usize) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input = get_per_input_vec(&entry_id);
    
    per_input[input].charge_time = 0;
    per_input[input].life = 0;
    per_input[input].step = 0;
}

/// Makes it so the only way an input can be reset is with ChargeInputModule::reset_input_step
pub unsafe fn requier_manual_input_kill(entry_id: usize, input: usize) {

    let per_input = get_per_input_vec(&entry_id);

    per_input[input].requier_manual_input_kill = true;

}

/// Resets the data in the module so the next fighter can use it
pub unsafe fn reset_module(module_accessor:*mut BattleObjectModuleAccessor) {
    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir = get_per_dir_vec(&entry_id);
    let per_input = get_per_input_vec(&entry_id);
    let last_update = get_last_update_frame(&entry_id);

    *per_dir = Vec::new();
    *per_input = Vec::new();
    *last_update = 0;

}
/// Updates the life of each charge input
/// 
/// needs to be run once per frame
pub unsafe fn update_timers(module_accessor:*mut BattleObjectModuleAccessor) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input = get_per_input_vec(&entry_id);
    let per_dir = get_per_dir_vec(&entry_id);
    let stick_dir = CommandInputModule::get_stick_dir(module_accessor);

    for index in 0 .. per_input.len() {

        let step = per_input[index].step as usize;

        if per_input[index].life > 0 && !per_dir[index][step].direction.contains(&stick_dir) {

            per_input[index].life -= 1;

        }
        else if per_dir[index][step].direction.contains(&stick_dir) {

            let new_life = per_dir[index][step].defualt_life;
            per_input[index].life = new_life;

        }
        
    }
}

/// Changes which control stick can update the charge inputs
/// 
/// by default its set to control_stick_only
pub unsafe fn set_stick_type(entry_id: usize, input: usize, new_stick_type: StickType) {
    
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].stick_type = new_stick_type;

}

///updates all the charge inputs for that frame 
pub unsafe fn update_module(module_accessor:*mut BattleObjectModuleAccessor, frame: i32, ignore_repeat_frame: bool) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let last_update_frame = get_last_update_frame(&entry_id);
    

    if frame == *last_update_frame && !ignore_repeat_frame {


        return;

    }
    else {

        *last_update_frame = frame;

    }

    let per_dir = get_per_dir_vec(&entry_id);
    let per_input = get_per_input_vec(&entry_id);

    for input in 0 .. per_input.len() {

        let mut step = per_input[input].step as usize;
        let life = per_input[input].life;
        let max_step = per_dir[input].len() - 1;
        let dirs = per_dir[input][step as usize].direction.clone();
        let stick_dir = CommandInputModule::get_stick_dir(module_accessor);
        let is_missed_strict_timing = !check_charge(module_accessor, input) && check_buttons(module_accessor, input) || check_charge(module_accessor, input) && !check_buttons(module_accessor, input);
        let input_stick_type = per_input[input].stick_type;
        let is_cstick = (input_stick_type == StickType::c_stick_only && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) ) || input_stick_type != StickType::c_stick_only;
        let is_main_stick = (input_stick_type == StickType::control_stick_only && !ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)) || input_stick_type != StickType::control_stick_only;

        if input_stick_type == StickType::both || is_cstick && is_main_stick {

            if life == 0 && !per_input[input].requier_manual_input_kill && ( !is_complete(entry_id, input) || is_complete(entry_id, input) && CancelModule::is_enable_cancel(module_accessor) ) {
                per_input[input].step = 0;
                per_input[input].charge_time = 0;

            }

            if per_dir[input][step].direction.contains(&stick_dir) || per_dir[input][step].direction.contains(&NULL) && step != max_step {

                if per_input[input].charge_time < per_dir[input][step].required_charge_time {
            
                    per_input[input].charge_time += 1;

                }
            }
            else if !(per_dir[input][step].direction.contains(&stick_dir) || per_dir[input][step].direction.contains(&NULL)) && per_input[input].regress_with_failed_input {
                // todo finish buster style charge inputs
            }

            if is_missed_strict_timing && per_dir[input][step].strict {

                per_input[input].life = 0;
                per_input[input].step = 0;
                per_input[input].charge_time = 0;

            }
            else if check_charge(module_accessor, input) && check_buttons(module_accessor, input) && step != max_step {
                if should_progress(module_accessor, input) {
                
                    step += 1;
                    per_input[input].step = step as u8;
                    let new_life = per_dir[input][step].defualt_life;
                    per_input[input].life = new_life;
                    per_input[input].charge_time = 0;

                }
            }
        }
    }
}

unsafe fn check_charge(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {
    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input = get_per_input_vec(&entry_id);
    let per_dir = get_per_dir_vec(&entry_id);
    let step = per_input[input].step as usize;
    let max_step = per_dir[input].len() - 1;

    if per_input[input].charge_time >= per_dir[input][step].required_charge_time {

        return true

    }

    false
}
unsafe fn check_buttons(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {


    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let per_input_vec = get_per_input_vec(&entry_id);

    let step = per_input_vec[input].step as usize;

    let input_type = per_dir_vec[input][step].input_type;
    let require_multiple_pressed_inputs = per_dir_vec[input][step].require_multiple_pressed_inputs;

    if input_type == InputType::none {


        return true

    }
    else {

    let buttons = per_dir_vec[input][step].button.clone().expect("could not find buttons to check");

    if require_multiple_pressed_inputs {

        /*

        this is gonna be painfull to code im srry future me 

        what needs to happen

            off 
                while input is off dont break(done)
            on 
                while input is on dont break(done)
            on_trigger
                if input is on_trigger && other inputs are on dont break
            on_release
                if input is on_release && other inputs are off dont break
            trigger
                if input is trigger && other inputs are on dont break
            release
                if input is release && other inputs are off dont break
            perfect
                if inputes are perfect dont break(done)
        */

        let mut ret = true;
        let mut should_continue = true;
        for button_index in 0..buttons.len() {
            if !ControlModule::check_button_off(module_accessor, buttons[button_index]) && input_type == InputType::off {

                ret = false;
                break;

            }
            else if !ControlModule::check_button_on(module_accessor, buttons[button_index]) && input_type == InputType::on {

                ret = false;
                break;

            }
            else if input_type == InputType::perfect {

                should_continue = false;
                let allow_extra_frame= per_dir_vec[input][step].allow_extra_frame.expect("could not find extra frame bool");
                let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge.expect("could not find negative edge bool");
                let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("could not find c-stick perfect bool");

                for dir_index in 0 .. per_dir_vec[input][step].direction.len() {
                    
                    let dir = per_dir_vec[input][step].direction[dir_index];
                    
                    if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                        should_continue = true;

                    }
                }

                if !should_continue {

                    ret = false;
                    break;

                }
            }
            else if input_type == InputType::trigger {
                
                should_continue = false;
                if 
                    !ControlModule::check_button_trigger(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_on(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::on_trigger {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_on(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::release {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_release(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::on_release {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_on_release(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
        }
        return ret
    }
    else {
        for button_index in 0 .. buttons.len() {
            
            if input_type == InputType::off && ControlModule::check_button_off(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::on && ControlModule::check_button_on(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::on_trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) {

                return true
                
            }
            else if input_type == InputType::on_release && ControlModule::check_button_on_release(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::release && ControlModule::check_button_release(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::perfect {

                let allow_extra_frame = per_dir_vec[input][step].allow_extra_frame.expect("unable to find extra frame bool");
                let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge.expect("unable to find negative edge bool");
                let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("unable to find c-stick input bool");

                for dir_index in 0 .. per_dir_vec[input][step].direction.len() {

                    let dir = per_dir_vec[input][step].direction[dir_index];

                    if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                        return true

                    }
                }
            }
        }
    }

    }
    

    false

}

unsafe fn should_progress(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input = get_per_input_vec(&entry_id);
    let per_dir = get_per_dir_vec(&entry_id);
    let step = (per_input[input].step + 1) as usize;
    let max_step = per_dir[input].len() - 1;
    let stick_dir = CommandInputModule::get_stick_dir(module_accessor);
    

    if step >= max_step {
        
        return false

    }

    let correct_stick = 
        if per_dir[input][step].direction.contains(&stick_dir) 
        || per_dir[input][step].direction.contains(&NULL) {true} 
        else {false};

    if correct_stick && check_next_buttons(module_accessor, input) {
        
        return true

    }
    else {

        false

    }

}

unsafe fn check_next_buttons(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {


    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let per_input_vec = get_per_input_vec(&entry_id);

    let step = (per_input_vec[input].step + 1) as usize;

    let input_type = per_dir_vec[input][step].input_type;
    let require_multiple_pressed_inputs = per_dir_vec[input][step].require_multiple_pressed_inputs;

    if input_type == InputType::none {


        return true

    }
    else {

    let buttons = per_dir_vec[input][step].button.clone().expect("could not find buttons to check");

    if require_multiple_pressed_inputs {

        /*

        this is gonna be painfull to code im srry future me 

        what needs to happen

            off 
                while input is off dont break(done)
            on 
                while input is on dont break(done)
            on_trigger
                if input is on_trigger && other inputs are on dont break
            on_release
                if input is on_release && other inputs are off dont break
            trigger
                if input is trigger && other inputs are on dont break
            release
                if input is release && other inputs are off dont break
            perfect
                if inputes are perfect dont break(done)
        */

        let mut ret = true;
        let mut should_continue = true;
        for button_index in 0..buttons.len() {
            if !ControlModule::check_button_off(module_accessor, buttons[button_index]) && input_type == InputType::off {

                ret = false;
                break;

            }
            else if !ControlModule::check_button_on(module_accessor, buttons[button_index]) && input_type == InputType::on {

                ret = false;
                break;

            }
            else if input_type == InputType::perfect {

                should_continue = false;
                let allow_extra_frame= per_dir_vec[input][step].allow_extra_frame.expect("could not find extra frame bool");
                let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge.expect("could not find negative edge bool");
                let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("could not find c-stick perfect bool");

                for dir_index in 0 .. per_dir_vec[input][step].direction.len() {
                    
                    let dir = per_dir_vec[input][step].direction[dir_index];
                    
                    if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                        should_continue = true;

                    }
                }

                if !should_continue {

                    ret = false;
                    break;

                }
            }
            else if input_type == InputType::trigger {
                
                should_continue = false;
                if 
                    !ControlModule::check_button_trigger(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_on(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::on_trigger {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_on(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::release {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_release(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
            else if input_type == InputType::on_release {
                
                should_continue = false;

                if 
                    !ControlModule::check_button_on_release(module_accessor, buttons[button_index]) && 
                    !ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                {
                    ret = false;
                    break;

                }
                else {
                    should_continue = true;
                }
            }
        }
        return ret
    }
    else {
        for button_index in 0 .. buttons.len() {
            
            if input_type == InputType::off && ControlModule::check_button_off(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::on && ControlModule::check_button_on(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::on_trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) {

                return true
                
            }
            else if input_type == InputType::on_release && ControlModule::check_button_on_release(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::release && ControlModule::check_button_release(module_accessor, buttons[button_index]) {

                return true

            }
            else if input_type == InputType::perfect {

                let allow_extra_frame = per_dir_vec[input][step].allow_extra_frame.expect("unable to find extra frame bool");
                let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge.expect("unable to find negative edge bool");
                let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("unable to find c-stick input bool");

                for dir_index in 0 .. per_dir_vec[input][step].direction.len() {

                    let dir = per_dir_vec[input][step].direction[dir_index];

                    if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                        return true

                    }
                }
            }
        }
    }

    }
    

    false

}

unsafe fn get_per_input_vec(entry_id: &usize) -> &mut Vec<per_input> {
    let ret = if *entry_id == 0 { &mut player_1_per_input_vec } else if *entry_id == 1 { &mut player_2_per_input_vec }
        else if *entry_id == 2 { &mut player_3_per_input_vec } else if *entry_id == 3 { &mut player_4_per_input_vec }
        else if *entry_id == 4 { &mut player_5_per_input_vec } else if *entry_id == 5 { &mut player_6_per_input_vec }
        else if *entry_id == 6 { &mut player_7_per_input_vec } else { &mut player_8_per_input_vec };

    ret
}
unsafe fn get_per_dir_vec(entry_id: &usize) -> &mut Vec<Vec<per_dir>> {
    let ret = if *entry_id == 0 { &mut player_1_per_direction_vec } else if *entry_id == 1 { &mut player_2_per_direction_vec }
        else if *entry_id == 2 { &mut player_3_per_direction_vec } else if *entry_id == 3 { &mut player_4_per_direction_vec }
        else if *entry_id == 4 { &mut player_5_per_direction_vec } else if *entry_id == 5 { &mut player_6_per_direction_vec }
        else if *entry_id == 6 { &mut player_7_per_direction_vec } else { &mut player_8_per_direction_vec };

    ret
}
unsafe fn get_last_update_frame(entry_id: &usize) -> *mut i32 {
    let ret = if *entry_id == 0 {&raw mut player_1_last_frame} else if *entry_id == 1 { &raw mut player_2_last_frame }
        else if *entry_id == 2 { &mut player_3_last_frame } else if *entry_id == 3 { &mut player_4_last_frame }
        else if *entry_id == 4 { &mut player_5_last_frame } else if *entry_id == 5 { &mut player_6_last_frame }
        else if *entry_id == 6 { &mut player_7_last_frame } else { &mut player_8_last_frame };

    ret
}
//todo ... make the f-ing module @games