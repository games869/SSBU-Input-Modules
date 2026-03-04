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
    defualt_life: u8,
    max_shortcuts: u8,
    stick_type: StickType

}
#[derive(PartialEq, Debug, Clone)]

struct per_dir {

    is_raw: bool,
    direction: Vec<InputDirection>,
    raw_direction: Vec<InputDirectionRaw>,
    button: Option<Vec<i32>>,
    input_type: InputType,
    allow_extra_frame: Option<bool>,
    allow_negative_edge: Option<bool>,
    allow_c_stick_input: Option<bool>,
    require_multiple_pressed_inputs: bool,
    strict: bool,
    can_shortcut: bool,

}


const defualt_life: u8 = 9;

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

/// Adds a new motion input to the character 
/// 
/// # Arguments
/// 
/// * `entry_id` - a pointer to what fighter you are using `usize`
/// 
/// * `vec` - a `Vec` containing a `Vec` of `InputDirections` to check for each step of the input (Null can be used for inputs where you dont need to check a direction)
/// 
/// # Exaple
/// 
/// ```
///     use inputmodule::{*, CommandInputModule::{*, InputDirection::*}};
/// 
///     unsafe extern "C" fighter_init(fighter: &mut L2CFighterCommon) {
///         
///         //adds geese howard's pretzel input (1632143)
///         MotionInputModule::add_motion(fighter.entry_id, [[DOWN_BACK].to_vec(), [FORWARD].to_vec(), [DOWN_FORWARD].to_vec(), [DOWN].to_vec(), [DOWN_BACK].to_vec(), [BACK].to_vec(), [DOWN_FORWARD].to_vec()].to_vec());
///         
///     }
/// ```
pub unsafe fn add_motion(entry_id: usize, mut vec: Vec<Vec<InputDirection>>) {

    vec.push([NULL].to_vec());
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let per_input_vec = get_per_input_vec(&entry_id);
    let index = per_dir_vec.len();

    per_dir_vec.push(Vec::new());

    for i in 0..vec.len() {

        let blank_dir: per_dir = per_dir { 
            is_raw: false,
            direction: vec[i].clone(), 
            raw_direction: Vec::new(),
            button: None, 
            input_type: InputType::none, 
            allow_extra_frame: None, 
            allow_negative_edge: None, 
            allow_c_stick_input: None,
            require_multiple_pressed_inputs: false,
            strict: false,
            can_shortcut: false 
        };

        per_dir_vec[index].push(blank_dir);
        
        
    }

    let blank_input = per_input {
        defualt_life,
        life: defualt_life,
        step: 0,
        max_shortcuts: 1,
        stick_type: StickType::control_stick_only
    };
    per_input_vec.push(blank_input);


}

/// Adds a new motion input to the character 
/// 
/// # Arguments
/// 
/// * `entry_id` - a pointer to what fighter you are using `usize`
/// 
/// * `vec` - a `Vec` containing a `Vec` of `InputDirectionRaw`s to check for each step of the input (Null can be used for inputs where you dont need to check a direction)
/// 
/// # Exaple
/// 
/// ```
///     use inputmodule::{*, CommandInputModule::{*, InputDirectionRaw::*}};
/// 
///     unsafe extern "C" fighter_init(fighter: &mut L2CFighterCommon) {
///         
///        
///         MotionInputModule::add_raw_motion(fighter.entry_id, [[DOWN].to_vec(), [DOWN_LEFT].to_vec(), [LEFT].to_vec());
///         
///     }
/// ```
pub unsafe fn add_raw_motion(entry_id: usize, mut vec: Vec<Vec<InputDirectionRaw>>) {

    vec.push([InputDirectionRaw::NULL].to_vec());
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let per_input_vec = get_per_input_vec(&entry_id);
    let index = per_dir_vec.len();

    per_dir_vec.push(Vec::new());

    for i in 0..vec.len() {

        let blank_dir: per_dir = per_dir { 
            is_raw: true,
            direction: Vec::new(),
            raw_direction: vec[i].clone(), 
            button: None, 
            input_type: InputType::none, 
            allow_extra_frame: None, 
            allow_negative_edge: None, 
            allow_c_stick_input: None,
            require_multiple_pressed_inputs: false,
            strict: false,
            can_shortcut: false 
        };

        per_dir_vec[index].push(blank_dir);
        
        
    }

    let blank_input = per_input {
        defualt_life,
        life: defualt_life,
        step: 0,
        max_shortcuts: 1,
        stick_type: StickType::control_stick_only
    };
    per_input_vec.push(blank_input);


}

/// Resets an input manually 
pub unsafe fn reset_input_step(module_accessor:*mut BattleObjectModuleAccessor, input: usize) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].step = 0;
    per_input_vec[input].life = 0;

}

