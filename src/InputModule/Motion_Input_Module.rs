use {
    smash::{
        lua2cpp::*, 
        phx::*,
        app::{ 
            lua_bind::*, 
            sv_animcmd::*, 
            * 
        }, 
        lib::{ 
            lua_const::*, 
            L2CAgent, 
            L2CValue 
        },
        hash40
    }, 
    smash_script::*, 
    smashline::{ 
        Priority::*,
        * 
    }, 
    std::{ 
        any::type_name, 
        usize 
    },
    crate::InputModule::{
        *,
        Command_Input_Module::{
            *,
            InputDirection::*
        }
    }
}; 

const default_buffer: u8 = 7;

static mut player_1_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_2_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_3_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_4_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_5_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_6_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_7_motion_vec: Vec<Vec<InputDirection>> = Vec::new();
static mut player_8_motion_vec: Vec<Vec<InputDirection>> = Vec::new();

static mut player_1_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_2_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_3_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_4_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_5_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_6_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_7_input_vec: Vec<Vec<inputs>> = Vec::new();
static mut player_8_input_vec: Vec<Vec<inputs>> = Vec::new();

static mut player_1_step: Vec<usize> = Vec::new();
static mut player_2_step: Vec<usize> = Vec::new();
static mut player_3_step: Vec<usize> = Vec::new();
static mut player_4_step: Vec<usize> = Vec::new();
static mut player_5_step: Vec<usize> = Vec::new();
static mut player_6_step: Vec<usize> = Vec::new();
static mut player_7_step: Vec<usize> = Vec::new();
static mut player_8_step: Vec<usize> = Vec::new();

static mut player_1_buffer: Vec<u8> = Vec::new();
static mut player_2_buffer: Vec<u8> = Vec::new();
static mut player_3_buffer: Vec<u8> = Vec::new();
static mut player_4_buffer: Vec<u8> = Vec::new();
static mut player_5_buffer: Vec<u8> = Vec::new();
static mut player_6_buffer: Vec<u8> = Vec::new();
static mut player_7_buffer: Vec<u8> = Vec::new();
static mut player_8_buffer: Vec<u8> = Vec::new();

static mut player_1_buffer_window: u8 = default_buffer;
static mut player_2_buffer_window: u8 = default_buffer;
static mut player_3_buffer_window: u8 = default_buffer;
static mut player_4_buffer_window: u8 = default_buffer;
static mut player_5_buffer_window: u8 = default_buffer;
static mut player_6_buffer_window: u8 = default_buffer;
static mut player_7_buffer_window: u8 = default_buffer;
static mut player_8_buffer_window: u8 = default_buffer;

static mut player_1_use_motion_input_module: bool = false;
static mut player_2_use_motion_input_module: bool = false;
static mut player_3_use_motion_input_module: bool = false;
static mut player_4_use_motion_input_module: bool = false;
static mut player_5_use_motion_input_module: bool = false;
static mut player_6_use_motion_input_module: bool = false;
static mut player_7_use_motion_input_module: bool = false;
static mut player_8_use_motion_input_module: bool = false;

/// A custom Module made for making custom motion inputs eaiser to make 
pub mod MotionInputModule {

    use super::*;

    /// Adds a custom motion to the fighter, this should be done on init or start
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `vec` - a `Vec` containing the desired `InputDirections` for the input
    /// 
    /// # Example
    /// 
    /// ```unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    /// 
    ///     //adds a down down motion to all fighters
    ///     MotionInputModule::add_motion(fighter.module_accessor, Vec::From(InputDirection::DOWN, InputDirection::NEUTRAL, InputDirection::DOWN))
    /// 
    /// }
    /// pub fn install() {
    /// 
    ///     Agent::new("fighter")
    ///     .on_start(on_start)
    ///     .install();
    /// 
    /// }```
    pub unsafe fn add_motion(module_accessor:*mut BattleObjectModuleAccessor, vec: Vec<InputDirection>) {

        let mut motion = vec;
        motion.push(NULL);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_vec = get_motion_vec(&entry_id);
        let player_input_vec = get_input_vec(&entry_id);
        let mut new_vec: Vec<inputs> = Vec::new();
        let use_motion_input_module = get_use_motion_input_module(&entry_id);
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = *get_buffer_window(&entry_id);

        player_vec.push(motion.clone());

        if !*use_motion_input_module { *use_motion_input_module = true; }

        let len = motion.len();
        let blank_inputs:inputs = inputs {

            has_input_atached: (false), 
            input_type: (None),
            button: (None), 
            strict: (false), 
            dir: (None), 
            allow_extra_frame: (None), 
            allow_negative_edge: (None), 
            allow_c_stick_input: (None) 

        };

        for i in 0..len { new_vec.push(blank_inputs.clone()); }
        player_input_vec.push(new_vec);
        step.push(0);
        buffer.push(buffer_window);
        
        dbg!(&player_vec);
        dbg!(&player_input_vec); 
        dbg!(&step);
        dbg!(&buffer);


    }

