<script>
import FrameRateDisplay from './FrameRateDisplay.svelte';

    import GameOfLife from './GameOfLife.svelte';

    let width = 64;
    let height = 64;
    let randomnessFactor = 50;

    let now;

    const captureFps = (event) => {
        now = event.detail;
    }

</script>

<form>
    <label>
        <p>Width [{width}]:</p>
        <input type='range' min={1} max={300} bind:value={width}>
    </label>
    <label>
        <p>Height [{height}]:</p>
        <input type='range' min={1} max={300} bind:value={height}>
    </label>
    <label>
        <p>Generation Percent [{randomnessFactor}%]:</p>
        <input type='range' min={1} max={100} bind:value={randomnessFactor}>
    </label>
</form>
<FrameRateDisplay {now} />
<GameOfLife {width} {height} {randomnessFactor} on:tick={captureFps}/>

<style>
    input, label, p {
        margin: 0px;
    }
</style>