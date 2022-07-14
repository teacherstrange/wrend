use super::{
    framebuffer_create_context::FramebufferCreateContext, id::Id, id_name::IdName,
    renderer::Renderer,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub type CreateFramebufferCallback<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
    BufferId,
    TextureId,
    FramebufferId,
    UserCtx,
> = Rc<
    dyn Fn(
        FramebufferCreateContext<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> WebGlTexture,
>;

#[derive(Clone)]
pub struct FramebufferLink<
    VertexShaderId: Id,
    FragmentShaderId: Id,
    ProgramId: Id,
    UniformId: Id + IdName,
    BufferId: Id + IdName,
    TextureId: Id,
    FramebufferId: Id,
    UserCtx: 'static,
> {
    program_id: ProgramId,
    framebuffer_id: FramebufferId,
    create_framebuffer_callback: CreateFramebufferCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >,
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    >
    FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
where
    ProgramId: Id,
    FramebufferId: Id,
{
    pub fn new(
        program_id: ProgramId,
        framebuffer_id: FramebufferId,
        create_framebuffer_callback: CreateFramebufferCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> Self {
        Self {
            program_id,
            framebuffer_id,
            create_framebuffer_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn framebuffer_id(&self) -> &FramebufferId {
        &self.framebuffer_id
    }

    pub fn create_framebuffer(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        renderer: &Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
        user_ctx: Option<&UserCtx>,
    ) -> WebGlTexture {
        let framebuffer_create_context = FramebufferCreateContext::new(gl, now, renderer, user_ctx);
        (self.create_framebuffer_callback)(framebuffer_create_context)
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Debug
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FramebufferLink")
            .field("program_id", &self.program_id)
            .field("framebuffer_id", &self.framebuffer_id)
            .finish()
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Hash
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.framebuffer_id.hash(state);
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > PartialEq
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id && self.framebuffer_id == other.framebuffer_id
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Eq
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
}