    /// Sets a custom buffer ammout for all custom inputs 
    /// 
    /// default is 7 which translates to 8 frames of time between inputs
    pub unsafe fn set_custom_input_buffer(module_accessor:*mut BattleObjectModuleAccessor, new_buffer: u8) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_buffer_window = get_buffer_window(&entry_id);

        *player_buffer_window = new_buffer;

    }

    /// Adds the ControlModule::check_button_off check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(DOWN, NEUTRAL, DOWN));
    /// MotionInputModule::add_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_off(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(off),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };

    }
    /// Adds the ControlModule::check_button_on check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a qcf motion and is pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(DOWN, DOWN_FORWARD, FORWARD));
    /// MotionInputModule::add_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, false, 2);
    /// ```
    pub unsafe fn add_button_on(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_on_release check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    pub unsafe fn add_button_on_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on_release),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_on_trigger check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    pub unsafe fn add_button_on_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(on_trigger),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_trigger check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(BACK, FORWARD ));
    /// MotionInputModule::add_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_trigger(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(trigger),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the ControlModule::check_button_release check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a  motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(UP, UP_FORWARD, FORWARD));
    /// MotionInputModule::add_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, true, 2);
    /// ```
    pub unsafe fn add_button_release(module_accessor:*mut BattleObjectModuleAccessor, button: i32, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(release),
            button: Some(button),
            strict: strict,
            dir: None,
            allow_extra_frame: None,
            allow_negative_edge: None,
            allow_c_stick_input: None
        };
    }
    /// Adds the CommandInputModule::is_perfect_input check to a desired input
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `button` - a CONTROL_PAD_BUTTON lua const
    /// 
    /// * `dir` - the `InputDirection` being checked
    /// 
    /// * `allow_extra_frame` - a `bool` that determines if you can do the input 1 frame late simmaler to kazuyas ewgf
    /// 
    /// * `allow_negative_edge` - a `bool` that determines if you can use negative edge to trigger the input
    /// 
    /// * `allow_c_stick_perfect` - a `bool` that determines if the cstick can be used to do the input
    /// 
    /// * `strict` - a `bool` that determines if the direction and the button were not hit on the same frame to kill all progress on the input
    /// 
    /// * `input` - a `usize` of what input you want to add the check to
    /// 
    /// # Exaple
    /// 
    /// ``` if the player dose a down down motion and is not pressing attack 
    /// MotionInputModule::add_motion(fighter.module_accessor, Vec::from(FORWARD, DOWN, DOWN_FORWARD));
    /// MotionInputModule::add_perfect_input(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK, DOWN_FORWARD, false, false, false, true, 2);
    /// ```
    pub unsafe fn add_perfect_input(module_accessor:*mut BattleObjectModuleAccessor, button: i32, dir: InputDirection, allow_extra_frame: bool, allow_negative_edge: bool, allow_c_stick_perfect: bool, strict: bool, input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let player_input_vec = get_input_vec(&entry_id);
        let player_input_count = player_input_vec.len() - 1;

        player_input_vec[player_input_count][input] = inputs{
            has_input_atached: true,
            input_type: Some(perfect),
            button: Some(button),
            strict: strict,
            dir: Some(dir),
            allow_extra_frame: Some(allow_extra_frame),
            allow_negative_edge: Some(allow_negative_edge),
            allow_c_stick_input: Some(allow_c_stick_perfect)
        };

        dbg!(&player_input_vec);

    }
    /// Gets what step a given input is on as a 'u8'
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `motion_input` - a `usize` of the input you are checking coresponding to the order it was set in the init or start
    pub unsafe fn get_input_step(module_accessor:*mut BattleObjectModuleAccessor, motion_input: usize) -> u8 {

        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let step = get_step(&entry_id);

        step[motion_input] as u8
    }
    /// Forces a given input to reset back to the first step [step 0]
    /// 
    /// # Arguments
    /// 
    /// * `module_accessor` - a pointer to `BattleObjectModuleAccessor`
    /// 
    /// * `motion_input` - a `usize` of the input you are checking coresponding to the order it was set in the init or start
    pub unsafe fn reset_input_step(module_accessor:*mut BattleObjectModuleAccessor, motion_input: usize) {
        
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = get_buffer_window(&entry_id);

        step[motion_input] = 0;
        buffer[motion_input] = 0;

    }
}

