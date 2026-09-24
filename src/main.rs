use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Arc, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window
};
use embedded_graphics_simulator::sdl2::Keycode;
use std::{thread, time::Duration};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 64x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    // Create styles used by the drawing operations.
    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(8)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Middle)
        .alignment(Alignment::Center)
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Progress", &output_settings);

    // The current progress percentage
    let mut progress = 78;

    'running: loop {
        display.clear(BinaryColor::Off)?;

        let sweep = progress as f32 * 360.0 / 100.0;
        let remaining = ( 101 - progress ) / 10 as u16;

        // Draw an arc with a 5px wide stroke.
        let arc = Arc::with_center(Point::new(32, 31), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(&mut display)?;

        // Draw centered text for arc
        let text = format!("{}%", progress);
        let text_block = Text::with_text_style(
            &text,
            Point::new(32, 31),
            character_style,
            text_style,
        );
        text_block.draw(&mut display)?;

        // commenting this line will allow code to run. Error is:
        // thread 'main' (15355787) panicked at /Users/johnwalmsley/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embedded-graphics-simulator-0.7.0/src/window/mod.rs:172:14:
        // called `Option::unwrap()` on a `None` value
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        for event in window.events(){

        }

        // for event in window.events(){
        //     match event{
        //         SimulatorEvent::KeyDown { keycode, keymod, repeat } => { 
        //             match keycode{ 
        //                 Keycode::Space => {// Draw DOSE text
        //                     let dose_text = "DOSE";
        //                     let dose_text_block = Text::with_text_style(
        //                         &dose_text,
        //                         Point::new(96, 15),
        //                         character_style,
        //                         text_style,
        //                     );
        //                     dose_text_block.draw(&mut display)?;
        //                 },
        //                 _ => { }
        //             }
        //         },
        //         _ => {}
        //     }
        // }

        // Draw breaths remaining text
        let est_remaining_text = format!("{} LEFT", remaining);
        let est_remaining_text_block = Text::with_text_style(
            &est_remaining_text,
            Point::new(96, 47),
            character_style,
            text_style,
        );
        est_remaining_text_block.draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % 101;
    }
}