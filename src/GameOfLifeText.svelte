<script>
import { onMount } from 'svelte';

    let pre;
    let play_pause;
    let framesPerSecond;
    import wasm from './alert/Cargo.toml';

    let universe;

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

    const play = () => {
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
        let wasmCompilation = await wasm();
        universe = wasmCompilation.Universe.new();
    })

    const renderLoop = () => {
        renderFPS();
        console.log(universe);
        pre.textContent = universe.render();
        universe.tick();

        animationId = requestAnimationFrame(renderLoop);
    }

</script>
<button bind:this={play_pause} on:click={pauseOrPlay}>Play</button>
<div bind:this={framesPerSecond}></div>
<pre bind:this={pre}></pre>