unsafe fn get_motion_vec(entry_id: &usize) -> &mut Vec<Vec<InputDirection>> {

    let motion_vec = if *entry_id == 0 { &mut player_1_motion_vec }
        else if *entry_id == 1 { &mut player_2_motion_vec } else if *entry_id == 2 { &mut player_3_motion_vec }
        else if *entry_id == 3 { &mut player_4_motion_vec } else if *entry_id == 4 { &mut player_5_motion_vec }
        else if *entry_id == 5 { &mut player_6_motion_vec } else if *entry_id == 6 { &mut player_7_motion_vec }
    else { &mut player_8_motion_vec };

    motion_vec

}
unsafe fn get_input_vec(entry_id: &usize) -> &mut Vec<Vec<inputs>> {
    
    let input_vec = if *entry_id == 0 { &mut player_1_input_vec }
        else if *entry_id == 1 { &mut player_2_input_vec } else if *entry_id == 2 { &mut player_3_input_vec }
        else if *entry_id == 3 { &mut player_4_input_vec } else if *entry_id == 4 { &mut player_5_input_vec }
        else if *entry_id == 5 { &mut player_6_input_vec } else if *entry_id == 6 { &mut player_7_input_vec }
    else { &mut player_8_input_vec };

    input_vec
}
unsafe fn get_step(entry_id: &usize) -> &mut Vec<usize> {

    let step = if *entry_id == 0 { &mut player_1_step }
        else if *entry_id == 1 { &mut player_2_step } else if *entry_id == 2 { &mut player_3_step }
        else if *entry_id == 3 { &mut player_4_step } else if *entry_id == 4 { &mut player_5_step }
        else if *entry_id == 5 { &mut player_6_step } else if *entry_id == 6 { &mut player_7_step }
    else { &mut player_8_step };

    step

}
unsafe fn get_buffer(entry_id: &usize) -> &mut Vec<u8> {
    
    let buffer = if *entry_id == 0 { &mut player_1_buffer }
        else if *entry_id == 1 { &mut player_2_buffer } else if *entry_id == 2 { &mut player_3_buffer }
        else if *entry_id == 3 { &mut player_4_buffer } else if *entry_id == 4 { &mut player_5_buffer }
        else if *entry_id == 5 { &mut player_6_buffer } else if *entry_id == 6 { &mut player_7_buffer }
    else { &mut player_8_buffer };

    buffer
}
unsafe fn get_buffer_window(entry_id: &usize) -> *mut u8 {
    
    let buffer_window = if *entry_id == 0 { &raw mut player_1_buffer_window }
        else if *entry_id == 1 { &raw mut player_2_buffer_window } else if *entry_id == 2 { &raw mut player_3_buffer_window }
        else if *entry_id == 3 { &raw mut player_4_buffer_window } else if *entry_id == 4 { &raw mut player_5_buffer_window }
        else if *entry_id == 5 { &raw mut player_6_buffer_window } else if *entry_id == 6 { &raw mut player_7_buffer_window }
    else { &raw mut player_8_buffer_window };

    buffer_window
}
unsafe fn get_use_motion_input_module(entry_id: &usize) -> *mut bool {

    let is_using_motion_input_module = if *entry_id == 0 { &raw mut player_1_use_motion_input_module }
        else if *entry_id == 1 { &raw mut player_2_use_motion_input_module } else if *entry_id == 2 { &raw mut player_3_use_motion_input_module }
        else if *entry_id == 3 { &raw mut player_4_use_motion_input_module } else if *entry_id == 4 { &raw mut player_5_use_motion_input_module }
        else if *entry_id == 5 { &raw mut player_6_use_motion_input_module } else if *entry_id == 6 { &raw mut player_7_use_motion_input_module }
    else { &raw mut player_8_use_motion_input_module };

    is_using_motion_input_module

}

