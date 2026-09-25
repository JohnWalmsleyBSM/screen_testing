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
        // TODO calculate breaths remaining based on duration of last breath.
        let remaining = ( 101 - progress ) / 12 as u16;
    
        // Draw an arc with a 5px wide stroke.
        let _arc = Arc::with_center(Point::new(32, 31), 64 - 4, 90.0.deg(), sweep.deg())
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

    // Draws the dose complete state
fn draw_dose_complete_state<D>(target: &mut D ) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
    {
        // Create styles used by the drawing operations.
        // TODO: Pass in or make constant
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Center)
            .build();
        target.clear(BinaryColor::Off)?;  
    
        let dose_text = "DOSE";
        let dose_text_block = Text::with_text_style(
            &dose_text,
            Point::new(64, 15),
            character_style,
            text_style,
        );
        dose_text_block.draw(target)?;
    
        // Draw breaths remaining text
        let est_remaining_text = "COMPLETE";
        let est_remaining_text_block = Text::with_text_style(
            &est_remaining_text,
            Point::new(64, 47),
            character_style,
            text_style,
        );
        est_remaining_text_block.draw(target)?;
        Ok(())
    }

// Draws the waiting state
fn draw_waiting_state<D>(target: &mut D ) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
    {
        // Create styles used by the drawing operations.
        // TODO: Pass in or make constant
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Center)
            .build();
        target.clear(BinaryColor::Off)?;  
    
        let dose_text = "NEXT DOSE";
        let dose_text_block = Text::with_text_style(
            &dose_text,
            Point::new(64, 15),
            character_style,
            text_style,
        );
        dose_text_block.draw(target)?;
    
        // Draw breaths remaining text
        let est_remaining_text = format!("{} HR", 1);
        let est_remaining_text_block = Text::with_text_style(
            &est_remaining_text,
            Point::new(64, 47),
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
        .theme(BinaryColorTheme::Default)
        .build();
    let mut window = Window::new("Progress", &output_settings);
       
    // Initialize timers
    let mut progress: u16 = 0;
    let mut startup_timer: u16 = 0;
    let mut power_off_timer = 0;
    let mut power_off_timer_enable = false;
    let mut power_off_timer_complete = false;

    // Device starts in the Off state
    let mut current_state = States::Off;
    draw_startup_state(&mut display);
    // Must update first, otherwise, calls to window.events() will panic.
    window.update(&display);

    'running: loop {
        // Get a keydown event to control state
        for event in window.events(){
            match event{
                SimulatorEvent::KeyDown { keycode, .. } => { 
                    match keycode{
                        // Button press down starts timer - >~3s powers off
                        Keycode::Space => {
                            power_off_timer_enable = true;
                        },
                        Keycode::B =>{ // Breathing
                            match current_state{
                                States::Dosing => {progress = progress+2; },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                },
                SimulatorEvent::KeyUp { keycode, .. } => {
                    match keycode{
                        Keycode::Space => { // releasing button stops timer and resets
                            power_off_timer_enable = false;
                            power_off_timer = 0;
                            if power_off_timer_complete { // power off timer expired due to long spacebar press
                                current_state = States::Off;
                                startup_timer = 0;
                                progress = 0;
                                power_off_timer_complete = false;
                            }
                            else { // State transition
                                match current_state{
                                    States::Off => { current_state = States::Startup;},
                                    States::Startup => { }, // Startup cannot be interrupted
                                    States::Waiting => { current_state = States::Dosing },
                                    States::Dosing => { }, // Button does nothing until dose complete
                                    States::DoseComplete => { current_state = States::Waiting }, // Send back to Dose State for now
                                }
                            }
                        },
                        _ => {},
                    };
                    
                },
                SimulatorEvent::Quit => { break 'running Ok(()); },
                _ => {}
            }
        }
        
        // Update screen based on current state
        match current_state{ 
            States::Off => { draw_off_state(&mut display); },
            States::Startup => { draw_startup_state(&mut display); },
            States::Waiting => { draw_waiting_state(&mut display); },
            States::Dosing => { draw_dosing_state(&mut display, &progress); },
            States::DoseComplete => { draw_dose_complete_state(&mut display); },
        }
 
        // Update timers, and states if transition reached.
        match current_state{
            States::Startup =>{ startup_timer = startup_timer + 1;
                                // When timer expires, go to dosing and reset timer.
                                if startup_timer == 10{
                                    current_state = States::Waiting;
                                    startup_timer = 0;
                                };
                            },
            States::Dosing =>{ if progress == 100 {
                                    current_state = States::DoseComplete;
                                    progress = 0;
                                };
                            },
            _ => {},
        }
        // Update power off timer if enabled.
        if power_off_timer_enable {
            power_off_timer = power_off_timer + 1;
            if power_off_timer > 10{
                power_off_timer_complete = true;
            }
        }

        window.update(&display);
        
        thread::sleep(Duration::from_millis(50));

    }
}