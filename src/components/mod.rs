use imgui::*;
use support::TITLE_FONT;

use crate::support;

pub fn render_title(ui: &Ui, title: &str) {
    let title_font_ptr = ui.push_font(unsafe { TITLE_FONT.unwrap() });

    let win_width = ui.window_size()[0];
    let text_width = ui.calc_text_size(title)[0];

    ui.set_cursor_pos([(win_width - text_width) * 0.5, 20.0]);

    ui.text(title);
    title_font_ptr.pop();
}

pub fn render_colored_btn(ui: &Ui, label: &str, alt: bool) -> bool {
    let mut bg_col = [1.0, 1.0, 1.0, 1.0];
    let mut text_col = [0.0, 0.0, 0.0, 1.0];
    let mut bg_hover_col = [0.8, 0.8, 0.8, 1.0];
    let mut bg_active_col = [0.6, 0.6, 0.6, 1.0];

    if alt {
        bg_col = [0.3, 0.3, 0.3, 1.0];
        text_col = [1.0, 1.0, 1.0, 1.0];
        bg_hover_col = [0.4, 0.4, 0.4, 1.0];
        bg_active_col = [0.1, 0.1, 0.1, 1.0];
    }

    let btn_bg = ui.push_style_color(StyleColor::Button, bg_col);
    let btn_text = ui.push_style_color(StyleColor::Text, text_col);
    let btn_hover = ui.push_style_color(StyleColor::ButtonHovered, bg_hover_col);
    let btn_active = ui.push_style_color(StyleColor::ButtonActive, bg_active_col);
    let btn_rounding = ui.push_style_var(StyleVar::FrameRounding(8.0));

    let btn = ui.button_with_size(label, [-1.0, 40.0]);

    btn_rounding.pop();

    btn_active.pop();
    btn_hover.pop();
    btn_text.pop();
    btn_bg.pop();

    return btn;
}

pub fn render_radio_btn<T>(
    ui: &Ui, 
    label: impl AsRef<str>, 
    mut value: T, button_value: T
) -> bool
where
    T: Copy + PartialEq 
    
{
    let rbg = ui.push_style_color(StyleColor::FrameBg, [0.4, 0.4, 0.4, 1.0]);
    let rbg2 = ui.push_style_color(StyleColor::FrameBgHovered, [0.3, 0.3, 0.3, 1.0]);
    let rbg3 = ui.push_style_color(StyleColor::FrameBgActive, [0.5, 0.5, 0.5, 1.0]);

    let rbtn = ui.radio_button(label, &mut value, button_value);
    
    rbg3.pop();
    rbg2.pop();
    rbg.pop();

    return rbtn;
}