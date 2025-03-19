use std::process::exit;

use imgui::*;
use components::*;

mod support;
mod components;


fn main() {
    let mut state = State::default();
    state.install_variant = "steam";

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

                ui.text_colored(
                    [0.5, 0.5, 0.5, 1.0], 
                    "Choose your GD installation:"
                );
                ui.set_cursor_pos([20.0, 120.0]);
                
                if render_radio_btn(
                    &ui, "Wine", 
                    state.install_variant, 
                    "wine"
                ) {
                    state.install_variant = "wine";
                }

                ui.set_cursor_pos([20.0, ui.cursor_pos()[1]]);

                if render_radio_btn(
                    &ui, "Steam", 
                    state.install_variant, 
                    "steam"
                ) {
                    state.install_variant = "steam";
                }


                ui.set_cursor_pos([10.0, 250.0]);
                render_colored_btn(&ui, "Install", false);

                ui.set_cursor_pos([10.0, 300.0]);
                if render_colored_btn(&ui, "Exit", true) {
                    exit(0);
                }
            });
    });
}

#[derive(Default)]
struct State {
    install_variant: &'static str,
}