use druid::{
    Env, Widget,
    widget::{Button, Flex, Padding}
};
use super::{ViewState, Switch};
pub fn setup_window(config: ViewState) -> impl Widget<ViewState> {
    let buttons = config.switches.iter().map(setup_button_group).collect::<Vec<_>>();
    let mut rows = Vec::new();
    match buttons.len() {
        1 | 2 | 3 => {
            let mut row = Flex::row();
            for button in buttons {
                row.add_child(button, 1.0);
            }
            rows.push(row);
        },
        4 | 6 => {
            let top = buttons.len() / 2;
            let mut b = buttons.into_iter();
            for _ in 0..2 {
                let mut row = Flex::row();
                for _ in 0..top {
                    if let Some(button) = b.next() {
                        row.add_child(button, 1.0)
                    }
                }
                rows.push(row);
            }
        },
        5 => {
            let mut b = buttons.into_iter();
            for _ in 0..2 {
                let mut row = Flex::row();
                for _ in 0..3 {
                    if let Some(button) = b.next() {
                        row.add_child(button, 1.0)
                    }
                }
                rows.push(row);
            }
        }
        _ => (),
    }
    let mut col = Flex::column();
    for row in rows {
        col.add_child(row, 1.0);
    }
    col
}

fn setup_button_group(sw: &Switch) -> Flex<ViewState> {
    let mut ret = Flex::column();
    let label = druid::widget::Label::new(sw.name.to_string());
    ret.add_child(Padding::new(5.0, label), 0.25);
    setup_button("on", sw.on_code, &mut ret);
    setup_button("off", sw.off_code, &mut ret);
    ret
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
