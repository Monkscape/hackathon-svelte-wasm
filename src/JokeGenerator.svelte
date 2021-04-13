<script>
    import { onMount } from "svelte";
    import wasm from './fetch/Cargo.toml';

    let joke = ''
    let wasmCompilation;


    onMount(async () => {
        wasmCompilation = await wasm();
    })

    const onClick = async () => {
        let res = await wasmCompilation.run();
        joke = res.value;
    }

</script>

{#await wasmCompilation then compiled}
    <button on:click={onClick}>Press me!</button>
{/await}
<div>Joke: {joke}</div>