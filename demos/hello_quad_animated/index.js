import './style.scss';

/// import wasm module
import('./pkg').catch(console.error).then((module) => module.hello_quad_animated());

