use sdl3::event::Event;
use sdl3::gpu::{Device, ShaderFormat};
use sdl3::keyboard::Keycode;
use shame_sdl3::Gpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl3::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window("Shame SDL3 Demo", 1280, 720)
        .position_centered()
        .build()?;
    let device =
        Device::new(ShaderFormat::SpirV, true)?.with_window(&window)?;
    let command_buffer = device.acquire_command_buffer()?;
    let texture_format = device.get_swapchain_texture_format(&window);
    let _gpu = Gpu::new(command_buffer, device, Some(texture_format));

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
