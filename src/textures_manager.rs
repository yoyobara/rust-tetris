use sdl2::{
    render:: {
        Texture, TextureCreator
    },
    
    video::WindowContext,
    image::LoadTexture,
};

pub struct TexturesManager {
    pub background: Texture,
    pub pieces: Texture,
}

impl TexturesManager {
    pub fn new(creator: &TextureCreator<WindowContext>) -> Result<Self, String> {
        Ok(TexturesManager {
            background: creator.load_texture("./assests/img/tetris_bg.png")?,
            pieces: creator.load_texture("./assests/img/pieces.png")?
        })
    }
}

