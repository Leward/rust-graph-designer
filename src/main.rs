extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use cairo::Context;
use gtk::{Button, DrawingArea, Label, Orientation, Window, WindowType};

use gdk::{ EventButton, EventMask, EventType};

// use std::cell::Ref;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

struct Node {
    position: (f64, f64),
}

struct Model {
    count: i32,
    nodes: Vec<Node>,
}

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Graph Editor");
    window.set_default_size(500, 500);

    let model = Rc::new(RefCell::new(Model {
        count: 0,
        nodes: vec![
            Node {
                position: (80.0, 80.0),
            },
            Node {
                position: (150., 200.),
            },
        ],
    }));

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.set_homogeneous(true);
    window.add(&vbox);

    let drawing_area = DrawingArea::new();
    drawing_area.set_events(EventMask::BUTTON_PRESS_MASK.bits() as i32);
    vbox.add(&drawing_area);

    let label = Label::new(Some("0"));
    vbox.add(&label);

    let button = Button::new_with_label("Increment");
    vbox.add(&button);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let model = model.clone();
        button.connect_clicked(move |_| {
            {
                (*model.borrow_mut()).count += 1;
            }
            label.set_text(&format!("{}", (*model.borrow()).count));
        });
    }

    {
        let model = model.clone();
        drawing_area.connect_draw(move |da, cr| draw_fn(da, cr, &model));
    }

    // A click in the drawing area creates a node in that position and ask for a redraw
    {
        let model = model.clone();
        drawing_area.connect_event(move |da, event| match event.get_event_type() {
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

    cr.set_source_rgba(0.77, 0.77, 0.77, 1.0);
    cr.rectangle(0.0, 0.0, width, height);
    cr.fill();

    let rect = |x, y, width, height| {
        let point = cr.device_to_user(x, y);
        let size = cr.device_to_user(width, height);
        cr.rectangle(point.0, point.1, size.0, size.1);
        cr.fill();
    };

    let draw_node = |x, y| {
        let point = cr.device_to_user(x, y);
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
