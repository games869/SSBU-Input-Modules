use {
    super::{InputType, StickType, CommandInputModule, InputDirection::{self, *}, InputDirectionRaw::{self}, ToDirVector, ToRawDirVector, ToVector}, 
    smash::{ app::{ lua_bind::*, * }, lib::lua_const::* },
    std::usize
}; 


#[derive(PartialEq, Debug, Clone)]

struct PerInput {

    step: u8,
    life: u8,
    default_life: u8,
    max_shortcuts: u8,
    stick_type: StickType

}
#[derive(PartialEq, Debug, Clone)]

struct PerDir {

    is_raw: bool,
    direction: Vec<InputDirection>,
    raw_direction: Vec<InputDirectionRaw>,
    button: Option<Vec<i32>>,
    input_type: InputType,
    allow_extra_frame: Option<bool>,
    allow_negative_edge: bool,
    allow_c_stick_input: Option<bool>,
    require_multiple_pressed_inputs: bool,
    strict: bool,
    can_shortcut: bool,

}

const DEFAULT_LIFE: u8 = 9;

static mut MOTION_INPUT_STORAGE: [(Vec<PerInput>, Vec<Vec<PerDir>>, f32); 8] = [
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0),
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0), 
    (Vec::new(), Vec::new(), 0.0)
];

unsafe fn is_input_index_safe(entry_id: usize, input: usize, should_panic: bool, fn_name: &str) -> bool {

    let per_input = &MOTION_INPUT_STORAGE[entry_id].0;

    if input >= per_input.len() {
        let crash_msg = String::from("[inputmodule::MotionInputModule::") + fn_name +"] Error:\nfn has bad arguments\n\ninput len = (" + &per_input.len().to_string() + ") but the index is (" + &input.to_string() + ").\0";
        
        if should_panic { 

            skyline::error::show_error(89, "inputmodule error, press Details.\0", &crash_msg);
            skyline::nn::oe::ExitApplication(); 
            
        }
        else { eprintln!("{}", crash_msg); }

        return false

    }

    true
}

