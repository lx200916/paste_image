mod escape;
use std::cmp::max;
use std::fs::{read, read_to_string};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use usvg::Options;
use crate::escape::Escape;

// use usvg::Options;

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
    let mut line_vec= Vec::new();


    for line in content.lines() {
        y += 15;
        let mut line_str:Vec<String> = Vec::new();
        line_str.push(format!("<text x=\"10\" y=\"{}\" font-size=\"16px\">", y));
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        let mut line_width = 0;
        for range in ranges.iter() {
            if bg.is_none() {
                bg = Some(range.0.background)
            }
            // println!("{:?}", range);
            let foreground = range.0.foreground;
            let content = range.1;
            line_width += content.chars().fold(10,|all,y| {if y.is_ascii(){all+7} else { all+15 }});
            line_str.push(format!("<tspan fill=\"rgb({},{},{})\">{} </tspan>", foreground.r, foreground.g, foreground.b, Escape(content)) );
        }
        width = max(line_width, width);
        line_str.push(String::from("</text>"));
        line_vec.push(line_str.join(""));
    }
     // format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",width+15,y+10);
    let mut svg_content =String::from(format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",width+15,y+10));

    if let Some(bg) = bg{
        svg_content+= format!("<rect width=\"100%\" height=\"100%\" fill=\"rgb({},{},{})\"/>", bg.r, bg.g, bg.b).as_str();
    }
    svg_content+=line_vec.join("\n").as_str();
    svg_content+="</svg>";
    // let svg_data = read("/Users/xiandeyu/CLionProjects/paste_image/1.svg").unwrap();
    let mut option = Options::default();

    // option.fontdb.load_font_data();
    let rtree = usvg::Tree::from_data(&svg_content.as_bytes(), &option.to_ref()).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(width+15, y+10).unwrap();
    resvg::render(&rtree, usvg::FitTo::Original, tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
    pixmap.save_png("1.png").unwrap();

}

