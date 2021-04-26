use crate::{create_surface, AllData};
use gdk;
use glib;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

struct Labels {
    elevation_label: gtk::Label,
    distance_label: gtk::Label,
    latitude_label: gtk::Label,
    longitude_label: gtk::Label,
    dir_elev_label: gtk::Label,
    dir_azim_label: gtk::Label,
}

fn as_dms(ang: f64) -> (usize, usize, usize) {
    let ang = ang.abs();
    let deg = ang as usize;
    let min = ((ang - deg as f64) * 60.0) as usize;
    let sec = ((ang - deg as f64 - min as f64 / 60.0) * 3600.0) as usize;
    (deg, min, sec)
}

const CROSSHAIR_RADIUS: f64 = 12.0;
const CROSSHAIR_LINE_LEN: f64 = 20.0;

fn create_drawing_area(data: Rc<AllData>, labels: Labels) -> gtk::DrawingArea {
    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_events(gdk::EventMask::BUTTON_PRESS_MASK);

    let img_surface = create_surface(&data);

    let chosen_pixel: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));

    let chosen_pixel2 = chosen_pixel.clone();
    drawing_area.connect_draw(move |_area, cr| {
        cr.set_source_surface(&img_surface, 0.0, 0.0);
        cr.paint();

        if let Some((x, y)) = *chosen_pixel2.borrow() {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(1.0);
            cr.arc(x, y, CROSSHAIR_RADIUS, 0.0, 2.0 * PI);
            cr.stroke();

            cr.move_to(x, y - CROSSHAIR_LINE_LEN);
            cr.line_to(x, y + CROSSHAIR_LINE_LEN);
            cr.stroke();
            cr.move_to(x - CROSSHAIR_LINE_LEN, y);
            cr.line_to(x + CROSSHAIR_LINE_LEN, y);
            cr.stroke();
        }
        glib::signal::Inhibit(true)
    });

    let data2 = data.clone();
    let chosen_pixel3 = chosen_pixel.clone();
    drawing_area.connect_button_press_event(move |area, ev_button| {
        let (x, y) = ev_button.get_position();
        *chosen_pixel3.borrow_mut() = Some((x, y));
        area.queue_draw();
        let x = x as usize;
        let y = y as usize;

        // set direction data
        let (azim, elev) = data2.params.get_azim_and_elev(x, y);
        let dir_elevation = format!("Elevation: {:.3} deg", elev);
        let dir_azimuth = format!("Azimuth: {:.3} deg", azim);
        labels.dir_elev_label.set_label(&dir_elevation);
        labels.dir_azim_label.set_label(&dir_azimuth);

        if let Some(pixel) = data2.result[y][x] {
            let elevation = format!(
                "Elevation: {:.1} m ({:.0} ft)",
                pixel.elevation,
                pixel.elevation / 0.304
            );
            let si_distance = if pixel.distance > 1000.0 {
                format!("{:.1} km", pixel.distance / 1000.0)
            } else {
                format!("{:.1} m", pixel.distance)
            };
            let imperial_distance = if pixel.distance > 805.0 {
                format!("{:.1} mi", pixel.distance / 1609.0)
            } else {
                format!("{:.1} yds", pixel.distance / 0.912)
            };
            let distance = format!("Distance: {} ({})", si_distance, imperial_distance);
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

    let dir_elev_label = gtk::Label::new(Some("Elevation: none"));
    let dir_azim_label = gtk::Label::new(Some("Azimuth: none"));

    dir_elev_label.set_xalign(0.0);
    dir_azim_label.set_xalign(0.0);

    let labels = Labels {
        elevation_label: elev_label.clone(),
        distance_label: dist_label.clone(),
        latitude_label: lat_label.clone(),
        longitude_label: lon_label.clone(),
        dir_elev_label: dir_elev_label.clone(),
        dir_azim_label: dir_azim_label.clone(),
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

    let direction_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    direction_box.set_margin_top(10);

    data_box.add(&elev_label);
    data_box.add(&dist_label);
    data_box.add(&lat_label);
    data_box.add(&lon_label);

    direction_box.add(&dir_elev_label);
    direction_box.add(&dir_azim_label);

    side_box.add(&box_label);
    side_box.add(&data_box);
    side_box.add(&direction_box);

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
