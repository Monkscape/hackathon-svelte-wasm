<script>
import { onMount } from 'svelte';

    let pre;
    import wasm from './alert/Cargo.toml';

    let universe;

    onMount(async () => {
        let wasmCompilation = await wasm();
        universe = wasmCompilation.Universe.new();
    })

    const renderLoop = () => {
        console.log(universe);
        pre.textContent = universe.render();
        universe.tick();

        requestAnimationFrame(renderLoop);
    }
</script>

<pre bind:this={pre}></pre>
<button on:click={renderLoop}>Start Game!</button>