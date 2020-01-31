use druid::{
    AppLauncher, WindowDesc
};



use lights::app_del::Del;
pub use lights::view_state::ViewState;

type BoxedRes<T> = Result<T, String>;

fn main() -> BoxedRes<()> {
    let resource = lights::config::get_config()?;
    let (tx, rx) = crossbeam::channel::unbounded();
    let rabbit_url = resource.borrow().rabbit.url.clone();
    let del = Del { resource, tx };
    let view_state = del.get_view_state();
    let vs = view_state.clone();
    let w = WindowDesc::new(move || {
        let vs = vs.clone();
        lights::ui_setup::setup_window(vs)
    })
    .window_size((480.0, 320.0));
    {
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(0) => std::process::exit(0),
                    Ok(code) => {
                        if let Err(e) = lights::rabbit::send_code(code, &rabbit_url) {
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