unsafe fn get_inputs_for_current_step(module_accessor:*mut BattleObjectModuleAccessor, input: &inputs) -> bool {

    if !input.has_input_atached {

        return true

    }
    else if input.input_type == Some(on) {
        
        return ControlModule::check_button_on(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(off) {

        return ControlModule::check_button_off(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(on_trigger) {

        return ControlModule::check_button_on_trriger(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(on_release) {

        return ControlModule::check_button_on_release(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(trigger) {

        return ControlModule::check_button_trigger(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(release) {

        return ControlModule::check_button_release(module_accessor, input.button.expect("no input found"))

    }
    else if input.input_type == Some(perfect) {

        return CommandInputModule::is_perfect_input(module_accessor, input.button.expect("no input found"), input.dir.expect("no input found"), input.allow_extra_frame.expect("no input found"), input.allow_negative_edge.expect("no input found"), input.allow_c_stick_input.expect("no input found"))

    }
    else {

        false
    }
}

unsafe extern "C" fn motion_input_frame(fighter: &mut L2CFighterCommon) {
        
    if !StatusModule::is_changing(fighter.module_accessor) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if *get_use_motion_input_module(&entry_id) {

            let motion_vec = get_motion_vec(&entry_id);
            let motion_input_count = motion_vec.len();
            let input_vec = get_input_vec(&entry_id);
            let step = get_step(&entry_id);
            let prev_step = step.clone();
            let buffer = get_buffer(&entry_id);
            let buffer_window = *get_buffer_window(&entry_id);
            let input_dir = CommandInputModule::get_stick_dir(fighter.module_accessor);
            for current_input in 0..motion_input_count {

                let input = get_inputs_for_current_step(fighter.module_accessor, &input_vec[current_input][step[current_input]]);
                let input_compare_against = if input_vec[current_input][step[current_input]].input_type == Some(perfect) { ControlModule::check_button_on_trriger(fighter.module_accessor, input_vec[current_input][step[current_input]].button.expect("no input found"))} 
                    else {get_inputs_for_current_step(fighter.module_accessor, &input_vec[current_input][step[current_input]])};
                let max = motion_vec[current_input].len() - 1;

                if step[current_input] == prev_step[current_input] && buffer[current_input] > 0 {

                    buffer[current_input] -= 1;

                }
                else if buffer[current_input] == 0 {

                    step[current_input] = 0;

                }

                if input_dir == motion_vec[current_input][step[current_input]] && step[current_input] != max && input {

                    step[current_input] += 1;
                    buffer[current_input] = buffer_window;

                }
                else if input_dir == motion_vec[current_input][step[current_input]] && step[current_input] != max && input_vec[current_input][step[current_input]].strict 
                || input_compare_against && step[current_input] != max && input_vec[current_input][step[current_input]].strict {

                    step[current_input] = 0;
                    buffer[current_input] = buffer_window;

                }
            }
            dbg!(&step);
        }
    }
}

unsafe extern "C" fn reset_motion_input_module(fighter: &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if *get_use_motion_input_module(&entry_id) {

        let motion_vec = get_motion_vec(&entry_id);
        let input_vec = get_input_vec(&entry_id);
        let step = get_step(&entry_id);
        let buffer = get_buffer(&entry_id);
        let buffer_window = get_buffer_window(&entry_id);
        let use_motion_input_module = get_use_motion_input_module(&entry_id);

        *motion_vec = Vec::new();
        *input_vec = Vec::new();
        *step = Vec::new();
        *buffer = Vec::new();
        *buffer_window = default_buffer;
        *use_motion_input_module = false;

    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Main, motion_input_frame)
        .on_end(reset_motion_input_module)
    .install();
}


/*
todo

    raging demon shortcut options (allow the check to run additonal times if the first is sucsessful [dependent on the input])
    allow for the user to specifiy how strict the input should be 

*/