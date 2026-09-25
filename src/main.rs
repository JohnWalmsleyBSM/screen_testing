use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    image::Image,
    prelude::*,
    primitives::{Arc, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window
};
use embedded_graphics_simulator::sdl2::Keycode;
use tinybmp::Bmp;
use std::{thread, time::Duration};

enum States {
    Off,
    Startup,
    Waiting,
    Dosing,
    DoseComplete
}

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
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
    // Otherwise, calls to window.events() will panic.
    window.update(&display);
    // The current progress percentage
    let mut progress = 0;

    // Device starts in the Off state
    let mut current_state = States::Off;

    'running: loop {
        // Get a keydown event to control state
        for event in window.events(){
            match event{
                SimulatorEvent::KeyDown { keycode, keymod, repeat } => { 
                    match keycode{
                        // Button press changes state
                        Keycode::Space => {
                            match current_state{
                                States::Off => { current_state = States::Startup;},
                                States::Startup => { current_state = States::Off;},
                                _ => { current_state = States::Off;}
                            }
                        },
                        //TODO: B = breathing
                        _ => {},
                    }
                },
                SimulatorEvent::Quit => { break 'running Ok(()); },
                _ => {}
            }
        }
        // Update screen based on current state

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

        match current_state{ 
            States::Off => {},
            States::Startup => {
                let dose_text = "DOSE";
                let dose_text_block = Text::with_text_style(
                    &dose_text,
                    Point::new(96, 15),
                    character_style,
                    text_style,
                );
                dose_text_block.draw(&mut display)?;
            },
            _ =>{},
        }
 
        // Draw breaths remaining text
        let est_remaining_text = format!("{} LEFT", remaining);
        let est_remaining_text_block = Text::with_text_style(
            &est_remaining_text,
            Point::new(96, 47),
            character_style,
            text_style,
        );
        est_remaining_text_block.draw(&mut display)?;

        // draw image over the top...
        // let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../assets/BridgeSource_64_128.bmp")).unwrap();
        // let image = Image::new(&bmp, Point::new(0, 0));
        // // Display the image
        // image.draw(&mut display)?;

        window.update(&display);

        // if window.events().any(|e| e == SimulatorEvent::Quit) {
        //     break 'running Ok(());
        // }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % 101;
    }
}