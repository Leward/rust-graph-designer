extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use cairo::Context;
use gtk::DrawingArea;

use gdk::{EventButton, EventType};

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

mod app;
use app::App;
use app::Model;
use app::Node;

fn main() {
    gtk::init().unwrap();

    let app = App::new();

    app.window.show_all();

    app.window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let model = app.model.clone();
        let button = app.button;
        let label = app.label;
        button.connect_clicked(move |_| {
            {
                (*model.borrow_mut()).count += 1;
            }
            label.set_text(&format!("{}", (*model.borrow()).count));
        });
    }

    {
        let model = app.model.clone();
        app.drawing_area.connect_draw(move |da, cr| draw_fn(da, cr, &model));
    }

    // A click in the drawing area creates a node in that position and ask for a redraw
    {
        let model = app.model.clone();
        app.drawing_area.connect_event(move |da, event| match event.get_event_type() {
            EventType::ButtonPress => {
                let event_btn = event.clone().downcast::<EventButton>().expect(
                    "Fail to downcast to lower level type of event (ButtonPress to EventButton)",
                );
                let node = Node {
                    position: event_btn.get_position(),
                };
                (*model.borrow_mut()).nodes.push(node);
                da.queue_draw();
                Inhibit(false)
            }
            _ => Inhibit(false),
        });
    }

    gtk::main();
}

fn draw_fn(da: &DrawingArea, cr: &Context, model: &Rc<RefCell<Model>>) -> gtk::Inhibit {
    let width = da.get_allocated_width() as f64;
    let height = da.get_allocated_height() as f64;
    // allocation describes size and position of the widget in the window
    // drawing to x in the canvas, means drawing to x + allocation.x in the window
    // it seems to be the values used by the drawing context
    let allocation = da.get_allocation();

    cr.set_source_rgba(0.77, 0.77, 0.77, 1.0);
    cr.rectangle(0.0, 0.0, width, height);
    cr.fill();

    let rect = |x, y, width, height| {
        let point = cr.device_to_user(x, y);
        let point = (point.0 + allocation.x as f64, point.1 + allocation.y as f64);
        cr.rectangle(point.0, point.1, width, height);
        cr.fill();
    };

    let draw_node = |x, y| {
        let point = cr.device_to_user(x, y);
        let point = (point.0 + allocation.x as f64, point.1 + allocation.y as f64);
        let distance = cr.device_to_user_distance(15.0, 15.0);
        cr.arc(point.0, point.1, distance.0, 0.0, PI * 2.0);
        cr.fill();
    };

    cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    rect(10.0, 10.0, 50.0, 50.0);

    let model = &*model.borrow();
    let iterator = model.nodes.iter();
    for node in iterator {
        draw_node(node.position.0, node.position.1);
    }

    Inhibit(false)
}
