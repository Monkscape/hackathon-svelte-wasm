<script>
    import { onMount } from 'svelte';
    import { default as wasm, greet} from 'wasm-alert';

    let name = '';
    let wasmCompilation;

    onMount(async () => {
        wasmCompilation = await wasm();
		console.log(wasmCompilation);
	});
</script>

<input type='text' bind:value={name} />

{#await wasmCompilation then compiled}
    <button on:click={() => greet(name)}>Click me!</button>
{:catch err} 
    <div>Web Assembly Loading Failed</div>
{/await}

