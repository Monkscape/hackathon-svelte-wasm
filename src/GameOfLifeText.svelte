<script>
import { onMount } from 'svelte';
    import wasm from './alert/Cargo.toml';

    let pre;
    let play_pause;

    let wasmCompilation;
    let universe;
    let width = 64;
    let height = 64;

    let animationId = null;

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
<pre bind:this={pre}></pre>