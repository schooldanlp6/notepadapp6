fn main(){
    println!("Hello World");
    let mut window = window::Window::new(720, 480, "Testwindow");

    window.init_gl();

    while !window.should_close(){
        unsafe {
            gl::ClearColor(0.0, 2.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        window.update();
    }

}