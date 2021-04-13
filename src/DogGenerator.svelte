<script>
    import { onMount } from "svelte";
    import { flip } from 'svelte/animate';
    import wasm from './fetch/Cargo.toml';

    let images = [];
    let wasmCompilation;
    let message = '';
    let breed = '';
    let number = 0;
    let horizontal = false;

    onMount(async () => {
        wasmCompilation = await wasm();
    })

    const getDogs = async () => {
        console.log(breed);
        let res = await wasmCompilation.run(breed.toLowerCase(), number);
        console.log('res', res);
        images = res.message;
        if (!images.length) {
            message = `No images found for "${breed}"`
        }
    }

</script>

<label>
    Horizontal
    <input type="checkbox" bind:checked={horizontal}>
</label>

<form on:submit|preventDefault={getDogs}>
    <label>
        Breed
        <input type='text' bind:value={breed}>
    </label>
    <label>
        Number of Pictures {number}
        <input type='range' min=0 max=20 bind:value={number}>
    </label>
    {#await wasmCompilation then compiled}
        <button disabled={!breed}>Get Dogs</button>
    {:catch error}
        <div>WASM Compilation Error: {error}</div>
    {/await}
</form>


{#each images as imageUrl, index (index)}
    <div class="dog" animate:flip={{duration: 500}} class:horizontal>
        <img alt="dog" src={imageUrl}>
    </div>
{:else}
    {#if message}
        <div>{message}</div>
    {/if}
{/each}

<style>
    img {
        width:inherit;
    }

    .dog {
        width: 120px;
        height: 120px;
        width: fit-content;
        height: fit-content;
    }

    .horizontal {
        display: inline-block;
        margin-left: 10px;
    }
</style>