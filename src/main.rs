extern crate sfml;

use sfml::window::{VideoMode, Event, window_style, ContextSettings, Key};
use sfml::graphics::{RenderWindow, Color, RenderTarget, Texture, Sprite, IntRect};

fn main() {

    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
						"The Game!",
						window_style::CLOSE,
						&ContextSettings::default()
			).unwrap();

    

    loop {

        let tiles = Texture::from_file("tiles.png").unwrap();
        let mut image = Sprite::with_texture(&tiles);

        let rect = IntRect{top:0, left:0, height:18, width:18};

        image.set_texture_rect(&rect);

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
        window.draw(&image);
 	    window.display();
    }

    

}