/// Clears all the data in the Module so its ready for the next fighter
pub unsafe fn reset_module(module_accessor:*mut BattleObjectModuleAccessor) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let per_input_vec = get_per_input_vec(&entry_id);
    let last_update_frame = get_last_update_frame(&entry_id);

    *per_dir_vec = Vec::new();
    *per_input_vec = Vec::new();
    *last_update_frame = 0;
        
}

/// Changes how long the input can go without a new step before its reset defualt is 9
/// 
/// for raging demon style inputs its best to change this to 20 
pub unsafe fn change_life(entry_id: usize, input: usize, new_life: u8) {

    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].defualt_life = new_life;

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
/// use inputmodule::{*, InputType::*, CommandInputModule::{*, InputDirection::*}}
/// 
/// //adds the raging demon input
/// MotionInputModule::add_motion(fighter.entry_id, [[NULL].to_vec(), [NULL].to_vec(), [FORWARD].to_vec(), [NULL].to_vec(), [NULL].to_vec()].to_vec());
/// MotionInputModule::add_button(fighter.entry_id, 0, 0, [*CONTROL_PAD_BUTTON_ATTACK].to_vec(), trigger, None, None, None);
/// MotionInputModule::add_button(fighter.entry_id, 0, 1, [*CONTROL_PAD_BUTTON_ATTACK].to_vec(), trigger, None, None, None);
/// MotionInputModule::add_button(fighter.entry_id, 0, 3, [*CONTROL_PAD_BUTTON_ATTACK].to_vec(), trigger, None, None, None);
/// MotionInputModule::add_button(fighter.entry_id, 0, 4, [*CONTROL_PAD_BUTTON_SPECIAL].to_vec(), trigger, None, None, None);
/// MotionInputModule::allow_shortcut(fighter.entry_id, 0, 2);
/// MotionInputModule::set_max_shortcuts(fighter.entry_id, 0, 2);
/// MotionInputModule::change_life(fighter.entry_id, 0, 20);
/// ``` 

pub unsafe fn add_button(entry_id: usize, input: usize, step: usize, buttons: Vec<i32>, input_type: InputType, allow_extra_frame: Option<bool>, allow_negative_edge: Option<bool>, allow_c_stick_input: Option<bool>) {
        
    let per_dir_vec = get_per_dir_vec(&entry_id);

    per_dir_vec[input][step].button = Some(buttons);
    per_dir_vec[input][step].input_type = input_type;
    per_dir_vec[input][step].allow_extra_frame = allow_extra_frame;
    per_dir_vec[input][step].allow_negative_edge = allow_negative_edge;
    per_dir_vec[input][step].allow_c_stick_input = allow_c_stick_input;

}

/// Makes it so all the buttons pervided must be pressed for the input to complete
/// 
/// if you use this its best if you use the RAW version of the `CONTROL_PAD_BUTTON`s
pub unsafe fn require_simultaneously_buttons(entry_id: usize, input: usize, step: usize) {

    let per_dir_vec = get_per_dir_vec(&entry_id);

    per_dir_vec[input][step].require_multiple_pressed_inputs = true;
}

/// Makes it so that if the motion is done and the buttons are not pressed or vise versa the input will reset
pub unsafe fn add_strict(entry_id: usize, input: usize, step: usize) {

    let per_dir_vec = get_per_dir_vec(&entry_id);

    per_dir_vec[input][step].strict = true;
}

/// Sets the total amount of inputs that can be done in 1 frame
/// 
/// Defualts to 1 and wont allow any shortcutting
pub unsafe fn set_max_shortcuts(entry_id: usize, input: usize, new_max_shortcuts: u8) {

    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].max_shortcuts = new_max_shortcuts;

}

/// Allows the input to check the next input in the series on the same frame
pub unsafe fn allow_shortcut(entry_id: usize, input: usize, steps: Vec<usize>) {

    let per_dir_vec = get_per_dir_vec(&entry_id);
    
    for step in steps {
        
        per_dir_vec[input][step].can_shortcut = true;

    }
}

/// Returns what step the given input is on as a `u8`
pub unsafe fn get_step(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> u8 {
    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].step

}

/// Returns the life of the given input as a `u8`
pub unsafe fn get_life(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> u8 {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].life

}

