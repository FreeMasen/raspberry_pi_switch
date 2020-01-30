use druid::widget::{Button, Flex, Padding};
use druid::{
    AppDelegate, AppLauncher, Data, DelegateCtx, Env, Event, Widget, WindowDesc,
    WindowId,
};
use serde::Deserialize;
use warmy::{toml::Toml, Res, SimpleKey, Store, StoreOpt};

type BoxedRes<T> = Result<T, String>;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    pub switches: Vec<Switch>,
}
impl Data for ViewState {
    fn same(&self, other: &Self) -> bool {
        if self.switches.len() != other.switches.len() {
            return false;
        }
        for (lhs, rhs) in self.switches.iter().zip(other.switches.iter()) {
            if lhs.name != rhs.name {
                return false;
            }
        }
        true
    }
}
#[derive(Debug, Deserialize, Data, Clone)]
struct Switch {
    pub name: String,
    pub code: u32,
}

#[derive(Debug, Clone)]
struct ViewState {
    switches: Vec<Switch>,
    tx: crossbeam::channel::Sender<u32>,
}

fn main() -> BoxedRes<()> {
    let ctx = &mut ();
    let home_dir = if let Some(home_dir) = dirs::home_dir() {
        home_dir
    } else {
        return Err("Unable to find the home directory".to_string());
    };
    let opt = StoreOpt::default().set_root(home_dir);
    let mut store: Store<(), SimpleKey> =
        Store::new(opt).map_err(|e| format!("error establishing storage {}", e))?;

    let resource: Res<Config> = store
        .get_by(&SimpleKey::from_path("/.lights"), ctx, Toml)
        .map_err(|e| format!("failed to get config from store {}", e))?;
    let (tx, rx) = crossbeam::channel::unbounded();
    let del = Del { resource, tx };
    let view_state = del.get_view_state();
    let vs = view_state.clone();
    let w = WindowDesc::new(move || {
        let vs = vs.clone();
        window_ui(vs)
    })
    .window_size((480.0, 320.0));
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(0) => std::process::exit(0),
                Ok(code) => {
                    //send code
                    println!("code: {}", code);
                }
                Err(e) => {
                    eprintln!("Error in reciever: {}", e);
                }
            }
        }
    });
    AppLauncher::with_window(w)
        .delegate(del)
        .launch(view_state)
        .expect("failed to launch window");
    Ok(())
}

struct Del {
    resource: Res<Config>,
    tx: crossbeam::channel::Sender<u32>,
}
impl Del {
    fn get_view_state(&self) -> ViewState {
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
        id: WindowId,
        _data: &mut ViewState,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        ctx.submit_command(druid::commands::QUIT_APP.into(), id)
    }
}

fn window_ui(config: ViewState) -> impl Widget<ViewState> {
    let mut row = Flex::row();
    let mut col = Flex::column();
    for (i, switch) in config.switches.iter().enumerate() {
        if i > 2 {
            let old = std::mem::replace(&mut row, Flex::row());
            col.add_child(old, 1.0);
        }
        setup_button(&switch.name, switch.code, &mut row);
    }
    col.add_child(row, 1.0);
    col
}

fn setup_button(name: &str, code: u32, flex: &mut Flex<ViewState>) {
    let button = create_button(name, code);
    let p = Padding::new(5.0, button);
    flex.add_child(p, 1.0);
}

fn create_button(name: &str, code: u32) -> Button<ViewState> {
    Button::new(
        name.to_string(),
        move |_ctx, c: &mut ViewState, _e: &Env| {
            let _ = c.tx.send(code);
        },
    )
}
