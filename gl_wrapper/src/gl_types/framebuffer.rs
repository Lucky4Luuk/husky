use crate::gl_types::Texture;

/// A framebuffer to render to. Currently does not support 3D textures or render buffers.
pub struct Framebuffer {
    pub id: gl::types::GLuint,

    //Attachments
    pub col: Option<Texture>,
    pub depth: Option<Texture>,
}

impl Framebuffer {
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }
        Self {
            id: id,

            col: None,
            depth: None,
        }
    }

    //TODO: Support 3D textures too
    pub fn set_color_attachment(&mut self, texture: Texture) {
        self.bind();
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id, 0);
        }
        self.col = Some(texture); //Take ownership of texture, we don't want to drop it
        self.unbind();
    }

    pub fn set_depth_attachment(&mut self, texture: Texture) {
        self.bind();
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, texture.id, 0);
        }
        self.depth = Some(texture); //Take ownership
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn status(&self) -> gl::types::GLenum {
        self.bind();
        let res = unsafe {
            gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
        };
        self.unbind();
        res
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
