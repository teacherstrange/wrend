use crate::{
    graphics::{
        attribute_id::AttributeId,
        buffer_id::BufferId,
        create_attribute::{
            create_particle_color_attribute, create_particle_position_attribute,
            create_quad_vertex_attribute,
        },
        create_buffer::{
            create_quad_vertex_buffer, make_create_particle_buffer_a,
            make_create_particle_buffer_b, make_create_particle_color_buffer,
        },
        create_framebuffer::create_perlin_noise_framebuffer,
        create_texture::{create_perlin_noise_texture, create_white_noise_texture},
        fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId,
        program_id::ProgramId,
        render::render,
        texture_id::TextureId,
        transform_feedback_id::TransformFeedbackId,
        uniform_id::UniformId,
        vao_id::VAOId,
        vertex_shader_id::VertexShaderId,
    },
    state::{render_state::RenderState, render_state_handle::RenderStateHandle},
};
use js_sys::Math;
use std::rc::Rc;
use ui::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext, WebGlContextAttributes};
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLinkBuilder, Renderer, TextureLink,
    TransformFeedbackLink, UniformContext, UniformLink, WebGlContextError,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, Callback};
use yew_router::prelude::*;

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/quad.vert");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.frag");
const PERLIN_NOISE_FRAGMENT_SHADER: &str = include_str!("../shaders/perlin_noise.frag");
const UPDATE_PARTICLES_FRAGMENT_SHADER: &str = include_str!("../shaders/update_particles.frag");
const UPDATE_PARTICLES_VERTEX_SHADER: &str = include_str!("../shaders/update_particles.vert");
const DRAW_PARTICLES_FRAGMENT_SHADER: &str = include_str!("../shaders/draw_particles.frag");
const DRAW_PARTICLES_VERTEX_SHADER: &str = include_str!("../shaders/draw_particles.vert");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let render_state = use_mut_ref(RenderState::default);
    let renderer_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let renderer_handle = renderer_handle;
            let render_state = Rc::clone(&render_state);
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let mut pass_through_program_link = ProgramLinkBuilder::new();
                pass_through_program_link
                    .set_program_id(ProgramId::PassThrough)
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_fragment_shader_id(FragmentShaderId::PassThrough);
                let pass_through_program_link = pass_through_program_link
                    .build()
                    .expect("Should build PassThrough ProgramLink successfully");

                let mut perlin_noise_program_link = ProgramLinkBuilder::new();
                perlin_noise_program_link
                    .set_program_id(ProgramId::PerlinNoise)
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_fragment_shader_id(FragmentShaderId::PerlinNoise);
                let perlin_noise_program_link = perlin_noise_program_link
                    .build()
                    .expect("Should build PerlinNoise ProgramLink successfully");

                let mut update_particles_program_link = ProgramLinkBuilder::new();
                update_particles_program_link
                    .set_program_id(ProgramId::UpdateParticles)
                    .set_vertex_shader_id(VertexShaderId::UpdateParticles)
                    .set_fragment_shader_id(FragmentShaderId::UpdateParticles)
                    .set_transform_feedback_varyings(["o_position".to_string()]);
                let update_particles_program_link = update_particles_program_link
                    .build()
                    .expect("Should build UpdateParticles ProgramLink successfully");

                let mut draw_particles_program_link = ProgramLinkBuilder::new();
                draw_particles_program_link
                    .set_program_id(ProgramId::DrawParticles)
                    .set_vertex_shader_id(VertexShaderId::DrawParticles)
                    .set_fragment_shader_id(FragmentShaderId::DrawParticles);
                let draw_particles_program_link = draw_particles_program_link
                    .build()
                    .expect("Should build DrawParticles ProgramLink successfully");

                let render_state_handle: RenderStateHandle = render_state.into();

                let vertex_buffer_link =
                    BufferLink::new(BufferId::QuadVertexBuffer, create_quad_vertex_buffer);

                let particle_buffer_a_link = BufferLink::new(
                    BufferId::ParticleBufferA,
                    make_create_particle_buffer_a(render_state_handle.clone()),
                );

                let particle_buffer_b_link = BufferLink::new(
                    BufferId::ParticleBufferB,
                    make_create_particle_buffer_b(render_state_handle.clone()),
                );

                let particle_color_buffer_link = BufferLink::new(
                    BufferId::ParticleColorBuffer,
                    make_create_particle_color_buffer(render_state_handle.clone()),
                );

                let a_particle_color_link = AttributeLink::new(
                    VAOId::DrawParticles,
                    BufferId::ParticleColorBuffer,
                    AttributeId::AParticleColor,
                    create_particle_color_attribute,
                );

                let a_particle_position_link_a = AttributeLink::new(
                    (VAOId::DrawParticles, VAOId::UpdateParticlesA),
                    BufferId::ParticleBufferA,
                    AttributeId::AParticlePosition,
                    create_particle_position_attribute,
                );

                let a_particle_position_link_b = AttributeLink::new(
                    (VAOId::DrawParticles, VAOId::UpdateParticlesB),
                    BufferId::ParticleBufferB,
                    AttributeId::AParticlePosition,
                    create_particle_position_attribute,
                );

                let a_quad_vertex_link = AttributeLink::new(
                    (VAOId::PassThrough, VAOId::PerlinNoise),
                    BufferId::QuadVertexBuffer,
                    AttributeId::AQuadVertex,
                    create_quad_vertex_attribute,
                );

                let white_noise_texture_link =
                    TextureLink::new(TextureId::WhiteNoise, create_white_noise_texture);

                let u_white_noise_texture = UniformLink::new(
                    ProgramId::PerlinNoise,
                    UniformId::UWhiteNoiseTexture,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    },
                );

                let perlin_noise_texture_link =
                    TextureLink::new(TextureId::PerlinNoise, create_perlin_noise_texture);

                let u_perlin_noise_texture = UniformLink::new(
                    (ProgramId::PassThrough, ProgramId::UpdateParticles),
                    UniformId::UPerlinNoiseTexture,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 1);
                    },
                );

                let perlin_noise_framebuffer_link = FramebufferLink::new(
                    FramebufferId::PerlinNoise,
                    create_perlin_noise_framebuffer,
                    Some(TextureId::PerlinNoise),
                );

                let u_now_link = UniformLink::new(
                    ProgramId::PerlinNoise,
                    UniformId::UNow,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        const TIME_CONSTANT: f64 = 50_000.0;
                        const MAX_OFFSET: f32 = f32::MAX / 10.0;
                        let time_offset = Math::random() as f32 * MAX_OFFSET;
                        let final_time = (ctx.now() / TIME_CONSTANT) as f32 + time_offset;
                        gl.uniform1f(Some(uniform_location), final_time);
                    },
                );

                let transform_feedback_link =
                    TransformFeedbackLink::new(TransformFeedbackId::Particle);

                // provide custom attributes when getting WebGL context
                let get_context_callback = |canvas: &HtmlCanvasElement| {
                    let mut webgl_context_attributes = WebGlContextAttributes::new();
                    webgl_context_attributes.preserve_drawing_buffer(true);

                    let gl = canvas
                        .get_context_with_context_options("webgl2", &webgl_context_attributes)
                        .map_err(|_| WebGlContextError::RetrievalError)?;

                    let gl = gl.ok_or(WebGlContextError::NotFoundError)?;

                    let gl: WebGl2RenderingContext = gl
                        .dyn_into()
                        .map_err(|_| WebGlContextError::TypeConversionError)?;

                    Ok(gl)
                };

                let mut renderer_builder = Renderer::builder();
                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state_handle)
                    .set_render_callback(render)
                    .add_vertex_shader_src(VertexShaderId::Quad, QUAD_VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::PerlinNoise,
                        PERLIN_NOISE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::UpdateParticles,
                        UPDATE_PARTICLES_FRAGMENT_SHADER.to_string(),
                    )
                    .add_vertex_shader_src(
                        VertexShaderId::UpdateParticles,
                        UPDATE_PARTICLES_VERTEX_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::DrawParticles,
                        DRAW_PARTICLES_FRAGMENT_SHADER.to_string(),
                    )
                    .add_vertex_shader_src(
                        VertexShaderId::DrawParticles,
                        DRAW_PARTICLES_VERTEX_SHADER.to_string(),
                    )
                    .add_program_link(perlin_noise_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_program_link(update_particles_program_link)
                    .add_program_link(draw_particles_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_buffer_link(particle_buffer_a_link)
                    .add_buffer_link(particle_buffer_b_link)
                    .add_buffer_link(particle_color_buffer_link)
                    .add_attribute_link(a_quad_vertex_link)
                    .add_attribute_link(a_particle_position_link_a)
                    .add_attribute_link(a_particle_position_link_b)
                    .add_attribute_link(a_particle_color_link)
                    .add_texture_link(perlin_noise_texture_link)
                    .add_texture_link(white_noise_texture_link)
                    .add_framebuffer_link(perlin_noise_framebuffer_link)
                    .add_uniform_link(u_perlin_noise_texture)
                    .add_uniform_link(u_white_noise_texture)
                    .add_uniform_link(u_now_link)
                    .add_transform_feedback_link(transform_feedback_link)
                    .add_vao_link(VAOId::PerlinNoise)
                    .add_vao_link(VAOId::PassThrough)
                    .add_vao_link(VAOId::UpdateParticlesA)
                    .add_vao_link(VAOId::UpdateParticlesB)
                    .add_vao_link(VAOId::DrawParticles)
                    .set_get_context_callback(get_context_callback);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                let mut new_renderer_handle = renderer.into_renderer_handle();
                new_renderer_handle.set_animation_callback(Some(
                    |renderer: &Renderer<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer.update_uniforms();
                        renderer.render();
                    },
                ));

                new_renderer_handle.start_animating();

                // save handle to keep animation going
                *renderer_handle.borrow_mut() = Some(new_renderer_handle);

                || {}
            }
        },
        (),
    );

    let handle_click = {
        let render_state = Rc::clone(&render_state);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            render_state.borrow_mut().set_should_save_image(true);
        })
    };

    html! {
        <div class="flow-field-colorful">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <button onclick={handle_click}>{"Save Image"}</button>
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
