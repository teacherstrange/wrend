import './style.scss'
import vertexShader from './shaders/vertex.glsl?raw';
import fragmentShader from './shaders/fragment.glsl?raw';
import init, { AttributeLink, BufferLink, ProgramLink, UniformLink, Renderer }  from 'wrend';

const main = async () => {
    // it's necessary to initialize wasm module before using
    await init();

    const canvas = document.querySelector('canvas');

    const QUAD = [
        -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0,
    ];

    const PROGRAM_ID = 'program';
    const VAO_ID = 'vao';
    const VERTEX_SHADER_ID = 'vertex_shader';
    const FRAGMENT_SHADER_ID = 'fragment_shader';
    const VERTEX_BUFFER_ID = 'buffer_id';
    const A_POSITION_ID = 'a_position';
    const U_NOW_ID = 'u_now';

    const programLink = new ProgramLink(PROGRAM_ID, VERTEX_SHADER_ID, FRAGMENT_SHADER_ID);

    const vertexBufferLink = new BufferLink(VERTEX_BUFFER_ID, (ctx) => {
        const gl = ctx.gl();
        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(QUAD), gl.STATIC_DRAW);
        return buffer;
    })

    const aPositionLink = new AttributeLink([VAO_ID], VERTEX_BUFFER_ID, A_POSITION_ID, (ctx) => {
        const gl = ctx.gl();
        const attributeLocation = ctx.attributeLocation();
        const webglBuffer = ctx.webglBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, webglBuffer);
        gl.vertexAttribPointer(attributeLocation.get(), 2, gl.FLOAT, false, 0, 0);
    })

    const uNowLink = new UniformLink([PROGRAM_ID], U_NOW_ID, (ctx) => {
        const gl = ctx.gl();
        const uniformLocation = ctx.uniformLocation();
        gl.uniform1f(uniformLocation, ctx.now());
    });
    uNowLink.setUseInitCallbackForUpdate(true);

    const render = (renderer) => {
        const gl = renderer.gl();
        const canvas = renderer.canvas();

        renderer.useProgram(PROGRAM_ID);
        renderer.useVAO(VAO_ID);

        gl.viewport(0, 0, canvas.width, canvas.height);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.drawArrays(gl.TRIANGLES, 0, 6);
    };

    const rendererBuilder = Renderer.builder();
    rendererBuilder.setCanvas(canvas)
    rendererBuilder.setRenderCallback(render)
    rendererBuilder.addProgramLink(programLink)
    rendererBuilder.addVertexShaderSrc(VERTEX_SHADER_ID, vertexShader)
    rendererBuilder.addFragmentShaderSrc(FRAGMENT_SHADER_ID, fragmentShader)
    rendererBuilder.addBufferLink(vertexBufferLink)
    rendererBuilder.addAttributeLink(aPositionLink)
    rendererBuilder.addUniformLink(uNowLink)
    rendererBuilder.addVAOLink(VAO_ID)
    const renderer = rendererBuilder.buildRenderer();

    // test a lone render() from the renderer
    renderer.render();

    // test an animated render() from the renderer
    renderer.setAnimationCallback((rendererData) => {
        rendererData.updateUniforms();
        rendererData.render();
    });

    renderer.startAnimating();

    // will force the animation stop and clean up all wasm memory
    // (this something you would want to call when you're ready to stop
    // using the renderer you created)
    setTimeout(() => renderer.free(), 5000)
}

main();