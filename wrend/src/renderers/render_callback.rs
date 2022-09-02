use crate::RendererData;
use crate::{CallbackWithContext, Either, Id, IdDefault, IdName, RenderCallbackJs};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct RenderCallback<
    VertexShaderId: Id = IdDefault,
    FragmentShaderId: Id = IdDefault,
    ProgramId: Id = IdDefault,
    UniformId: Id + IdName = IdDefault,
    BufferId: Id = IdDefault,
    AttributeId: Id + IdName = IdDefault,
    TextureId: Id = IdDefault,
    FramebufferId: Id = IdDefault,
    TransformFeedbackId: Id = IdDefault,
    VertexArrayObjectId: Id = IdDefault,
    UserCtx: Clone + 'static = (),
>(
    Either<
        CallbackWithContext<
            dyn Fn(
                &RendererData<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    AttributeId,
                    TextureId,
                    FramebufferId,
                    TransformFeedbackId,
                    VertexArrayObjectId,
                    UserCtx,
                >,
            ),
        >,
        // It's not crucial that the JavaScript function have access to the RendererData as
        // as an argument to the function itself (like the Rust callback),
        // since in JavaScript, it's very easy for the renderer_data function to hold the RendererData
        // in the `render` callback closure
        CallbackWithContext<RenderCallbackJs>,
    >,
);

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    > PartialEq
    for RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    > Deref
    for RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    type Target = Either<
        CallbackWithContext<
            dyn Fn(
                &RendererData<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    AttributeId,
                    TextureId,
                    FramebufferId,
                    TransformFeedbackId,
                    VertexArrayObjectId,
                    UserCtx,
                >,
            ),
        >,
        CallbackWithContext<RenderCallbackJs>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
        F: Fn(
                &RendererData<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    AttributeId,
                    TextureId,
                    FramebufferId,
                    TransformFeedbackId,
                    VertexArrayObjectId,
                    UserCtx,
                >,
            ) + 'static,
    > From<F>
    for RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(
                    &RendererData<
                        VertexShaderId,
                        FragmentShaderId,
                        ProgramId,
                        UniformId,
                        BufferId,
                        AttributeId,
                        TextureId,
                        FramebufferId,
                        TransformFeedbackId,
                        VertexArrayObjectId,
                        UserCtx,
                    >,
                ),
            >)))
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    > From<RenderCallbackJs>
    for RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn from(callback: RenderCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
        F: Fn(
                &RendererData<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    AttributeId,
                    TextureId,
                    FramebufferId,
                    TransformFeedbackId,
                    VertexArrayObjectId,
                    UserCtx,
                >,
            ) + 'static,
    > From<Rc<F>>
    for RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback
                as Rc<
                    dyn Fn(
                        &RendererData<
                            VertexShaderId,
                            FragmentShaderId,
                            ProgramId,
                            UniformId,
                            BufferId,
                            AttributeId,
                            TextureId,
                            FramebufferId,
                            TransformFeedbackId,
                            VertexArrayObjectId,
                            UserCtx,
                        >,
                    ),
                >,
        )))
    }
}