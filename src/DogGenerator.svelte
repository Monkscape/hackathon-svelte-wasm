<script>
    import { onMount } from "svelte";
    import { flip } from 'svelte/animate';
    import {default as wasm, DogRequest, run } from 'wasm-fetch';

    let images = [];
    let wasmCompilation;
    let message = '';
    let breed = '';
    let number = 1;
    let horizontal = true;

    let difference;
    $: difference = images.length - number;

    onMount(async () => {
        wasmCompilation = await wasm();
    })

    const getMoreDogs = async () => {
        let breedRequest = breed.toLowerCase().replace(/\s+/g, '');
        
        if (difference > 0) {
            images = images.splice(0, number);
        } else {
            let dogs = await fetchDogs(breedRequest, -difference);
            images.unshift(...dogs);
            images = images;
        }
    }

    const fetchDogs = async (breedRequest, number) => {
        let request = DogRequest.new(breedRequest, number);
        let response = await run(request);
        return response.message;
    }

    const getNewDogArray = async () => {
        let breedRequest = breed.toLowerCase().replace(/\s+/g, '');
        let response = await fetchDogs(breedRequest, number);
        console.log(response);
        images = response;
        if (!images.length) {
            message = `No images found for "${breed}"`
        }
    }

</script>

<label>
    Horizontal:
    <input type="checkbox" bind:checked={horizontal}>
</label>

{#await wasmCompilation then compiled}
    <form>
        <label>
            <p>Breed</p>
            <input type='text' bind:value={breed} on:change={getNewDogArray}>
        </label>
        <label>
            <p>Number of Pictures [{number}]</p>
            <input disabled={!breed} type='range' min=1 max=50 bind:value={number} on:change={getMoreDogs}>
        </label>
    </form>
{:catch error}
    <div>WASM Compilation Error: {error}</div>
{/await}


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
    p {
        margin: 0px;
    }

    img {
        width:inherit;
        height: inherit;
    }

    .dog {
        width: 240px;
        height: 240px;
    }

    .horizontal {
        display: inline-block;
        margin-left: 10px;
    }
</style>