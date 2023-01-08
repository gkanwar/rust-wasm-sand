import init, { WasmGameContext } from 'rust-wasm-sand';

const RESCALE = 4;

(async () => {
    await init();

    const canvas = document.getElementById("game-canvas");
    const [width, height] = [canvas.width, canvas.height];
    if (width % RESCALE !== 0 || height % RESCALE !== 0) {
        console.error("Game canvas width/height incorrectly set");
    }
    const [gameWidth, gameHeight] = [width/RESCALE, height/RESCALE];

    const gameContext = WasmGameContext.new(gameWidth, gameHeight);
    gameContext.bind_canvas(canvas);
    setMouseHooks(canvas, [gameWidth, gameHeight], gameContext);

    const render = (timestamp) => {
        gameContext.render();
        gameContext.update(timestamp);
        window.requestAnimationFrame(render);
    }

    window.requestAnimationFrame(render);
})()

function setMouseHooks(canvas, dims, gameContext) {
    const [_, gameHeight] = dims;
    canvas.addEventListener("mousedown", (event) => {
        console.log("mouse down");
        gameContext.mouse_down(
            event.offsetX / RESCALE,
            gameHeight - event.offsetY / RESCALE);
    });
    canvas.addEventListener("mouseup", (event) => {
        console.log("mouse up");
        gameContext.mouse_up(
            event.offsetX / RESCALE,
            gameHeight - event.offsetY / RESCALE);
    });
    canvas.addEventListener("mousemove", (event) => {
        console.log("mouse move");
        gameContext.mouse_move(
            event.offsetX / RESCALE,
            gameHeight - event.offsetY / RESCALE);
    });
}