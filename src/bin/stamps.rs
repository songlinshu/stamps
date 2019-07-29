extern crate sdl2;

use std::time;
use std::string::String;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use sdl2::event::Event;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
static DESIRED_DURATION_PER_FRAME:time::Duration = time::Duration::from_millis(4);

const MOUSE_CONSTANT: i32 = 1;
struct SceneGraph {
    
}
struct SceneState{
    scene_graph: SceneGraph,
    mouse_x: i32,
    mouse_y: i32,
    cursor: Cursor,
}
impl SceneState {
    fn render<T:sdl2::render::RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(),String> {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        canvas.fill_rect(Rect::new(self.mouse_x, self.mouse_y, 1, 1))?;
        canvas.present();
        Ok(())
    }
    fn apply_keys(&mut self, keys_down: &HashMap<Keycode, ()>) {
        if keys_down.contains_key(&Keycode::Left) {
            self.mouse_x -= MOUSE_CONSTANT;
        }
        if keys_down.contains_key(&Keycode::Right) {
            self.mouse_x += MOUSE_CONSTANT;
        }
        if keys_down.contains_key(&Keycode::Up) {
            self.mouse_y -= MOUSE_CONSTANT;
        }
        if keys_down.contains_key(&Keycode::Down) {
            self.mouse_y += MOUSE_CONSTANT;
        }
    }
}

fn process<T:sdl2::render::RenderTarget>(state: &mut SceneState, event: sdl2::event::Event, canvas: &mut sdl2::render::Canvas<T>, keys_down: &mut HashMap<Keycode, ()>) -> Result<bool,String>{
    let mut key_encountered = false;
    match event {
        Event::Quit{..} => return Err("Exit".to_string()),
        Event::KeyDown {keycode: Option::Some(key_code), ..} =>{
            if let None = keys_down.insert(key_code, ()) {
                for (key,_)in keys_down.iter() {
                    eprintln!("Key is down {}\n", *key)
                }
            }
            key_encountered = true;
            state.apply_keys(&keys_down);
        },
        Event::KeyUp {keycode: Option::Some(key_code), ..} =>
        {
            keys_down.remove(&key_code);
        },
        Event::MouseButtonDown {x, y, ..} => {
            state.mouse_x = x;
            state.mouse_y = y;
        }
        Event::MouseMotion {x, y, ..} => {
            state.mouse_x = x;
            state.mouse_y = y;
        }
        _ => {}
    }
    state.render(canvas)?;
    Ok(key_encountered)
}

pub fn run(png: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let mut keys_down = HashMap::<Keycode, ()>::new();
    let surface = Surface::from_file(png)
        .map_err(|err| format!("failed to load cursor image: {}", err))?;
    let mut scene_state = SceneState {
        scene_graph:SceneGraph{},
        mouse_x:0,
        mouse_y:0,
        cursor:Cursor::from_surface(surface, 0, 0).map_err(
            |err| format!("failed to load cursor: {}", err))?,
    };
    scene_state.cursor.set();
    'mainloop: loop {
        let loop_start_time = time::Instant::now();
        let mut events = sdl_context.event_pump()?;
        if keys_down.len() != 0 {
            for event in events.poll_iter() {
                process(&mut scene_state, event, &mut canvas, &mut keys_down)?; // always break
            }
            let process_time = loop_start_time.elapsed();
            if keys_down.len() != 0 && process_time < DESIRED_DURATION_PER_FRAME {
                std::thread::sleep(DESIRED_DURATION_PER_FRAME - process_time);
                scene_state.apply_keys(&keys_down);
                scene_state.render(&mut canvas)?;
            }
        } else {
            for event in events.wait_iter() {
                if process(&mut scene_state, event, &mut canvas, &mut keys_down)? {
                    break;
                }
            }
        };
    }
}


fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)");
        Ok(())
    } else {
        let ret = run(Path::new(&args[1]));
        match ret {
            Err(x) => {
                if x == "Exit" {
                    Ok(())
                } else {
                    Err(x)
                }
            },
            ret => ret,
        }
    }
}
