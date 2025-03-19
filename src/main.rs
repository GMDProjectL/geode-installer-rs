use std::process::exit;

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
    let mut state = State::default();

    support::simple_init("Geode Installer", move |_, ui| {
        ui.window("Hello world")
            .position([-2.0, -2.0], Condition::Always)
            .size([504.0, 354.0], Condition::Always)
            .flags(WindowFlags::NO_BACKGROUND)
            .flags(WindowFlags::NO_TITLE_BAR)
            .resizable(false)
            .movable(false)
            .bg_alpha(0.0)
            .build(|| {
                render_title(ui);

                ui.set_cursor_pos([20.0, 75.0]);

                ui.text_colored([0.5, 0.5, 0.5, 1.0], "Choose your GD installation:");

                let rbg = ui.push_style_color(StyleColor::FrameBg, [0.4, 0.4, 0.4, 1.0]);
                let rbg2 = ui.push_style_color(StyleColor::FrameBgHovered, [0.3, 0.3, 0.3, 1.0]);
                let rbg3 = ui.push_style_color(StyleColor::FrameBgActive, [0.5, 0.5, 0.5, 1.0]);

                ui.set_cursor_pos([20.0, 120.0]);
                ui.radio_button("Wine", &mut state.install_variant, "wine");
                
                ui.set_cursor_pos([20.0, ui.cursor_pos()[1]]);
                ui.radio_button("Steam", &mut state.install_variant, "steam");

                rbg3.pop();
                rbg2.pop();
                rbg.pop();

                ui.set_cursor_pos([10.0, 250.0]);
                
                let bright_btn = ui.push_style_color(StyleColor::Button, [1.0, 1.0, 1.0, 1.0]);
                let bright_btn_text = ui.push_style_color(StyleColor::Text, [0.0, 0.0, 0.0, 1.0]);
                let bright_btn_hover = ui.push_style_color(StyleColor::ButtonHovered, [0.8, 0.8, 0.8, 1.0]);
                let bright_btn_active = ui.push_style_color(StyleColor::ButtonActive, [0.6, 0.6, 0.6, 1.0]);
                let bright_btn_rounding = ui.push_style_var(StyleVar::FrameRounding((8.0)));

                ui.button_with_size("Install", [-1.0, 40.0]);

                bright_btn_active.pop();
                bright_btn_hover.pop();
                bright_btn_text.pop();
                bright_btn.pop();
                
                let close_btn = ui.push_style_color(StyleColor::Button, [0.3, 0.3, 0.3, 1.0]);
                let close_btn_hover = ui.push_style_color(StyleColor::ButtonHovered, [0.4, 0.4, 0.4, 1.0]);
                let close_btn_active = ui.push_style_color(StyleColor::ButtonActive, [0.1, 0.1, 0.1, 1.0]);

                ui.set_cursor_pos([10.0, 300.0]);
                
                if ui.button_with_size("Close", [-1.0, 40.0]) {
                    exit(0);
                }

                close_btn.pop();
                close_btn_hover.pop();
                close_btn_active.pop();

                bright_btn_rounding.pop();
            });
    });
}

#[derive(Default)]
struct State {
    install_variant: &'static str,
}

impl State {
    fn reset(&mut self) {}
}