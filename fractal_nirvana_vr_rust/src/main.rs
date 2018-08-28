extern crate winit;

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let _companion_window = winit::WindowBuilder::new()
        .with_title("Fractal Nirvava VR")
        .with_dimensions((1080,1200).into())
        .with_resizable(false)
        .build(&events_loop)
        .unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}
