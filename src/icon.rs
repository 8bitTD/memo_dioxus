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