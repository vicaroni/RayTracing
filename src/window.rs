use core::num::NonZeroU32;
use winit::window::Window;
use softbuffer::{
    Context,
    Surface
};

pub struct GraphicsContext {
    surface: Surface,
}

impl GraphicsContext {

    pub fn new(window: &Window) -> Self {
        let context = unsafe { Context::new(window)}.expect("Failed to create a softbuffer context");
        let mut surface = unsafe { Surface::new(&context, window)}.expect("Failed to create a softbuffer surface");
        let size = window.inner_size();
        surface.resize(NonZeroU32::new(size.width).unwrap(), NonZeroU32::new(size.height).unwrap()).expect("Failed to resize the softbuffer surface");
        let mut buffer = surface.buffer_mut().expect("Failed to get the softbuffer buffer");
        buffer.fill(0);
        buffer.present().expect("Failed to present the softbuffer buffer");
        Self { surface }
    }

    pub fn draw_pixel(&mut self, i: usize, color: u32) {
        let mut buffer = self.surface.buffer_mut().expect("Failed to get the softbuffer buffer");
        if let Some(pixel) = buffer.get_mut(i){
            *pixel = color;
            buffer.present().expect("Failed to present the softbuffer buffer");
        } else {
            println!("{}: get_mut failed", i);
        }
    }
}
