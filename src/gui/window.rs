// A simple example that demonstrates using conrod within a basic `winit` window loop, using
// `glium` to render the `conrod::render::Primitives` to screen.

use conrod::{widget, color, Colorable, Positionable, Widget, Sizeable, Borderable, Labelable};
use conrod::backend::glium::glium::{self, Surface};
use gui;
use std::sync::mpsc::Sender;
use controller::Command;

enum Osc { Sine, Saw, Supersaw }

struct AppState {
    pub title: String,
    pub oscillator_sel: Option<usize>,
    pub oscillator_list: Vec<String>,
    pub oscillator: Osc,
}
impl AppState {
    fn new() -> AppState {
        AppState {
            title: "Sintetizador Maravilhoso".to_string(),
            oscillator_sel: None,
            oscillator_list: vec!["Sine".to_string(), "Saw".to_string(), "Supersaw".to_string()],
            oscillator: Osc::Sine,
        }
    }
}

widget_ids!(struct Ids { text, canvas, oscillator_sel });

pub fn show(cmd_out: Sender<Command>) {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;
    let mut app = AppState::new();

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut framework = gui::EventLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(app.title.clone())
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/VT323-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

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

            gui::keymap::command_for(&input)
                .into_iter().map(|c| cmd_out.send(c))
                .collect::<Result<Vec<_>, _>>().unwrap(); //TODO propagate up

            // Handle the input with the `Ui`.
            ui.handle_event(input);

            // Set the widgets.
            let ui = &mut ui.set_widgets();
            set_widgets(ui, &mut app, &mut ids);
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

fn set_widgets(ui: &mut conrod::UiCell, app: &mut AppState, ids: &mut Ids) {

    widget::Canvas::new()
        .border(1.)
        .pad(30.)
        .color(color::BLACK)
        .scroll_kids()
        .set(ids.canvas, ui);

    for selected_idx in widget::DropDownList::new(&app.oscillator_list, app.oscillator_sel)
        .w_h(100., 20.)
        .top_left_of(ids.canvas)
        .max_visible_items(3)
        .color(color::BLACK)
        .border(1.)
        .border_color(color::WHITE)
        .label("Oscillator")
        .label_color(color::WHITE)
        .label_font_size(14)
        .scrollbar_on_top()
        .set(ids.oscillator_sel, ui){

        app.oscillator_sel = Some(selected_idx);
        app.oscillator = match &app.oscillator_list[selected_idx][..] {
            "Sine" => Osc::Sine,
            "Saw" => Osc::Saw,
            "Supersaw" => Osc::Supersaw,
            other => panic!("Unexpected oscillator name: {}", other),
        }
    }

}