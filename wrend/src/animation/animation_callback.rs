use std::{ops::Deref, rc::Rc};

use crate::{CallbackWithContext, Either, Id, IdDefault, IdName, Renderer, AnimationCallbackJs};

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct AnimationCallback<
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
                &Renderer<
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
        CallbackWithContext<AnimationCallbackJs>,
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
    for AnimationCallback<
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
    for AnimationCallback<
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
                &Renderer<
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
        CallbackWithContext<AnimationCallbackJs>,
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
                &Renderer<
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
    for AnimationCallback<
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
                    &Renderer<
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
        F: Fn(
                &Renderer<
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
    for AnimationCallback<
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
                        &Renderer<
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
    > From<AnimationCallbackJs>
    for AnimationCallback<
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
    fn from(callback: AnimationCallbackJs) -> Self {
        Self(Either::new_b(callback.into()))
    }
}
