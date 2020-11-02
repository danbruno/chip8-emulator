use std::fs::File;
use std::io::Read;
use cpu::Cpu;
use gpu::Gpu;

mod cpu;
mod gpu;
mod input;
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::time::{SystemTime, Instant};

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let rom = get_file_as_byte_vec("C:/IBM.ch8");
    let mut cpu = Cpu::new();
    let gpu = Gpu::new();

    println!("{:?}", rom);
    cpu.initialize();
    cpu.load(rom);

    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let mut is_maximized = false;

    let mut cpu_time = Instant::now();
    let mut countdown_time = Instant::now();
    let mut frequency = 600; //Hz
    let countdown_frequency = 60; // Hz

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now()
        );
        let cpu_elapsed_time = cpu_time.elapsed().as_micros();
        let countdown_elapsed_time = countdown_time.elapsed().as_micros();

        if cpu_elapsed_time > (1000000 / frequency) {
            cpu_time = Instant::now();
            cpu.tick();
        }

        if countdown_elapsed_time > (1000000 / countdown_frequency) {
            countdown_time = Instant::now();
            cpu.tick_timers();
        }

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(virtual_code),
                        state,
                        ..
                    },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => {
                        *control_flow = ControlFlow::Exit
                    }
                    (_,_) => cpu.process_key(virtual_code, state),
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
               // gl.draw_frame([0.0; 4]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
