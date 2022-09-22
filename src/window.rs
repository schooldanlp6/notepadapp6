use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

  #![allow(unused)]
  fn main() {
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
      sdl
        .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
        .unwrap();
    }
  }

  #![allow(unused)]
  fn main() {
    let _win = sdl
      .create_gl_window(
        "Hello Window",
        WindowPosition::Centered,
        800,
        600,
        WindowFlags::Shown,
      )
      .expect("couldn't make a window and context");
  }

  #![allow(unused)]
  fn main() {
    'main_loop: loop {
      // handle events this frame
      while let Some(event) = sdl.poll_events().and_then(Result::ok) {
        match event {
          Event::Quit(_) => break 'main_loop,
          _ => (),
        }
      }
      // now the events are clear
  
      // here's where we could change the world state and draw.
    }
  }
}      