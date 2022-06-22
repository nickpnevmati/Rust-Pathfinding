#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
mod pathfinding;
mod scene_renderer;

use gtk::{gdk::EventMask, glib::signal, prelude::*, ApplicationWindow, Builder, DrawingArea};
use scene_renderer::scene::Object;

use crate::scene_renderer::scene::SCENE;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../res/MainWindow.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));

    let button: gtk::Button = builder.object("button").expect("Error");
    button.connect_clicked(move |_| {
        for i in 0..10 {
            for j in 0..10 {
                let o: Object = Object::new()
                    .with_image("./res/test_image.png")
                    .with_position((i as f64 * 0.1, j as f64 * 0.1))
                    .with_scale((0.1, 0.1));
                SCENE.lock().unwrap().instantiate(o, None);
            }
        }
    });

    let draw_area: DrawingArea = builder
        .object("drawing_area")
        .expect("Couldn't find drawing area");
    draw_area.add_events(EventMask::BUTTON_PRESS_MASK);
    draw_area.connect_draw(drawing_function);
    draw_area.connect_button_press_event(on_button_clicked);

    window.present();
}

fn drawing_function(area: &gtk::DrawingArea, cr: &gtk::cairo::Context) -> signal::Inhibit {
    let width = area.allocated_width();
    let height = area.allocated_height();

    SCENE
        .lock()
        .unwrap()
        .update(cr, (width as f64, height as f64));

    return signal::Inhibit(false);
}

fn on_button_clicked(
    area: &gtk::DrawingArea,
    event_button: &gtk::gdk::EventButton,
) -> signal::Inhibit {
    let coords = event_button.coords().unwrap();
    let size = (area.allocated_width() as f64, area.allocated_height() as f64);
    let normalized_size = (coords.0 / size.0, coords.1 / size.1);
    
    let lck = SCENE.lock().unwrap();
    let evnt = lck.send_event(normalized_size);
    if evnt != None {
        println!("{}", evnt.unwrap().id);
    } else {
        println!("No object clicked");
    }
    area.queue_draw();
    return signal::Inhibit(false);
}
