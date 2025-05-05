use sdl3::event::Event;
use sdl3::keyboard::Keycode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl3::init()?;
    let video_subsystem = sdl.video()?;
    let _window = video_subsystem
        .window("Shame SDL3 Demo", 1280, 720)
        .position_centered()
        .build()?;

    let mut event_pump = sdl.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}
