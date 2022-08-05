#![warn(unused_imports, non_snake_case)]
//use fltk::enums::Font;
use fltk::frame::Frame;
//use fltk::text::TextDisplay;
use fltk::{
    app::{self, background}, //run},
    window,
};
use fltk::{button, prelude::*};
use std::thread;
use std::{
    sync::{Arc, Mutex},
    //thread::{sleep, Thread},
    time::Duration,
};

mod cps;
fn main() {
    let mut counter = Arc::new(Mutex::new(0));
    let mut clicked = Arc::new(Mutex::new(false));
    let application = app::App::default(); // start of the gui
    let mut win = window::Window::default() // the gui
        .with_size(420, 500)
        .with_label("CPS Tester"); // title
    let frame = Frame::default() // frame that will show text
        .with_size(150, 150)
        .with_pos(135, 0)
        .with_label("CPS TEST");
    // app::set_font_size(34);
    app::set_font_size(14);
    app::background2(190, 190, 190);
    app::foreground(0, 0, 0);
    let mut frame_text = Arc::new(Mutex::new(frame.clone())); // mutex to change the title on multiple threads
    let clicker_button = button::Button::default()
        .with_size(155, 150)
        .with_label("Click")
        .with_pos(135, 300);
    let mut clicker_button_ARC = Arc::new(Mutex::new(clicker_button));
    // ------------------------------
    let clicker_clone = Arc::clone(&mut clicker_button_ARC);
    let counter_clone = Arc::clone(&mut counter);
    let clicked_arc = Arc::clone(&mut clicked);
    let mut clicker_thread_main = clicker_clone.lock().unwrap();
    clicker_thread_main.set_callback(move |_b| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        match *count {
            c if (c >= 100) => app::background(211, 33, 45),
            c if c >= 90 => app::background(241, 156, 187),
            c if c >= 75 => app::background(175, 0, 42),
            c if c >= 50 => app::background(255, 126, 0),
            c if c >= 45 => app::background(196, 98, 16),
            c if c >= 30 => app::background(176, 191, 26),
            c if c >= 25 => app::background(0, 48, 143),
            c if c >= 15 => app::background(0, 72, 186),
            c if c >= 10 => app::background(124, 185, 232),
            c if c >= 5 => app::background(114, 160, 193),
            c if c == 1 => app::background(201, 255, 229),
            _ => app::background2(245, 245, 245),
        }
        let mut c = clicked_arc.lock().unwrap();
        *c = true;
        println!("{}", *count)
    });
    let out_of_time_clone = Arc::clone(&mut frame_text);
    let cps_counter = Arc::clone(&counter);
    drop(clicker_thread_main);
    let new_cps_arc = Arc::clone(&mut clicker_button_ARC);
    let clicked_wait = Arc::clone(&clicked);
    cps::timer_thread(clicked_wait, new_cps_arc, out_of_time_clone, cps_counter);
    let b = cps::retry_button(
        Arc::clone(&mut counter),
        Arc::clone(&mut clicked),
        Arc::clone(&mut frame_text),
        Arc::clone(&mut clicker_button_ARC),
    );

    win.end();
    win.show();
    application.run().unwrap();
}
