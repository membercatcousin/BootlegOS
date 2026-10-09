use chrono::Local;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::time::Duration;
use u8g2_fonts::{fonts, FontRenderer};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("BootlegOS v0.1 (Rolling)", &output_settings);

    window.update(&display);

    'running: loop {
        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'running;
            }
        }

        display.clear(Rgb565::BLACK)?;
        draw_watchface(&mut display);
        window.update(&display);

        std::thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}

fn draw_watchface(display: &mut SimulatorDisplay<Rgb565>) {
    let white = Rgb565::WHITE;
    let green = Rgb565::new(0, 63, 0);

    let battery_font = FontRenderer::new::<fonts::u8g2_font_helvB10_tr>();
    let _ = battery_font.render_aligned(
        "75%",
        Point::new(182, 18),
                                        u8g2_fonts::types::VerticalPosition::Center,
                                        u8g2_fonts::types::HorizontalAlignment::Right,
                                        u8g2_fonts::types::FontColor::Transparent(white),
                                        display,
    );

    let _ = Rectangle::new(Point::new(186, 12), Size::new(26, 13))
    .into_styled(PrimitiveStyleBuilder::new().stroke_color(white).stroke_width(1).build())
    .draw(display);

    let _ = Rectangle::new(Point::new(212, 15), Size::new(2, 7))
    .into_styled(PrimitiveStyleBuilder::new().fill_color(white).build())
    .draw(display);

    let _ = Rectangle::new(Point::new(188, 14), Size::new(16, 9))
    .into_styled(PrimitiveStyleBuilder::new().fill_color(green).build())
    .draw(display);

    let now = Local::now();
    let time_str = now.format("%H:%M").to_string();
    let date_str = now.format("%d %b %Y").to_string().to_uppercase();

    let clock_font = FontRenderer::new::<fonts::u8g2_font_logisoso38_tr>();
    let _ = clock_font.render_aligned(
        time_str.as_str(),
                                      Point::new(120, 105),
                                      u8g2_fonts::types::VerticalPosition::Center,
                                      u8g2_fonts::types::HorizontalAlignment::Center,
                                      u8g2_fonts::types::FontColor::Transparent(white),
                                      display,
    );

    let date_font = FontRenderer::new::<fonts::u8g2_font_helvB14_tr>();
    let _ = date_font.render_aligned(
        date_str.as_str(),
                                     Point::new(120, 145),
                                     u8g2_fonts::types::VerticalPosition::Center,
                                     u8g2_fonts::types::HorizontalAlignment::Center,
                                     u8g2_fonts::types::FontColor::Transparent(white),
                                     display,
    );
}
