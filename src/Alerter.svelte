<script>
    import { onMount } from 'svelte';
    import { default as wasm, greet} from 'wasm-alert';

    export let name = '';
    let wasmCompilation;

    onMount(async () => {
        wasmCompilation = await wasm();
		console.log(wasmCompilation);
	});
</script>

<form>
    <input type='text' bind:value={name} />

    {#await wasmCompilation then compiled}
        <button on:click={() => greet(name)}>Click me!</button>
    {:catch err} 
        <div>Web Assembly Loading Failed</div>
    {/await}
</form>

<style>
    form {
        text-align: center;
    }
</style>

