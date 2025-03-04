use crate::{FramebufferCreateCallback, FramebufferCreateContext, Id, IdDefault};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

#[derive(Clone)]
pub struct FramebufferLink<FramebufferId: Id, TextureId: Id = IdDefault> {
    framebuffer_id: FramebufferId,
    texture_id: Option<TextureId>,
    framebuffer_create_callback: FramebufferCreateCallback,
}

impl<FramebufferId: Id, TextureId: Id> FramebufferLink<FramebufferId, TextureId> {
    pub fn new(
        framebuffer_id: FramebufferId,
        framebuffer_create_callback: impl Into<FramebufferCreateCallback>,
        texture_id: Option<TextureId>,
    ) -> Self {
        Self {
            framebuffer_id,
            framebuffer_create_callback: framebuffer_create_callback.into(),
            texture_id,
        }
    }

    pub fn framebuffer_id(&self) -> &FramebufferId {
        &self.framebuffer_id
    }

    pub fn texture_id(&self) -> Option<TextureId> {
        self.texture_id.clone()
    }

    pub fn create_framebuffer(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        texture: Option<WebGlTexture>,
    ) -> WebGlFramebuffer {
        let framebuffer_create_context = FramebufferCreateContext::new(gl, now, texture);
        self.framebuffer_create_callback
            .call_with_into_js_arg_and_return(&framebuffer_create_context)
    }
}

impl<FramebufferId: Id, TextureId: Id> Debug for FramebufferLink<FramebufferId, TextureId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FramebufferLink")
            .field("framebuffer_id", &self.framebuffer_id)
            .finish()
    }
}

impl<FramebufferId: Id, TextureId: Id> Hash for FramebufferLink<FramebufferId, TextureId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.framebuffer_id.hash(state);
    }
}

impl<FramebufferId: Id, TextureId: Id> PartialEq for FramebufferLink<FramebufferId, TextureId> {
    fn eq(&self, other: &Self) -> bool {
        self.framebuffer_id == other.framebuffer_id
    }
}

impl<FramebufferId: Id, TextureId: Id> Eq for FramebufferLink<FramebufferId, TextureId> {}
