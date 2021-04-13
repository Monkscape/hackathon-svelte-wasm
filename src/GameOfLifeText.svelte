<script>
import { onMount } from 'svelte';
    import wasm from './alert/Cargo.toml';

    let pre;
    let play_pause;
    let framesPerSecond;
    let wasmCompilation;
    let universe;
    let width = 64;
    let height = 64;

    let animationId = null;
    let frames = []
    let lastFrameTimeStamp = performance.now();

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
        pre.textContent = universe.render();
    }

    const createUniverse = () => {
        if (!universe) {
            universe = wasmCompilation.Universe.new(width, height);
        }
    }

    const play = () => {
        createUniverse();
        play_pause.textContent = "Pause";
        renderLoop();
    };

    const pause = () => {
        play_pause.textContent = "Play";
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

    onMount(async () => {
        wasmCompilation = await wasm();
    })

    const renderLoop = () => {
        renderFPS();
        console.log(universe);
        pre.textContent = universe.render();
        universe.tick();

        animationId = requestAnimationFrame(renderLoop);
    }

</script>
<label>
    Width:
    <input type='number' bind:value={width}>
</label>
<label>
    Height:
    <input type='number' bind:value={height}>
</label>
<button bind:this={play_pause} on:click={pauseOrPlay}>Play</button>
<button disabled={animationId} on:click={resetUniverse}>Reset</button>
<div bind:this={framesPerSecond}></div>
<pre bind:this={pre}></pre>