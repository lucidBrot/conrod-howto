/// A Hello World based and annotated with help of https://docs.piston.rs/conrod/src/conrod_core/guide/chapter_3.rs.html
use conrod::backend::glium::glium;
use conrod::{widget_ids, widget, Positionable, Colorable, Widget};

/*
 `Surface` is a trait required for glium, specifically for the call to
`target.clear_color` which is coming later.
 */
use glium::Surface;

use image;

/*
 Support contains much boilerplate code for the event loop. It constrains e.g. it's rate to be only 60 FPS.
 */
mod support;

mod conrod_example_shared;

/*
 The first chunk of boilerplate creates an event loop, which will handle
interaction with the UI, then a window, then a context, then finally links the
event loop, window and context together into a display. The display is the
home for the UI, and is an OpenGL context provided by glium.
*/

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;
const TITLE: &str = "Hello Conrod!";


pub fn main() {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    /*
       Now create the UI itself. Conrod has a builder that contains and looks after
       the UI for the user.
       */
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    /*
       Boilerplate code to load fonts into the Ui's font::Map
       */
    const FONT_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();


    // Generate the widget identifiers.
    ////widget_ids!(struct Ids { text });
    ////let ids = Ids::new(ui.widget_id_generator());
    // The `widget::Id` of each widget instantiated in `conrod_example_shared::gui`.
    let ids = conrod_example_shared::Ids::new(ui.widget_id_generator());

    // Load the Rust logo from our assets folder to use as an example image.
    fn load_rust_logo(display: &glium::Display) -> conrod::glium::texture::Texture2d {
        const RUST_LOGO_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/images/rust.png");
        let rgba_image = image::open(&std::path::Path::new(&RUST_LOGO_PATH)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = conrod::glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let texture = conrod::glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }

    /*
       Conrod can use graphics. It stores these in a map. The system needs the map,
       even though it doesn't contain anything at this time, so create it:
       */
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let rust_logo = image_map.insert(load_rust_logo(&display));


    // A demonstration of some app state that we want to control with the conrod GUI.
    let mut app = conrod_example_shared::DemoApp::new(rust_logo);

    /*
       Finally, Conrod needs to render its UI. It uses a renderer to do this, so
       create one:
       */
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // Start the loop that is in support.rs module
    // It still requires the original events_loop we defined at the start of main()
    //
    // - Poll the window for available events.
    // - Update the widgets via the `conrod_example_shared::gui` fn.
    // - Render the current state of the `Ui`.
    // - Repeat.
    let mut event_loop = support::EventLoop::new();

    'render: loop {
        // Process the events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            // Break from the loop upon `Escape` or closed window.
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => { break 'render },
                        _ => (),
                    }
                }
                _ => (),
            };
        }

        let ui = &mut ui.set_widgets();

        // Add some Hello World Text
        // "Hello World!" in the middle of the screen.
        ////widget::Text::new("Hello World!")
        ////    .middle_of(ui.window)
        ////    .color(conrod::color::WHITE)
        ////    .font_size(32)
        ////    .set(ids.text, ui);

        // Instantiate a GUI demonstrating every widget type provided by conrod.
        conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);

        // Draw the UI if it has changed
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
