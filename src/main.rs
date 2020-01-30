use druid::{
    AppLauncher, WindowDesc
};

mod app_del;
mod config;
mod rabbit;
mod ui_setup;
mod view_state;

use app_del::Del;
pub use config::{
    Config,
    Switch,
};
pub use view_state::ViewState;

type BoxedRes<T> = Result<T, String>;

fn main() -> BoxedRes<()> {
    let resource = config::get_config()?;
    let (tx, rx) = crossbeam::channel::unbounded();
    let rabbit_url = resource.borrow().rabbit.url.clone();
    let del = Del { resource, tx };
    let view_state = del.get_view_state();
    let vs = view_state.clone();
    let w = WindowDesc::new(move || {
        let vs = vs.clone();
        ui_setup::setup_window(vs)
    })
    .window_size((480.0, 320.0));
    {
        let url = rabbit_url.clone();
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(0) => std::process::exit(0),
                    Ok(code) => {
                        if let Err(e) = rabbit::send_code(code, &url) {
                            eprintln!("failed to send code: {}\n {}", code, e);
                        } else {
                            eprintln!("successfully sent code: {}", code);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error in receiver: {}", e);
                    }
                }
            }
        });
    }
    AppLauncher::with_window(w)
        .delegate(del)
        .use_simple_logger()
        .launch(view_state)
        .expect("failed to launch window");
    Ok(())
}

