use sdl2::event::{Event, WindowEvent};
use sdl2::video::Window;
use render::WindowSize;
use render_gl::Viewport;
use gl;

#[derive(PartialEq)]
pub enum HandleResult {
    Quit,
    Continue,
}

pub fn handle_default_window_events(event: &Event, gl: &gl::Gl, window: &Window, window_size: &mut WindowSize, viewport: &mut Viewport) -> HandleResult {
    match event {
        Event::Quit {..} => return HandleResult::Quit,
        Event::Window {
            win_event: WindowEvent::Resized(w, h),
            ..
        } => {
            let (hdpi_w, hdpi_h) = window.drawable_size();
            viewport.update_size(hdpi_w as i32, hdpi_h as i32);
            viewport.set_used(&gl);
            *window_size = WindowSize {
                width: *w as i32,
                height: *h as i32,
                highdpi_width: hdpi_w as i32,
                highdpi_height: hdpi_h as i32
            };
            //camera.update_aspect(hdpi_w as f32 / hdpi_h as f32);
        },
        _ => (),
    };

    HandleResult::Continue
}