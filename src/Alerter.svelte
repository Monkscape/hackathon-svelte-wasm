<script>
    import { onMount } from 'svelte';
    import wasm from './alert/Cargo.toml';

    let name = '';
    let wasmCompilation;

    onMount(async () => {
		wasmCompilation = await wasm();
	});
</script>

<input type='text' bind:value={name} />

{#await wasmCompilation then compiled}
    <button on:click={() => compiled.greet(name)}>Click me!</button>
{:catch err} 
    <div>Web Assembly Loading Failed</div>
{/await}

