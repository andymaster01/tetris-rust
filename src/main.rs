extern crate sfml;

use sfml::window::{VideoMode, Event, window_style, ContextSettings, Key};
use sfml::graphics::{RenderWindow, Color, RenderTarget, Texture, Sprite, IntRect, Transformable};

fn main() {

    let M:i32 = 20;
    let N:i32 = 10;

    let field: [[i32; 4]; 4]; 

    let figures: [[i32; 4]; 7] = [ 
        [1,3,5,7], //I
        [2,4,5,7], //Z
        [3,5,4,6], //S
        [3,5,4,7], //T
        [2,3,5,7], //L
        [3,5,7,6], //J
        [2,3,4,5]  //O
    ]; 

    
    #[derive(Copy, Clone)]
    struct Point {x: i32, y: i32};

    let mut a: [Point; 4] = [Point{x:0, y:0}; 4];
    let mut b: [Point; 4] = [Point{x:0, y:0}; 4];

    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
						"The Game!",
						window_style::CLOSE,
						&ContextSettings::default()
			).unwrap();

    

    loop {

        let t = Texture::from_file("tiles.png").unwrap();
        let mut s = Sprite::with_texture(&t);

        let rect = IntRect{top:0, left:0, height:18, width:18};

        s.set_texture_rect(&rect);

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

        let n = 6;
        
        for i in 0..4 {
            a[i].x = figures[n][i] % 2;
            a[i].y = figures[n][i] / 2;
        }

        window.clear(&Color::white());

        for i in 0..4 {
            s.set_position2f((a[i].x*18) as f32, (a[i].y*18) as f32);
            window.draw(&s);
        }

 	    window.display();
    }

    

}
