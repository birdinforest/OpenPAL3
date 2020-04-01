use super::sce_commands::*;
use crate::resource_manager::ResourceManager;
use crate::scene::Mv3ModelEntity;
use imgui::*;
use radiance::math::Vec3;
use radiance::scene::Director;
use radiance::scene::{CoreEntity, Entity, Scene};
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SceDirector {
    res_man: Rc<ResourceManager>,
    commands: SceCommands,
    state: HashMap<String, Box<dyn Any>>,
    active_commands: Vec<Box<dyn SceCommand>>,
    init: bool,
}

pub struct WellKnownVariables;
impl WellKnownVariables {
    pub const RUN_MODE: &'static str = "run_mode";
}

impl Director for SceDirector {
    fn update(&mut self, scene: &mut Box<dyn Scene>, ui: &mut Ui, delta_sec: f32) {
        if self.active_commands.len() == 0 {
            loop {
                match self.commands.get_next() {
                    Some(mut cmd) => {
                        if !cmd.update(scene, ui, &mut self.state, delta_sec) {
                            self.active_commands.push(cmd);
                        }
                    }
                    None => (),
                };

                if *self
                    .state
                    .get(WellKnownVariables::RUN_MODE)
                    .unwrap()
                    .downcast_ref::<i32>()
                    .unwrap()
                    == 1
                {
                    break;
                }
            }
        } else {
            let state = &mut self.state;
            self.active_commands
                .drain_filter(|cmd| cmd.update(scene, ui, state, delta_sec));
        }
    }
}

impl SceDirector {
    pub fn new(res_man: &Rc<ResourceManager>) -> Self {
        let mut state = HashMap::<String, Box<dyn Any>>::new();
        state.insert(WellKnownVariables::RUN_MODE.to_owned(), Box::new(1));

        Self {
            res_man: res_man.clone(),
            commands: SceCommands::new(res_man),
            state,
            init: false,
            active_commands: vec![],
        }
    }
}

struct SceCommands {
    init: bool,
    res_man: Rc<ResourceManager>,
    commands: Vec<Box<dyn SceCommand>>,
    pc: usize,
}

impl SceCommands {
    pub fn new(res_man: &Rc<ResourceManager>) -> Self {
        Self {
            init: false,
            res_man: res_man.clone(),
            commands: vec![
                Box::new(SceCommandRoleActive::new(res_man)),
                Box::new(SceCommandRunScriptMode::new(1)),
                Box::new(SceCommandCameraSet::new(
                    33.24_f32.to_radians(),
                    -19.48_f32.to_radians(),
                    Vec3::new(308.31, 229.44, 468.61),
                )),
                Box::new(SceCommandIdle::new(10.)),
                Box::new(SceCommandRoleShowAction::new(res_man, 11, "j04", -2)),
                Box::new(SceCommandRoleShowAction::new(res_man, 11, "j04", -2)),
                Box::new(SceCommandDlg::new()),
            ],
            pc: 0,
        }
    }

    pub fn get_next(&mut self) -> Option<Box<dyn SceCommand>> {
        if self.pc < self.commands.len() {
            let command = dyn_clone::clone_box(&*self.commands[self.pc]);
            self.pc += 1;
            Some(command)
        } else {
            None
        }
    }
}

pub trait SceCommand: dyn_clone::DynClone {
    fn update(
        &mut self,
        scene: &mut Box<dyn Scene>,
        ui: &mut Ui,
        state: &mut HashMap<String, Box<dyn Any>>,
        delta_sec: f32,
    ) -> bool;
}

dyn_clone::clone_trait_object!(SceCommand);
