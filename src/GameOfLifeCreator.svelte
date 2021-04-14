<script>
import FrameRateDisplay from './FrameRateDisplay.svelte';

    import GameOfLife from './GameOfLife.svelte';

    let width = 64;
    let height = 64;
    let randomnessFactor = 50;
    let cellSize = 5;

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
    <label>
        <p>Cell Size [{cellSize}]:</p>
        <input type='range' min={5} max={30} step={5} bind:value={cellSize}>
    </label>
</form>
<FrameRateDisplay {now} />
<GameOfLife {width} {height} {randomnessFactor} {cellSize} on:tick={captureFps}/>

<style>
    input, label, p {
        margin: 0px;
    }
    form {
        text-align: center;
    }
</style>