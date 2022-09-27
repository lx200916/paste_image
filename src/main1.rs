mod escape;

use std::borrow::BorrowMut;
use std::cmp::max;
use std::fs::{read, read_to_string};
use image::{DynamicImage, ImageFormat, Rgb, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;


use rusttype::{Font, Scale};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use crate::escape::Escape;
use once_cell::sync::OnceCell;
use syntect::html::IncludeBackground::No;

const FONT_SIZE: usize = 26;
const FONT_WIDTH_A: usize = 8;
const FONT_WIDTH_U:usize = 10;
struct FontStore{
    store: OnceCell<Vec<Font<'static>>>
}
// x,y,font,color,text
struct  Drawable<'a>(i32,i32,Rgba<u8>,&'a Font<'static>,&'a str);

struct Drawables<'a>{
    height: usize,
    max_width: usize,
    drawables: Vec<Drawable<'a>>
}
impl FontStore {
    fn get_font(&self,is_ascii: bool) -> & Font<'static> {
        let store = self.store.get_or_init(||{
            let mut font = Vec::new();
            font.push(Font::try_from_bytes(include_bytes!("../assets/Hack-Regular.ttf")).unwrap());
            font.push(Font::try_from_bytes(include_bytes!("../assets/MicrosoftYaHeiUILight.ttf")).unwrap());
            font
        });
        if is_ascii { store.get(0).unwrap()} else { store.get(1).unwrap()}
    }
}

impl Default for FontStore {
    fn default() -> Self {
        let store = FontStore{store: OnceCell::new()};
        store.store.get_or_init(||{
            let mut font = Vec::new();
            font.push(Font::try_from_bytes(include_bytes!("../assets/Hack-Regular.ttf")).unwrap());
            font.push(Font::try_from_bytes(include_bytes!("../assets/MicrosoftYaHeiUILight.ttf")).unwrap());
            font
        });
        store
    }
}

// impl From<Color> for Rgba<u8>{
//     fn from(color: Color) -> Self {
//         Rgba([color.r,color.g,color.b,color.a])
//     }
// }

fn draw_text<'a> (text: &'a str, color: Color, x: &mut i32, y:i32, font:&'a FontStore) -> Vec<Drawable<'a>> {
    *x+=2;
    let mut draw = Vec::new();
    // ASCII characters can be draw with Hack Font without worrying about Full-Width characters....
    if text.is_ascii(){
        // draw_text_mut(canvas,Rgba([color.r,color.g,color.b,color.a]),*x,y,scale,font.get_font(true),text);
        draw.push(Drawable(*x,y,Rgba([color.r,color.g,color.b,color.a]),font.get_font(true),text));
        *x+=(FONT_WIDTH_A*text.len()) as i32;
        println!("ANSI {},",text);
    }else{
        // CJK or Unicode characters. Emoji characters are not allowed.
        for char in text.chars() {
            if char.is_ascii() {
                draw.push(Drawable(*x,y,Rgba([color.r,color.g,color.b,color.a]),font.get_font(true),text));

                // draw_text_mut(canvas,Rgba([color.r,color.g,color.b,color.a]),*x,y,scale,font.get_font(true),text);
                *x+=(FONT_WIDTH_A) as i32;
            }else {
                // draw.push(Drawable(*x,y,Rgba([color.r,color.g,color.b,color.a]),font.get_font(false),text));

                // draw_text_mut(canvas,Rgba([color.r,color.g,color.b,color.a]),*x,y,scale,font.get_font(false),text);
                *x+=FONT_WIDTH_U as i32;
            }
        }

    }
    draw



}
fn main() {
    println!("Hello, world!");
    let ss= SyntaxSet::load_defaults_newlines();

    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let content = read_to_string("/Users/xiandeyu/CLionProjects/paste_image/src/main.rs").unwrap();
    let mut height = 0;
    let mut width = 0;
    let mut y = 10;
    let mut bg:Option<Color> = None;
    let fontStore = FontStore::default();
    let mut image:Option<DynamicImage>  = None;
    let mut line_vec:Vec<Drawable> = Vec::new();

    for line in content.lines() {
        let mut x=10;
        y += FONT_SIZE as i32;
        // line_str.push(format!("<text x=\"10\" y=\"{}\" font-size=\"16px\">", y));

        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        let mut line_width = 0;
        for range in ranges.iter() {
            if bg.is_none() {
                bg = Some(range.0.background)
            }
            // println!("{:?}", range);
            let foreground = range.0.foreground;
            let content = range.1;
            line_vec.append(draw_text(content,foreground,x.borrow_mut(),y,&fontStore).as_mut());

            line_width += content.chars().fold(10,|all,y| {if y.is_ascii(){all+7} else { all+15 }});
           println!("X:{:?}",x);
            // line_str.push(format!("<tspan fill=\"rgb({},{},{})\">{} </tspan>", foreground.r, foreground.g, foreground.b, Escape(content)) );
        }
        width = max(x, width);
        // line_str.push(String::from("</text>"));
        // println!("{}", line_str.join(""));
        println!("{:?},{:?}",x,y)


    }
    // println!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",width+15,y+10);
    if let Some(bg) = bg{
        image=Some(DynamicImage::ImageRgba8(RgbaImage::from_pixel((width+10) as u32,(y+5) as u32,Rgba([bg.r,bg.g,bg.b,bg.a]))));
        // println!("<rect width=\"100%\" height=\"100%\" fill=\"rgb({},{},{})\"/>", bg.r, bg.g, bg.b);
    }
    let mut image=image.unwrap();
    let scale = Scale { x: (FONT_SIZE as f32)*0.5, y: FONT_SIZE  as f32, };
    for i in  line_vec{
        draw_text_mut(image.borrow_mut(),i.2,i.0,i.1,scale,i.3,i.4);
    }
    image.save_with_format("./2.png", ImageFormat::Png).expect("TODO: panic message");
    // let svg_data = read("/Users/xiandeyu/CLionProjects/paste_image/1.svg").unwrap();
    // let mut option = Options::default();
    // option.fontdb.load_system_fonts();
    // let rtree = usvg::Tree::from_data(&svg_data, &option.to_ref()).unwrap();
    // let mut pixmap = tiny_skia::Pixmap::new(width+15, y+10).unwrap();
    // resvg::render(&rtree, usvg::FitTo::Original, tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
    // pixmap.save_png("1.png").unwrap();

}

