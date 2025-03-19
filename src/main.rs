use std::{process::exit, thread, time::Duration};

use imgui::*;
use components::*;
use install::install_using_wine;

mod support;
mod components;
mod install;

#[tokio::main]
async fn main() {
    let mut state = State::default();
    state.install_variant = "steam";
    state.show_successful_installation_message = false;

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

                if state.show_successful_installation_message {
                    ui.set_cursor_pos([20.0, ui.cursor_pos()[1] + 20.0]);

                    ui.text_colored(
                        [0.0, 1.0, 0.0, 0.7], 
                        "Geode installed successfully!"
                    );
                }


                ui.set_cursor_pos([10.0, 250.0]);
                if render_colored_btn(&ui, "Install", false) {
                    if state.install_variant == "wine" {
                        let wine_installation_task =tokio::spawn(
                            async move {
                                let _ = install_using_wine().await;
                            }
                        );

                        while !wine_installation_task.is_finished() {
                            thread::sleep(Duration::new(0, 500));
                        }

                        state.show_successful_installation_message = true;
                    }
                }

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
    show_successful_installation_message: bool
}