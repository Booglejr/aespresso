mod about_window;
mod app_constants;
mod util;

extern crate gio;
extern crate gtk;
extern crate os_info;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{Application, ApplicationWindow, Box, Button, Frame, Label, ListBox};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

fn main() {
    let app_data = app_constants::get_app_data();

    let application = Application::new(Some(app_data.gtk_path), Default::default())
        .expect("Could not create application.");

    application.connect_activate(move |app| {
		let mut app_errors = app_constants::ErrMsg::new();
		app_errors.init();
		let app_errors_clone_a = app_errors.clone();
		let app_errors_clone_a2 = app_errors.clone();
		let app_errors_clone_a3 = app_errors.clone();
		let app_errors_clone_a4 = app_errors.clone();
		let app_errors_clone_b = app_errors.clone();
		let app_errors_clone_b2 = app_errors.clone();
		let app_errors_clone_b3 = app_errors.clone();
		
		let window = ApplicationWindow::new(app);
		window.set_title(app_data.title);
		window.set_default_size(300, 500);
		window.set_resizable(false);

		let root = Box::new(Vertical, 40);
		let about_window = about_window::new(app, &app_data);

		let app_quit_a = app.clone();
		let app_quit_a2 = app.clone();
		let app_quit_a3 = app.clone();
		let app_quit_a4 = app.clone();
		let app_quit_b = app.clone();
		let app_quit_b2 = app.clone();
		let app_quit_b3 = app.clone();
		let app_quit_b4 = app.clone();


		window.connect_delete_event(move |_,_|{
		    app_quit_a.quit();
		    Inhibit(false)
		});
		
		let env_list_wrapper = Box::new(Vertical,10);
		let env_list_frame = Frame::new(Some("Java Environments"));
		let env_list_box = ListBox::new();
		env_list_box.set_size_request(0,200);
		let env_list_box_clone = env_list_box.clone();
		let env_list_box_clone2 = env_list_box.clone();
		let env_list_box_clone3 = env_list_box.clone();
		let env_list_box_clone4 = env_list_box.clone();
		let list : Rc<RefCell<_>> = Rc::new(RefCell::new(Vec::new()));
		let list_clone2 = list.clone();
		let list_clone3 = list.clone();
		let list_clone4 = list.clone();
		
		match os_info::get().os_type() {
			os_info::Type::Arch => {
				println!("Arch Linux detected.")
			}
			os_info::Type::Manjaro => {
				println!("Manjaro detected.")
			}
			_ => {
				util::show_message(&app_quit_a2, Err(app_errors.get_err(0)), Some(("https://github.com/Booglejr/aespresso/issues", Some("Report an issue?"))));
			}
		}
		
		util::refresh_env(&app, app_errors_clone_a, &env_list_box, &list);
		
		let set_env = Button::with_label("Set Default");
		set_env.set_tooltip_text(Some("Sets the selected Java environment as the default environment."));
		
        	set_env.connect_clicked(move |_| {	
			match env_list_box_clone.get_selected_row() {
				None => {}
				Some(selected_row) => {
					let label_text = selected_row.get_child().unwrap().downcast::<Label>().unwrap().get_label().replace(" (default)", "");
					let set_c = Command::new("lxqt-sudo")
						.arg("-s")
				    .arg("archlinux-java")
						.arg("set")
						.arg(label_text)
				    .output();
					match set_c {
						Ok(_text) => {}
						Err(_text) => {util::show_message(&app_quit_b, Err(app_errors_clone_b.get_err(1)), None);}
					}
					util::refresh_env(&app_quit_b2, app_errors_clone_b.clone(), &env_list_box_clone, &list);
				}
			}
        });
        let refresh = Button::with_label("Refresh List");
		refresh.set_tooltip_text(Some("Refreshes the Java environment list."));
		refresh.connect_clicked(move |_| {
			//util::show_message(&app_quit4, Ok("Could not run lxqt-sudo. Please ensure that it is installed on your system."));
			util::refresh_env(&app_quit_a3, app_errors_clone_b2.clone(), &env_list_box_clone2, &list_clone2);
		});
		let unset = Button::with_label("Unset Default Env");
		unset.set_tooltip_text(Some("Unsets the default Java environment."));
		unset.connect_clicked(move |_| {
			let unset_c = Command::new("lxqt-sudo")
				.arg("-s")
				.arg("archlinux-java")
				.arg("unset")
				.output();
			match unset_c {
				Ok(_text) => {}
				Err(_text) => {util::show_message(&app_quit_b3, Err(app_errors_clone_a2.get_err(1)), None);}
			}
			util::refresh_env(&app_quit_b3, app_errors_clone_a4.clone(), &env_list_box_clone3, &list_clone3);
        });
		let fix = Button::with_label("Fix (Auto-select Env)");
		fix.set_tooltip_text(Some("Tries to fix invalid Java environment links. If no default Java environment is set, it will auto-select an environment for you."));		
		fix.connect_clicked(move |_| {
			let fix_c = Command::new("lxqt-sudo")
				.arg("-s")
	            .arg("archlinux-java")
				.arg("fix")
	            .output();
			match fix_c {
				Ok(_text) => {}
				Err(_text) => {util::show_message(&app_quit_b4, Err(app_errors_clone_a3.get_err(1)), None);}
			}
			util::refresh_env(&app_quit_b4, app_errors_clone_b3.clone(), &env_list_box_clone4, &list_clone4);
        });

		let about = Button::with_label("About aespresso");
		about.set_tooltip_text(Some("About this program."));
		about.connect_clicked(move |_| {
			about_window.show_all();
		});

		let close = Button::with_label("Close");
		close.set_tooltip_text(Some("Close this program."));
		close.connect_clicked(move |_| {
			&app_quit_a4.quit();
		});

		let opt_list_frame = Frame::new(Some("Options"));
		let opt_list_box = Box::new(Vertical,10);

		//env_list_box.set_placeholder(Some(&no_env));
		env_list_frame.add(&env_list_box);
		env_list_wrapper.add(&env_list_frame);
		env_list_wrapper.add(&set_env);
		env_list_wrapper.set_border_width(10);
		root.add(&env_list_wrapper);
		
		opt_list_box.add(&refresh);
		opt_list_box.add(&unset);
		opt_list_box.add(&fix);
		opt_list_box.add(&about);
		opt_list_box.add(&close);
		opt_list_box.set_border_width(10);
		opt_list_frame.add(&opt_list_box);
		opt_list_frame.set_border_width(10);
		root.add(&opt_list_frame);
		root.set_border_width(10);
		
		window.add(&root);
        window.show_all();
	});
    application.run(&[]);
}
