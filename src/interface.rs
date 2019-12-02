use crate::{create_surface, AllData};
use gdk;
use glib;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow};
use std::rc::Rc;

struct Labels {
    elevation_label: gtk::Label,
    distance_label: gtk::Label,
    latitude_label: gtk::Label,
    longitude_label: gtk::Label,
}

fn as_dms(ang: f64) -> (usize, usize, usize) {
    let ang = ang.abs();
    let deg = ang as usize;
    let min = ((ang - deg as f64) * 60.0) as usize;
    let sec = ((ang - deg as f64 - min as f64 / 60.0) * 3600.0) as usize;
    (deg, min, sec)
}

fn create_drawing_area(data: Rc<AllData>, labels: Labels) -> gtk::DrawingArea {
    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_events(gdk::EventMask::BUTTON_PRESS_MASK);

    let img_surface = create_surface(&data);

    drawing_area.connect_draw(move |_area, cr| {
        cr.set_source_surface(&img_surface, 0.0, 0.0);
        cr.paint();
        glib::signal::Inhibit(true)
    });

    let data2 = data.clone();
    drawing_area.connect_button_press_event(move |_area, ev_button| {
        let (x, y) = ev_button.get_position();
        let x = x as usize;
        let y = y as usize;

        if let Some(pixel) = data2.result[y][x] {
            let elevation = format!("Elevation: {:.1} m", pixel.elevation);
            let distance = if pixel.distance > 1000.0 {
                format!("Distance: {:.1} km", pixel.distance / 1000.0)
            } else {
                format!("Distance: {:.1} m", pixel.distance)
            };
            let lat = as_dms(pixel.lat);
            let lon = as_dms(pixel.lon);

            let latitude = format!(
                "Latitude: {}°{}'{}\"{} ({:.6})",
                lat.0,
                lat.1,
                lat.2,
                if pixel.lat >= 0.0 { "N" } else { "S" },
                pixel.lat
            );
            let longitude = format!(
                "Longitude: {}°{}'{}\"{} ({:.6})",
                lon.0,
                lon.1,
                lon.2,
                if pixel.lon >= 0.0 { "E" } else { "W" },
                pixel.lon
            );

            labels.elevation_label.set_label(&elevation);
            labels.distance_label.set_label(&distance);
            labels.latitude_label.set_label(&latitude);
            labels.longitude_label.set_label(&longitude);
        } else {
            labels.elevation_label.set_label("Elevation: none");
            labels.distance_label.set_label("Distance: none");
            labels.latitude_label.set_label("Latitude: none");
            labels.longitude_label.set_label("Longitude: none");
        }

        glib::signal::Inhibit(true)
    });

    drawing_area.set_size_request(
        data.params.output.width as i32,
        data.params.output.height as i32,
    );

    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);

    drawing_area
}

fn create_layout(data: Rc<AllData>) -> gtk::Box {
    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let elev_label = gtk::Label::new(Some("Elevation: none"));
    let dist_label = gtk::Label::new(Some("Distance: none"));
    let lat_label = gtk::Label::new(Some("Latitude: none"));
    let lon_label = gtk::Label::new(Some("Longitude: none"));
    elev_label.set_xalign(0.0);
    dist_label.set_xalign(0.0);
    lat_label.set_xalign(0.0);
    lon_label.set_xalign(0.0);

    let labels = Labels {
        elevation_label: elev_label.clone(),
        distance_label: dist_label.clone(),
        latitude_label: lat_label.clone(),
        longitude_label: lon_label.clone(),
    };

    let drawing_area = create_drawing_area(data, labels);

    main_box.add(&drawing_area);

    let side_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    side_box.set_hexpand(true);

    let box_label = gtk::Label::new(Some("Data of clicked pixel:"));
    box_label.set_xalign(0.0);
    box_label.set_size_request(270, -1);

    let data_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    data_box.set_margin_top(20);

    data_box.add(&elev_label);
    data_box.add(&dist_label);
    data_box.add(&lat_label);
    data_box.add(&lon_label);

    side_box.add(&box_label);
    side_box.add(&data_box);

    main_box.add(&side_box);

    main_box.set_margin_top(5);
    main_box.set_margin_bottom(5);
    main_box.set_margin_start(5);
    main_box.set_margin_end(5);

    main_box
}

pub fn build_ui(app: &Application, data: Rc<AllData>) {
    let win = ApplicationWindow::new(app);
    //let renderer_rc = Rc::new(RefCell::new(Renderer::new(sim.clone(), 0.0, 0.0)));
    //let mouse_state = Rc::new(RefCell::new(MouseState::None));

    win.set_title("Panorama renderer");
    win.set_default_size(
        data.params.output.width as i32 + 300,
        data.params.output.height as i32 + 10,
    );

    let layout = create_layout(data);

    win.add(&layout);

    win.show_all();
}
