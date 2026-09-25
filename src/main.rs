use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    image::Image,
    prelude::*,
    primitives::{Arc, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
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

// Draws the dosing state - spinning wheel currently
fn draw_dosing_state<D>(target: &mut D, progress: &u16 ) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
    {
        // Create styles used by the drawing operations.
        // TODO: Pass in or make constant
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
        target.clear(BinaryColor::Off)?;

        let sweep = *progress as f32 * 360.0 / 100.0;
        let remaining = ( 101 - progress ) / 10 as u16;
    
        // Draw an arc with a 5px wide stroke.
        let arc = Arc::with_center(Point::new(32, 31), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(target)?;
    
        // Draw centered text for arc
        let text = format!("{}%", progress);
        let text_block = Text::with_text_style(
            &text,
            Point::new(32, 31),
            character_style,
            text_style,
        );
        text_block.draw(target)?;
    
        let dose_text = "DOSE";
        let dose_text_block = Text::with_text_style(
            &dose_text,
            Point::new(96, 15),
            character_style,
            text_style,
        );
        dose_text_block.draw(target)?;
    
        // Draw breaths remaining text
        let est_remaining_text = format!("{} LEFT", remaining);
        let est_remaining_text_block = Text::with_text_style(
            &est_remaining_text,
            Point::new(96, 47),
            character_style,
            text_style,
        );
        est_remaining_text_block.draw(target)?;
        Ok(())
    }

/// Draws a blank screen
fn draw_off_state<D>(target: &mut D ) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
    {
        target.clear(BinaryColor::Off)?;
        Ok(())
    }

// Draws the BridgeSource logo
fn draw_startup_state<D>(target: &mut D ) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
    {
        target.clear(BinaryColor::Off)?;
        //draw image
        let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../assets/BridgeSource_64_128.bmp")).unwrap();
        let image = Image::new(&bmp, Point::new(0, 0));
        // Display the image
        image.draw(target)?;
        Ok(())
    }

// main
fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Progress", &output_settings);
       
    // Initialize timers
    let mut progress: u16 = 0;
    let mut startup_timer: u16 = 0;

    // Device starts in the Off state
    let mut current_state = States::Off;
    draw_startup_state(&mut display);
    // Must update first, otherwise, calls to window.events() will panic.
    window.update(&display);

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
                                States::Startup => { }, // Startup cannot be interrupted
                                States::Dosing => { current_state = States::Off;
                                                    startup_timer = 0;
                                                    progress = 0;
                                                }
                                _ => { current_state = States::Off;}
                            }
                        },
                        //TODO: B = breathing
                        //TODO: hold 3s for off
                        _ => {},
                    }
                },
                SimulatorEvent::Quit => { break 'running Ok(()); },
                _ => {}
            }
        }
        
        // Update screen based on current state
        match current_state{ 
            States::Off => { draw_off_state(&mut display); },
            States::Startup => { draw_startup_state(&mut display); },
            States::Dosing => { draw_dosing_state(&mut display, &progress); },
            _ =>{},
        }
 
        window.update(&display);

        // Update timers
        match current_state{
            States::Startup =>{ startup_timer = startup_timer + 1;
                                // When timer expires, go to dosing and reset timer.
                                if startup_timer == 10{
                                    current_state = States::Dosing;
                                    startup_timer = 0;
                                };
                            },
            States::Dosing => { progress = (progress + 1) % 101; }
            _ => {},
        }
        
        thread::sleep(Duration::from_millis(50));

    }
}