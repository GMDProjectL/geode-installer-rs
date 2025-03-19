use imgui::*;
use support::titleFont;

mod support;

fn render_title(ui: &Ui) {
    let title_font_ptr = ui.push_font(unsafe { titleFont.unwrap() });

    let win_width = ui.window_size()[0];
    let text_width = ui.calc_text_size("Geode Installer")[0];

    ui.set_cursor_pos([(win_width - text_width) * 0.5, 20.0]);

    ui.text("Geode Installer");
    title_font_ptr.pop();
}

fn main() {
    support::simple_init("Geode Installer", move |_, ui| {
        ui.window("Hello world")
            .position([-2.0, -2.0], Condition::Always)
            .size([502.0, 352.0], Condition::Always)
            .flags(WindowFlags::NO_BACKGROUND)
            .flags(WindowFlags::NO_TITLE_BAR)
            .resizable(false)
            .movable(false)
            .build(|| {
                render_title(ui);
            });
    });
}