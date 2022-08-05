pub use fltk::frame::Frame;
use fltk::{button::Button, window::DoubleWindow};
//use fltk::text::TextDisplay;
pub use fltk::{
    app::{self}, //run},
    window,
};
pub use fltk::{button, prelude::*};
pub use std::{
    sync::{Arc, Mutex},
    thread,
    thread::{sleep, Thread},
    time::Duration,
};
pub fn retry_button(
    counter: Arc<Mutex<i32>>,
    clicked: Arc<Mutex<bool>>,
    f: Arc<Mutex<Frame>>,
    button: Arc<Mutex<Button>>,
) -> Button {
    let mut b = button::Button::default()
        .with_size(50, 35)
        .with_pos(180, 455)
        .with_label("retry");
    b.set_callback(move |c| {
        let counter1 = Arc::clone(&counter);
        let clicked1 = Arc::clone(&clicked);
        let f1 = Arc::clone(&f);
        let button1 = Arc::clone(&button);
        timer_thread(clicked1, button1, f1, counter1);
        let mut count = counter.lock().unwrap();
        *count = 0;
        let mut click = clicked.lock().unwrap();
        *click = false;
        let mut frame = f.lock().unwrap();
        frame.set_label("CPS Tester");
        let mut but = button.lock().unwrap();
        but.activate();
        app::background(190, 190, 190);
    });
    b
}
pub fn timer_thread(
    clicked_wait: Arc<Mutex<bool>>,
    new_cps_arc: Arc<Mutex<Button>>,
    out_of_time_clone: Arc<Mutex<Frame>>,
    cps_counter: Arc<Mutex<i32>>,
) {
    thread::spawn(move || {
        while true {
            // waits intil the user has clicked the button
            let click = clicked_wait.lock().unwrap();
            if *click {
                println!("broken"); // notifies if It has been clicked test only
                break;
            }
        }
        thread::sleep(Duration::from_secs(5));
        // once out of time it will unwrap the button and disable it
        let mut cps_button_clock = new_cps_arc.lock().unwrap();
        cps_button_clock.deactivate();
        // ---------------------------------------------------------
        let mut out_of_time = out_of_time_clone.lock().unwrap(); // just the label
        let cps_tracker = cps_counter.lock().unwrap();
        out_of_time.set_label(&format!(
            "OUT OF TIME\n CLICKS: {}\n CPS:{}",
            *cps_tracker,
            *cps_tracker / 5
        ));
        println!("Out of time");
        println!("CPS: {}", *cps_tracker / 5);
        println!("Clicks: {}", *cps_tracker);
    });
}
