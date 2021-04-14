<script>
    import { createEventDispatcher, onMount } from "svelte";
    import { default as wasm, Universe, Cell } from "wasm-alert";

    const dispatch = createEventDispatcher();

    let init = false;

    export let width = 64;
    export let height = 64;
    export let randomnessFactor = 50;
    export let CELL_SIZE = 5;

    const GRID_COLOR = '#CCCCCC';
    const DEAD_COLOR = '#FFFFFF';
    const ALIVE_COLOR = '#000000';

    let universe;

    let canvas;
    let memory;
    let canvasHeight;
    let canvasWidth;
    let animationId = null;

    $: refreshUniverse(width, height, randomnessFactor);
    $: canvasHeight = (CELL_SIZE + 1) * height + 1;
    $: canvasWidth = (CELL_SIZE + 1) * width + 1;

    onMount(async () => {
        let wasmCompilation = await wasm();
        memory = wasmCompilation.memory;
        init = true;
    })

    const resetUniverse = () => {
        universe = null;
        createUniverse(width, height, randomnessFactor);
    }

    const refreshUniverse = (width, height, randomnessFactor) => {
        if (init) {
            universe = Universe.new(width, height, 1 - (randomnessFactor / 100));
        }
    }

    const createUniverse = (width, height, randomnessFactor) => {
        if (!universe) {
            universe = Universe.new(width, height, 1 - (randomnessFactor / 100));
        }
    }

    const play = () => {
        createUniverse(width, height, randomnessFactor);
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
        dispatch('tick', performance.now());
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

<div>
    <button on:click={pauseOrPlay}>{!animationId ? 'Play' : 'Pause'}</button>
    <button on:click={resetUniverse}>Reset</button>
</div>
<canvas 
    bind:this={canvas} 
    height={canvasHeight} 
    width={canvasWidth}
    on:click={addCell}
>
</canvas>