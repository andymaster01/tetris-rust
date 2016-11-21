extern crate sfml;

use sfml::window::{VideoMode, Event, window_style, ContextSettings, Key};
use sfml::graphics::{RenderWindow, Color, RenderTarget};

fn main() {

    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
						"The Game!",
						window_style::CLOSE,
						&ContextSettings::default()
			).unwrap();

    loop {

	    for event in window.events(){
		    match event {
    			Event::KeyPressed { code, .. } => {
	    			match code {
		    			Key::Escape => return,
			    		_ => {}
			    	}
			    },
		        _ => {}
		    }
        }

        window.clear(&Color::white());
 	    window.display();
    }

    

}
