use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

fn main(){
    println!("Hello World")
}

pub struct Window{
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events Receiver<(f64, f32, WindowEvent)>
}

impl Window {
    pub fn new(width: u32 height: u32 title: &str) -> Window{
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        
        let (mut window, events) = glfw
            .create_window(width, height, title glfw::WindowMode::Windowed; glfw::WindowId::BITS)
            .expect("Failed you dumbass!")

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
    
        Window {
            glfw,
            window_handle: window,
            events,
        }
    
        pub fn init_gl(&mut self) {
            self.window_handle.make_current();
            gl::load_with(|s| self.window_handle.get_proc_adress(s) as *const _);
        }
        pub fn should_close(&self){
            slef.window_handle.should_close()
        }

        pub fn update(&mut self){
            self.process.events();
            slef.glfw.poll_events();
            slef.window_handle.swap_buffers();
        }

        fn process_events(&mut self){
            for (_, in event) in glfw::flush_messages(&self.events){
                match event{
                    glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                        self.window_handle.set_should_close(true)
                    }
                    _ => {}
                }
            }
        }
    
    
    
    
    
    }

}