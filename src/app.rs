extern crate cairo;
extern crate gdk;
extern crate gtk;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Button, DrawingArea, Label, Orientation, Window, WindowType};

use gdk::EventMask;

pub struct Node {
    pub position: (f64, f64),
}

pub struct Model {
    pub count: i32,
    pub nodes: Vec<Node>,
}

pub struct App {
    pub model: Rc<RefCell<Model>>,
    pub window: Window,
    pub vbox: gtk::Box,
    pub toolbar: gtk::Box,
    pub toolbar_add_node_btn: Button,
    pub toolbar_add_rel_btn: Button,
    pub drawing_area: DrawingArea,
    pub label: Label,
    pub button: Button,
}

impl App {
    pub fn new() -> App {
        let model = Model {
            count: 0,
            nodes: vec![
                Node {
                    position: (80.0, 80.0),
                },
                Node {
                    position: (150., 200.),
                },
            ],
        };

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Graph Editor");
        window.set_default_size(500, 500);

        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        window.add(&vbox);

        let toolbar_hbox = gtk::Box::new(Orientation::Horizontal, 0);
        let add_node_btn = Button::new_with_label("Add Node");
        toolbar_hbox.pack_start(&add_node_btn, false, false, 0);
        let add_rel_btn = Button::new_with_label("Add Relationship");
        toolbar_hbox.pack_start(&add_rel_btn, false, false, 0);
        vbox.pack_start(&toolbar_hbox, false, false, 0);

        let drawing_area = DrawingArea::new();
        drawing_area.set_events(EventMask::BUTTON_PRESS_MASK.bits() as i32);
        vbox.pack_start(&drawing_area, true, true, 0);

        let label = Label::new(Some("0"));
        vbox.pack_start(&label, false, false, 0);

        let button = Button::new_with_label("Increment");
        vbox.pack_start(&button, false, false, 0);

        App {
            model: Rc::new(RefCell::new(model)),
            window: window,
            vbox: vbox,
            toolbar: toolbar_hbox,
            toolbar_add_node_btn: add_node_btn,
            toolbar_add_rel_btn: add_rel_btn,
            drawing_area: drawing_area,
            label: label,
            button: button
        }
    }
}