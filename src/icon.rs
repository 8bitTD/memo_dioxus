/*
pub fn load_icon_from_url(url: &str) -> Option<tao::window::Icon>{
    let Ok(response) = reqwest::blocking::get(url) else {return None};
    let bytes = response.bytes().unwrap();
    let Ok(img) = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format() else {return None};
    let Ok(dyim) = img.decode() else {return None};
    let pixels = dyim.as_bytes().to_vec();
    let width = dyim.width();
    let height = dyim.height();
    let Ok(ico) = tao::window::Icon::from_rgba(pixels, width, height) else {return None};
    return Some(ico); 
}
*/

pub fn load_icon() -> Option<tao::window::Icon> {
    let Ok(response) = image::ImageReader::open("./icon.png") else {return None};
    let Ok(icon_dec) = response.decode() else {return None};
    let width = icon_dec.width();
    let height = icon_dec.height();
    let bytes = icon_dec.as_bytes();
    let pixels = bytes.to_vec();
    let Ok(ico) = tao::window::Icon::from_rgba(pixels, width, height) else {return None};
    return Some(ico);
}