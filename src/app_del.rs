use crossbeam::channel::Sender;
use druid::{AppDelegate, DelegateCtx, Env, Event, WindowId};
use warmy::Res;

use crate::{config::Config, view_state::ViewState};
pub struct Del {
    pub resource: Res<Config>,
    pub tx: Sender<u32>,
}
impl Del {
    pub fn get_view_state(&self) -> ViewState {
        ViewState {
            switches: self.resource.borrow().switches.clone(),
            tx: self.tx.clone(),
        }
    }
}
impl AppDelegate<ViewState> for Del {
    fn event(
        &mut self,
        event: Event,
        data: &mut ViewState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) -> Option<Event> {
        match event {
            Event::AnimFrame(_) => {
                *data = ViewState {
                    switches: self.resource.borrow().switches.clone(),
                    tx: self.tx.clone(),
                }
            }
            Event::Command(ref c) => {
                println!("command: {:?}", c);
            }
            _ => (),
        }
        Some(event)
    }
    fn window_added(
        &mut self,
        _id: WindowId,
        _data: &mut ViewState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }
    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut ViewState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        std::process::exit(0);
    }
}
