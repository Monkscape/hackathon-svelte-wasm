<script>
    import { onMount } from 'svelte';
    import { default as wasm, Universe, Cell } from 'wasm-alert';

    const CELL_SIZE = 5;
    const GRID_COLOR = '#CCCCCC';
    const DEAD_COLOR = '#FFFFFF';
    const ALIVE_COLOR = '#000000';

    let framesPerSecond;

    let universe;
    let width = 64;
    let height = 64;
    let randomnessFactor = 50;

    let canvas;
    let memory;
    let canvasHeight = 0;
    let canvasWidth = 0;

    $: canvasHeight = (CELL_SIZE + 1) * height + 1;
    $: canvasWidth = (CELL_SIZE + 1) * width + 1;

    let animationId = null;
    let frames = []
    let lastFrameTimeStamp = performance.now();

    onMount(async () => {
        let wasmCompilation = await wasm();
        memory = wasmCompilation.memory;
    })

    const renderFPS = () => {
        const now = performance.now();
        const delta = now - lastFrameTimeStamp;
        lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        frames.push(fps);
        if (frames.length > 100) {
            frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < frames.length; i++) {
            sum += frames[i];
            min = Math.min(frames[i], min);
            max = Math.max(frames[i], max);
        }
        let mean = sum / frames.length;

        // Render the statistics.
        framesPerSecond.textContent = `Frames per Second:
                latest = ${Math.round(fps)}
                avg of last 100 = ${Math.round(mean)}
                min of last 100 = ${Math.round(min)}
                max of last 100 = ${Math.round(max)}`.trim();
    }

    const resetUniverse = () => {
        universe = null;
        createUniverse();
    }

    const createUniverse = () => {
        if (!universe) {
            universe = Universe.new(width, height, 1 - (randomnessFactor / 100));
        }
    }

    const play = () => {
        createUniverse();
        renderLoop();
    };

    const pause = () => {
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    const isPaused = () => {
        return animationId === null;
    };

    const pauseOrPlay = () => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    };

    const renderLoop = () => {
        renderFPS();
        universe.tick();

        drawGrid();
        drawCells();

        animationId = requestAnimationFrame(renderLoop);
    }

    const drawGrid = () => {
        let ctx = canvas.getContext('2d');
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
        }

        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
        }

	    ctx.stroke();
    };

    const getIndex = (row, column) => {
	return row * width + column;
};

    const drawCells = () => {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

        let ctx = canvas.getContext('2d');
        ctx.beginPath();
        
        ctx.fillStyle = ALIVE_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== Cell.Alive) {
                    continue;
                }

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.fillStyle = DEAD_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== Cell.Dead) {
                    continue;
                }

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.stroke();
    };

    const addCell = (event) => {
        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;

        const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

        console.log('Toggle Cell', row, col);
        universe.toggle_cell(row, col);
        drawGrid();
        drawCells();
    }

</script>

<form>
    <label>
        Width:
        <input type='number' bind:value={width}>
    </label>
    <label>
        Height:
        <input type='number' bind:value={height}>
    </label>
    <label>
        Generation Percent {randomnessFactor}%
        <input type='range' min={1} max={100} bind:value={randomnessFactor}>
    </label>
    <button on:click={pauseOrPlay}>{!animationId ? 'Play' : 'Pause'}</button>
    <button on:click={resetUniverse}>Reset</button>
</form>
<div class="fps" bind:this={framesPerSecond}></div>
<canvas 
    bind:this={canvas} 
    height={canvasHeight} 
    width={canvasWidth}
    on:click={addCell}
>
</canvas>

<style>
    .fps {
        white-space: pre;
        font-family: monospace;
    }
</style>