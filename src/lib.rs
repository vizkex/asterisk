use wgpu::{
    core::device::{self, queue},
    Surface,
};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
    window::WindowBuilder,
};
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window msut be declared after the surface so
    // it gets dropped after is as the surface contains
    // unsafe refernces to window`s resources
    window: &'a Window,
}
impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        todo!()
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }
    fn update(&mut self) {
        todo!()
    }
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    });
}
