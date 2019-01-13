// A simple example that demonstrates using conrod within a basic `winit` window loop, using
// `glium` to render the `conrod::render::Primitives` to screen.

use conrod::{widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium::{self, Surface};
use crate::gui;
use std::sync::mpsc::Sender;
use crate::core::control::manual_controller::Command;

pub fn show(cmd_out: Sender<Command>) {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut framework = gui::EventLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Sintetizador Maravilhoso")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/VT323-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let keymap = gui::keymap::KeyMap::new(WIDTH, HEIGHT);

    'render: loop {
        let mut events = framework.next(&mut events_loop);

        // Process the events.
        for event in events.drain(..) {

            // Break from the loop upon `Escape` or closed window.
            if framework.should_exit(&event) {
                break 'render
            }

            // Use the `winit` backend feature to convert the winit event to a conrod input.
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => {
                    framework.needs_update();
                    input
                },
            };

            keymap.command_for(&input)
                .into_iter().map(|c| cmd_out.send(c))
                .collect::<Result<Vec<_>, _>>().unwrap(); //TODO propagate up

            // Handle the input with the `Ui`.
            ui.handle_event(input);

            // Set the widgets.
            let ui = &mut ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Sintetizador\nMaravilhoso")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
        }

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}