/// Returns if the given input is finished as a `bool`
pub unsafe fn is_complete(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {
    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let step = per_input_vec[input].step as usize;
    let final_step = per_dir_vec[input].len() - 1;

    if step == final_step {

        return true

    }

    false
    
}

/// Changes which control stick can update the motion inputs
/// 
/// by default its set to control_stick_only
pub unsafe fn set_stick_type(entry_id: usize, input: usize, new_stick_type: StickType) {
    
    let per_input_vec = get_per_input_vec(&entry_id);

    per_input_vec[input].stick_type = new_stick_type;

}

/// Updates everything in the Module for that frame 
/// 
/// Best to run this before the inputs are checked and then again in a frame
/// 
/// # Example 
/// ```
///     if !StatusModule::is_changing(fighter.module_accessor) {
///         let frame = fighter.global_table[0xE].get_i32();
///         MotionInputModule::update_timers(fighter.module_accessor);
///         MotionInputModule::update_module(fighter.module_accessor, frame, false);
///     }
/// ``` 
pub unsafe fn update_module(module_accessor:*mut BattleObjectModuleAccessor, frame: i32, ignore_repeat_frame: bool) {

    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let last_update_frame = get_last_update_frame(&entry_id);
    

    if frame == *last_update_frame && !ignore_repeat_frame {


        return;

    }
    else {

        *last_update_frame = frame;

    }

    
    let per_input_vec = get_per_input_vec(&entry_id);
    let per_dir_vec = get_per_dir_vec(&entry_id);


    for inputs in 0 .. per_input_vec.len() {

        let max_shortcuts = per_input_vec[inputs].max_shortcuts;
        let input_stick_type = per_input_vec[inputs].stick_type;
        let is_cstick = (input_stick_type == StickType::c_stick_only && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) ) || input_stick_type != StickType::c_stick_only;
        let is_main_stick = (input_stick_type == StickType::control_stick_only && !ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)) || input_stick_type != StickType::control_stick_only;

        if input_stick_type == StickType::both || is_cstick && is_main_stick {
            
            for _ in 0 .. max_shortcuts {


                let step = per_input_vec[inputs].step;
                let max_step = per_dir_vec[inputs].len() - 1;
                let is_raw_input = per_dir_vec[inputs][step as usize].is_raw;
                let dirs = per_dir_vec[inputs][step as usize].direction.clone();
                let raw_dirs = per_dir_vec[inputs][step as usize].raw_direction.clone();
                let input_type = per_dir_vec[inputs][step as usize].input_type;
                let require_multiple_pressed_inputs = per_dir_vec[inputs][step as usize].require_multiple_pressed_inputs;
                let is_missed_strict_timing = is_motion_correct(module_accessor, dirs.clone(), raw_dirs.clone(), is_raw_input, inputs) && !is_buttons_correct(module_accessor, inputs) || !is_motion_correct(module_accessor, dirs.clone(), raw_dirs.clone(), is_raw_input, inputs) && is_buttons_correct(module_accessor, inputs);

                if per_input_vec[inputs].life == 0 && ( !is_complete(module_accessor, inputs) || is_complete(module_accessor, inputs) && CancelModule::is_enable_cancel(module_accessor) ) {

            
                    per_input_vec[inputs].step = 0;

                }

                if is_missed_strict_timing && per_dir_vec[inputs][step as usize].strict {

                    per_input_vec[inputs].step = 0;
                    per_input_vec[inputs].life = 0;

                }
                else if is_motion_correct(module_accessor, dirs, raw_dirs.clone(), is_raw_input, inputs) && is_buttons_correct(module_accessor, inputs) && step as usize != max_step {

                    let new_life = per_input_vec[inputs].defualt_life;

                    per_input_vec[inputs].step += 1;
                    per_input_vec[inputs].life = new_life;


                    if !per_dir_vec[inputs][step as usize].can_shortcut {

                        break;

                    }
                }
                else {

                    break;

                }
            }
        }
    }
}

/// Updates the life of every input
pub unsafe fn update_timers(module_accessor:*mut BattleObjectModuleAccessor) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);

    for index in 0 .. per_input_vec.len() {
        if per_input_vec[index].life > 0 {

            per_input_vec[index].life -= 1;

        }
    }
}

unsafe fn is_motion_correct(module_accessor:*mut BattleObjectModuleAccessor, motion_vec: Vec<InputDirection>, raw_motion_vec: Vec<InputDirectionRaw>, is_raw: bool, input: usize) -> bool {

    let input_dir = CommandInputModule::get_stick_dir(module_accessor);
    let raw_input_dir = CommandInputModule::get_stick_dir_raw(module_accessor);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = get_per_input_vec(&entry_id);
    let per_dir_vec = get_per_dir_vec(&entry_id);
    let raw_null = InputDirectionRaw::NULL;

    if !is_raw && motion_vec.contains(&input_dir) || is_raw && raw_motion_vec.contains(&raw_input_dir) {

        return true

    }
    else if (!is_raw && motion_vec.contains(&NULL) || is_raw && raw_motion_vec.contains(&raw_null)) && get_step(module_accessor, input) as usize != per_dir_vec[input].len() {

        return true

    }

    false

}

unsafe fn is_buttons_correct(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> bool {


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