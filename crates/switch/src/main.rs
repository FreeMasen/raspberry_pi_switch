use warmy::Res;
use serde::Deserialize;
use lights::config::Config;

type BoxedRes<T> = Result<T, String>;

type State<'a> = web_view::WebView<'a, Res<Config>>;

fn main() -> BoxedRes<()> {
    let resource = lights::config::get_config()?;
    let mut wv: State = web_view::builder()
        .resizable(false)
        .content(web_view::Content::Html(include_str!("index.html")))
        .frameless(true)
        .size(480, 320)
        .user_data(resource)
        .invoke_handler(invoke_handler)
        .build()
        .map_err(|e| format!("Failed to build webview {}", e))?;
    wv.set_fullscreen(true);
    wv.run().expect("failed to run webview");
    Ok(())
}

fn invoke_handler(wv: &mut State, arg: &str) -> web_view::WVResult {
    wv.set_fullscreen(true);
    match serde_json::from_str(arg) {
        Ok(ev) => {
            handle_event(wv, ev);
        },
        Err(e) => {
            eprintln!("error deserializing event {}", e);
        }
    }
    Ok(())
}

fn handle_event(wv: &mut State, ev: UIEvent) {
    match ev {
        UIEvent::Flip(code) => handle_flip(wv, code),
        UIEvent::Tick => handle_tick(wv),
    }
}

fn handle_flip(wv: &mut State, code: u32) {
    let url = &wv.user_data().borrow().rabbit.url;
    let _ = lights::rabbit::send_code(code, url);
}

fn handle_tick(wv: &mut State) {
    let json = if let Ok(state) = serde_json::to_string(&wv.user_data().borrow().switches) {
        state
    } else {
        "[]".to_string()
    };
    let _ = wv.eval(&format!("renderSwitches({:?})", json));
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind", content = "data")]
enum UIEvent {
    Flip(u32),
    Tick,
}