unsafe fn is_step_index_safe(entry_id: usize, input: usize, step: usize, should_panic: bool, fn_name: &str) -> bool {

    let per_dir = &MOTION_INPUT_STORAGE[entry_id].1;

    if step >= per_dir[input].len() {
        let crash_msg = String::from("[inputmodule::MotionInputModule::") + fn_name + "] Error:\nfn has bad arguments\n\ninput (" + &input.to_string() + ") step len = (" + &per_dir[input].len().to_string() + ") but the index is (" + &step.to_string() + ").\0";
        
        if should_panic { 

            skyline::error::show_error(90, "inputmodule error, press Details.\0", &crash_msg);
            skyline::nn::oe::ExitApplication(); 
            
        }
        else { eprintln!("{}", crash_msg); }

        return false
        
    }

    true
}

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
pub unsafe fn add_motion<V: ToDirVector + Clone >(entry_id: usize, vec: Vec<V>) {
    
    let mut new_vec: Vec<Vec<InputDirection>> = vec![];

    for i in 0..vec.len() {

        new_vec.push(vec[i].clone().into_vec());

    }

    new_vec.push([NULL].to_vec());
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;
    let index = per_dir_vec.len();

    per_dir_vec.push(Vec::new());

    for i in 0..new_vec.len() {

        let blank_dir: PerDir = PerDir { 
            is_raw: false,
            direction: new_vec[i].clone(), 
            raw_direction: Vec::new(),
            button: None, 
            input_type: InputType::None, 
            allow_extra_frame: None, 
            allow_negative_edge: false, 
            allow_c_stick_input: None,
            require_multiple_pressed_inputs: false,
            strict: false,
            can_shortcut: false 
        };

        per_dir_vec[index].push(blank_dir);
        
        
    }

    let blank_input = PerInput {
        default_life: DEFAULT_LIFE,
        life: 0,
        step: 0,
        max_shortcuts: 1,
        stick_type: StickType::Control_Stick_Only
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
pub unsafe fn add_raw_motion<V: ToRawDirVector + Clone>(entry_id: usize, vec: Vec<V>) {

    let mut new_vec: Vec<Vec<InputDirectionRaw>> = vec![];

    for i in 0..vec.len() {

        new_vec.push(vec[i].clone().into_raw_vec());

    }

    new_vec.push([InputDirectionRaw::NULL].to_vec());
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;
    let index = per_dir_vec.len();

    per_dir_vec.push(Vec::new());

    for i in 0..new_vec.len() {

        let blank_dir: PerDir = PerDir { 
            is_raw: true,
            direction: Vec::new(),
            raw_direction: new_vec[i].clone(), 
            button: None, 
            input_type: InputType::None, 
            allow_extra_frame: None, 
            allow_negative_edge: false, 
            allow_c_stick_input: None,
            require_multiple_pressed_inputs: false,
            strict: false,
            can_shortcut: false 
        };

        per_dir_vec[index].push(blank_dir);
        
    }

    let blank_input = PerInput {
        default_life: DEFAULT_LIFE,
        life: DEFAULT_LIFE,
        step: 0,
        max_shortcuts: 1,
        stick_type: StickType::Control_Stick_Only
    };

    per_input_vec.push(blank_input);

}

/// Resets an input manually 
pub unsafe fn reset_input_step<V: ToVector>(module_accessor:*mut BattleObjectModuleAccessor, input: V) {

    let inputs = input.as_usize_vec();
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

    for i in inputs {
        if !is_input_index_safe(entry_id, i, false, "reset_input_step") { continue; }

        per_input_vec[i].step = 0;
        per_input_vec[i].life = 0;
    }
}

/// Clears all the data in the Module so its ready for the next fighter
pub unsafe fn reset_module(module_accessor:*mut BattleObjectModuleAccessor) {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;
    let last_update_frame = &mut MOTION_INPUT_STORAGE[entry_id].2;

    *per_dir_vec = Vec::new();
    *per_input_vec = Vec::new();
    *last_update_frame = 0.0;
        
}

/// Changes how long the input can go without a new step before its reset default is 9
/// 
/// for raging demon style inputs its best to change this to 20 
pub unsafe fn change_life<V: ToVector>(entry_id: usize, input: V, new_life: u8) {

    let inputs = input.as_usize_vec();

    for i in inputs {
        if !is_input_index_safe(entry_id, i, true, "change_life") { return; }

        let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

        per_input_vec[i].default_life = new_life;
    }
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
/// MotionInputModule::add_motion(fighter.entry_id, [NULL, NULL, FORWARD, NULL, NULL].to_vec());
/// MotionInputModule::add_button(fighter.entry_id, 0, [0, 1, 3], [*CONTROL_PAD_BUTTON_ATTACK].to_vec(), trigger, None, None, None);
/// MotionInputModule::add_button(fighter.entry_id, 0, 4, [*CONTROL_PAD_BUTTON_SPECIAL].to_vec(), trigger, None, None, None);
/// MotionInputModule::allow_shortcut(fighter.entry_id, 0, 2);
/// MotionInputModule::set_max_shortcuts(fighter.entry_id, 0, 2);
/// MotionInputModule::change_life(fighter.entry_id, 0, 20);
/// ``` 

pub unsafe fn add_button<V: ToVector + Clone, B: ToVector + Clone>(entry_id: usize, input: usize, step: V, buttons: Vec<B>, input_type: InputType, allow_negative_edge: bool, allow_extra_frame: Option<bool>, allow_c_stick_input: Option<bool>) {

    if !is_input_index_safe(entry_id, input, true, "add_button") { return; }

    let steps = step.as_usize_vec();

    if steps.len() != buttons.len() && buttons.len() != 1 { 
        
        let crash_msg = String::from("[inputmodule::MotionInputModule::add_button] Error:\nfn has bad arguments\n\nstep len = (") + &steps.len().to_string() + ") but buttons len = " + &buttons.len().to_string() + ")\n\nbuttons should have the same length as step or 1 for it to be copyed to all steps";
        skyline::error::show_error(89, "inputmodule error, press Details.\0", &crash_msg);
        skyline::nn::oe::ExitApplication(); 

    }

    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;

    for i in 0..steps.len() {

        if !is_step_index_safe(entry_id, input, steps[i], true, "add_button") { return; }

        let index =
            if buttons.len() == 1 { 0 }
            else { i }
        ;

        per_dir_vec[input][steps[i]].button = Some(buttons[index].clone().as_int_vec());
        per_dir_vec[input][steps[i]].input_type = input_type;
        per_dir_vec[input][steps[i]].allow_extra_frame = allow_extra_frame;
        per_dir_vec[input][steps[i]].allow_negative_edge = allow_negative_edge;
        per_dir_vec[input][steps[i]].allow_c_stick_input = allow_c_stick_input;

    }
}

/// Makes it so all the buttons pervided must be pressed for the input to complete
/// 
/// if you use this its best if you use the RAW version of the `CONTROL_PAD_BUTTON`s
pub unsafe fn require_simultaneously_buttons<V: ToVector>(entry_id: usize, input: usize, step: V) {

    if !is_input_index_safe(entry_id, input, true, "require_simultaneously_buttons") { return; }

    let steps = step.as_usize_vec();

    for i in steps {

        if !is_step_index_safe(entry_id, input, i, true, "require_simultaneously_buttons"){ return; }

        let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;

        per_dir_vec[input][i].require_multiple_pressed_inputs = true;

    }
}

/// Makes it so that if the motion is done and the buttons are not pressed or vise versa the input will reset
pub unsafe fn add_strict<V: ToVector>(entry_id: usize, input: usize, step: V) {

    if !is_input_index_safe(entry_id, input, true, "add_strict") { return; }

    let steps = step.as_usize_vec();

    for i in steps {

        if !is_step_index_safe(entry_id, input, i, true, "add_strict") { return; }
        let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;

        per_dir_vec[input][i].strict = true;

    }
}

/// Sets the total amount of inputs that can be done in 1 frame
/// 
/// Defaults to 1 and wont allow any shortcutting
pub unsafe fn set_max_shortcuts<V: ToVector>(entry_id: usize, input: V, new_max_shortcuts: u8) {

    let inputs = input.as_usize_vec();

    for i in inputs {
        
        if !is_input_index_safe(entry_id, i, true, "set_max_shortcuts") { return; }

        let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

        per_input_vec[i].max_shortcuts = new_max_shortcuts;
    }
}

/// Allows the input to check the next input in the series on the same frame
pub unsafe fn allow_shortcut<V: ToVector>(entry_id: usize, input: usize, step: V) {

    if !is_input_index_safe(entry_id, input, true, "allow_shortcut") { return; }

    let steps = step.as_usize_vec();
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;
    
    for i in steps {

        if !is_step_index_safe(entry_id, input, i, true, "allow_shortcut") { return; }
        
        per_dir_vec[input][i].can_shortcut = true;

    }
}

/// Returns what step the given input is on as a `u8`
pub unsafe fn get_step(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> u8 {
    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

    if !is_input_index_safe(entry_id, input, false, "get_step") { return 0; }

    per_input_vec[input].step

}

/// Returns the life of the given input as a `u8`
pub unsafe fn get_life(module_accessor:*mut BattleObjectModuleAccessor, input: usize) -> u8 {

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

    if !is_input_index_safe(entry_id, input, false, "get_life") { return 0; }

    per_input_vec[input].life

}

/// Returns if the given input is finished as a `bool`
pub unsafe fn is_complete<V: ToVector>(module_accessor:*mut BattleObjectModuleAccessor, input: V) -> bool {
    
    let inputs = input.as_usize_vec();
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;

    for i in 0..inputs.len() {
        let step = per_input_vec[inputs[i]].step as usize;
        let final_step = per_dir_vec[inputs[i]].len() - 1;

        if !is_input_index_safe(entry_id, inputs[i], false, "is_complete") { continue; }

        if step == final_step {

            return true

        }
    }

    false
    
}

/// Changes which control stick can update the motion inputs
/// 
/// by default its set to control_stick_only
pub unsafe fn set_stick_type<V: ToVector>(entry_id: usize, input: V, new_stick_type: StickType) {

    let inputs = input.as_usize_vec();

    for i in inputs {
        if !is_input_index_safe(entry_id, i, true, "set_stick_type") { return; }
    
        let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

        per_input_vec[i].stick_type = new_stick_type;
    }
}

/// Updates everything in the Module for that frame 
/// 
/// Best to run this before the inputs are checked and then again in a frame
/// 
/// # Example 
/// ```
///     if !StatusModule::is_changing(fighter.module_accessor) {
///         let frame = fighter.global_table[0xE].get_f32();
///         MotionInputModule::update_timers(fighter.module_accessor);
///         MotionInputModule::update_module(fighter.module_accessor, frame, false);
///     }
/// ``` 
pub unsafe fn update_module(module_accessor:*mut BattleObjectModuleAccessor, frame: f32, ignore_repeat_frame: bool) {

    
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let last_update_frame = &mut MOTION_INPUT_STORAGE[entry_id].2;
    

    if frame == *last_update_frame && !ignore_repeat_frame {


        return;

    }
    else {

        *last_update_frame = frame;

    }

    
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;


    for inputs in 0 .. per_input_vec.len() {

        let max_shortcuts = per_input_vec[inputs].max_shortcuts;
        let input_stick_type = per_input_vec[inputs].stick_type;
        let is_cstick = (input_stick_type == StickType::C_Stick_Only && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) ) || input_stick_type != StickType::C_Stick_Only;
        let is_main_stick = (input_stick_type == StickType::Control_Stick_Only && !ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)) || input_stick_type != StickType::Control_Stick_Only;

        if input_stick_type == StickType::Both || is_cstick && is_main_stick {
            
            for _ in 0 .. max_shortcuts {


                let step = per_input_vec[inputs].step;
                let max_step = per_dir_vec[inputs].len() - 1;
                let is_raw_input = per_dir_vec[inputs][step as usize].is_raw;
                let dirs = per_dir_vec[inputs][step as usize].direction.clone();
                let raw_dirs = per_dir_vec[inputs][step as usize].raw_direction.clone();
                let is_missed_strict_timing = is_motion_correct(module_accessor, dirs.clone(), raw_dirs.clone(), is_raw_input, inputs) && !is_buttons_correct(module_accessor, inputs, step.into()) || !is_motion_correct(module_accessor, dirs.clone(), raw_dirs.clone(), is_raw_input, inputs) && is_buttons_correct(module_accessor, inputs, step.into());

                if per_input_vec[inputs].life == 0 && ( !is_complete(module_accessor, inputs) || is_complete(module_accessor, inputs) && CancelModule::is_enable_cancel(module_accessor) ) {

            
                    per_input_vec[inputs].step = 0;

                }

                if is_missed_strict_timing && per_dir_vec[inputs][step as usize].strict {

                    per_input_vec[inputs].step = 0;
                    per_input_vec[inputs].life = 0;

                }
                else if is_motion_correct(module_accessor, dirs, raw_dirs.clone(), is_raw_input, inputs) && is_buttons_correct(module_accessor, inputs, step.into()) && step as usize != max_step {

                    let new_life = per_input_vec[inputs].default_life;

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
    let per_input_vec = &mut MOTION_INPUT_STORAGE[entry_id].0;

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
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;
    let raw_null = InputDirectionRaw::NULL;

    if !is_raw && motion_vec.contains(&input_dir) || is_raw && raw_motion_vec.contains(&raw_input_dir) {

        return true

    }
    else if (!is_raw && motion_vec.contains(&NULL) || is_raw && raw_motion_vec.contains(&raw_null)) && get_step(module_accessor, input) as usize != per_dir_vec[input].len() {

        return true

    }

    false

}

unsafe fn is_buttons_correct(module_accessor:*mut BattleObjectModuleAccessor, input: usize, step: usize) -> bool {


    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let per_dir_vec = &mut MOTION_INPUT_STORAGE[entry_id].1;


    let input_type = per_dir_vec[input][step].input_type;
    let require_multiple_pressed_inputs = per_dir_vec[input][step].require_multiple_pressed_inputs;

    if input_type == InputType::None {

        return true

    }
    else {

        let buttons = per_dir_vec[input][step].button.clone().expect("could not find buttons to check");

        if !per_dir_vec[input][step].allow_negative_edge {
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

                for button_index in 0..buttons.len() {
                    let other_inputs_good =
                        if [InputType::Off, InputType::On, InputType::Perfect].contains(&input_type) { true }
                        else {
                            let mut ret = false;
                            
                            for other_index in 0..buttons.len() {
                                if button_index == other_index { continue; }

                                if 
                                    [InputType::Trigger, InputType::On_Trigger].contains(&input_type) && !ControlModule::check_button_on(module_accessor, buttons[other_index])
                                    || [InputType::Release, InputType::On_Release].contains(&input_type) && !ControlModule::check_button_off(module_accessor, buttons[other_index])
                                {

                                    break;

                                }
                                
                                if other_index < buttons.len() - 1 { continue; }
                            
                                    
                                ret = true;

                            }
                            

                            ret
                        }
                    ;
                    let skip: bool;
                    if 
                        input_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index])
                        || input_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || input_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index]) && other_inputs_good

                    {

                        skip = true;
                        
                    }
                    else if input_type == InputType::Perfect {

                        let mut should_continue = false;
                        let allow_extra_frame= per_dir_vec[input][step].allow_extra_frame.expect("could not find extra frame bool");
                        let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge;
                        let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("could not find c-stick perfect bool");

                        for dir_index in 0 .. per_dir_vec[input][step].direction.len() {
                    
                            let dir = per_dir_vec[input][step].direction[dir_index];
                    
                            if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                                should_continue = true;

                            }
                        }

                        skip = should_continue;

                    }
                    else {

                        skip = false;

                    }

                    if !skip {

                        ret = false;
                        break;
                    }
                }

                return ret

            }
            else {
                for button_index in 0 .. buttons.len() {
            
                    if 
                        input_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                        || input_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || input_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index])
                        || input_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index])
                        || input_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index])
                        || input_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index])
                    {

                       return true;

                    }
                    else if input_type == InputType::Perfect {

                        let allow_extra_frame = per_dir_vec[input][step].allow_extra_frame.expect("unable to find extra frame bool");
                        let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge;
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
        else {
            
            let negative_type = input_type.get_negative_instance();
            
            if !require_multiple_pressed_inputs {
                
                for button_index in 0 .. buttons.len() {
            
                    if 
                        input_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                        || input_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || input_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index])
                        || input_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index])
                        || input_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index])
                        || input_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index])
                    {

                       return true;

                    }
                    else if input_type == InputType::Perfect {

                        let allow_extra_frame = per_dir_vec[input][step].allow_extra_frame.expect("unable to find extra frame bool");
                        let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge;
                        let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("unable to find c-stick input bool");

                        for dir_index in 0 .. per_dir_vec[input][step].direction.len() {

                            let dir = per_dir_vec[input][step].direction[dir_index];

                            if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                                return true

                            }
                        }
                    }

                    if 
                        negative_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                        || negative_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || negative_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index])
                        || negative_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index])
                        || negative_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index])
                        || negative_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index])
                    {

                       return true;

                    }
                }
            }
            else {

                let mut ret = false;
                for button_index in 0 .. buttons.len() {
                    let mut skip_break = false;
                    let other_inputs_good =
                        if [InputType::Off, InputType::On, InputType::Perfect, InputType::None].contains(&input_type) { true }
                        else {
                            let mut ret = true;
                            for other_index in 0..buttons.len() {

                                if button_index == other_index { continue; }

                                if 
                                    !(input_type == InputType::On_Trigger && ControlModule::check_button_on(module_accessor, buttons[other_index])
                                    || input_type == InputType::On_Release && ControlModule::check_button_off(module_accessor, buttons[other_index]) 
                                    || input_type == InputType::Trigger && ControlModule::check_button_on(module_accessor, buttons[other_index]) 
                                    || input_type == InputType::Release && ControlModule::check_button_off(module_accessor, buttons[other_index]) 

                                    || negative_type == InputType::On_Trigger && ControlModule::check_button_on(module_accessor, buttons[other_index]) 
                                    || negative_type == InputType::On_Release && ControlModule::check_button_off(module_accessor, buttons[other_index]) 
                                    || negative_type == InputType::Trigger && ControlModule::check_button_on(module_accessor, buttons[other_index]) 
                                    || negative_type == InputType::Release && ControlModule::check_button_off(module_accessor, buttons[other_index]))
                                {

                                    ret = false;
                                    break;

                                }
                            }

                            ret

                        }
                    ;
                    
                    if 
                        input_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                        || input_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || input_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index]) && other_inputs_good
                        || input_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index]) && other_inputs_good

                        || negative_type == InputType::Off && ControlModule::check_button_off(module_accessor, buttons[button_index]) 
                        || negative_type == InputType::On && ControlModule::check_button_on(module_accessor, buttons[button_index])
                        || negative_type == InputType::On_Trigger && ControlModule::check_button_on_trriger(module_accessor, buttons[button_index]) && other_inputs_good
                        || negative_type == InputType::On_Release && ControlModule::check_button_on_release(module_accessor, buttons[button_index]) && other_inputs_good
                        || negative_type == InputType::Trigger && ControlModule::check_button_trigger(module_accessor, buttons[button_index]) && other_inputs_good
                        || negative_type == InputType::Release && ControlModule::check_button_release(module_accessor, buttons[button_index]) && other_inputs_good
                    {

                        skip_break = true

                    }
                    else if input_type == InputType::Perfect {

                        let allow_extra_frame = per_dir_vec[input][step].allow_extra_frame.expect("unable to find extra frame bool");
                        let allow_negative_edge = per_dir_vec[input][step].allow_negative_edge;
                        let allow_cstick_perfect = per_dir_vec[input][step].allow_c_stick_input.expect("unable to find c-stick input bool");

                        for dir_index in 0 .. per_dir_vec[input][step].direction.len() {

                            let dir = per_dir_vec[input][step].direction[dir_index];

                            if CommandInputModule::is_perfect_input(module_accessor, buttons[button_index], dir, allow_extra_frame, allow_negative_edge, allow_cstick_perfect) {
                        
                                skip_break = true

                            }
                        }
                    }
                    if !skip_break {

                        ret = false;
                        break

                    }
                    else {

                        ret = true;

                    }
                }

                return ret;
            }

        }

    }
    

    false

}