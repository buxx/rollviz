extern crate doryen_rs;

use std::path::PathBuf;
use structopt::StructOpt;
use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent, Image};
use image::{GenericImage, GenericImageView};

const CONSOLE_WIDTH: u32 = 30;
const CONSOLE_HEIGHT: u32 = 30;

struct RollViz {
    image: Option<Image>,
    tile: Option<u16>,
}

impl Engine for RollViz {
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con().clear(
            Some((0, 0, 0, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16)
        );
        if let (Some(tile)) = self.tile {
            for row_i in 0..5 {
                for col_i in 0..5 {
                    api.con().ascii(
                        col_i,
                        row_i,
                        tile as u16,
                    );
                    api.con().fore(
                        col_i,
                        row_i,
                        (255, 255, 255, 255),
                    );
                }
            }
        }
        if let Some(image) = self.image.as_mut() {
            image.blit_2x(api.con(), 0, 0, 0, 0, None, None, None);
        }
    }
    fn resize(&mut self, api: &mut dyn DoryenApi) {
        let width = api.get_screen_size().0 / 18;
        let height = api.get_screen_size().1 / 18;
        api.con().resize(width, height);
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "vizualize")]
enum Opt {
    image {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    tile {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    }
}


fn main() {
    let opt = Opt::from_args();
    let mut options = AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 18,
        screen_height: CONSOLE_HEIGHT * 18,
        window_title: "RollViz".to_owned(),
        font_path: "Teeto_K_18x18.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
        intercept_close_request: false,
    };

    let gui: Box<dyn Engine> = match opt {
        Opt::image{input} => {
            let image = Some(Image::new(input.to_str().unwrap()));
            Box::new(RollViz{image, tile: None})
        }
        Opt::tile{input} => {
            let mut tiles = image::open("static/Teeto_K_18x18.png").unwrap();
            let tile = image::open(input.to_str().unwrap()).unwrap();
            for row_i in 0..18 {
                for col_i in 0..18 {
                    let pixel = tile.get_pixel(col_i, row_i);
                    tiles.put_pixel(col_i + 18, row_i, pixel);
                }
            }
            tiles.save("static/tmp_18x18.png").unwrap();
            options.font_path = "tmp_18x18.png".to_owned();
            Box::new(RollViz{image: None, tile: Some(1 as u16)})
        }
    };

    let mut app = App::new(options);
    app.set_engine(gui);
    app.run();
}
