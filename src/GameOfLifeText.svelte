<script>
import { onMount } from 'svelte';

    let pre;
    let play_pause;
    import wasm from './alert/Cargo.toml';

    let universe;

    let animationId = null;

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
        console.log(universe);
        pre.textContent = universe.render();
        universe.tick();

        animationId = requestAnimationFrame(renderLoop);
    }

</script>

<button bind:this={play_pause} on:click={pauseOrPlay}>Play</button>
<pre bind:this={pre}></pre>