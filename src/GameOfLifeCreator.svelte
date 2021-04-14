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
        Width:
        <input type='number' bind:value={width}>
    </label>
    <label>
        Height:
        <input type='number' bind:value={height}>
    </label>
    <label>
        Generation Percent {randomnessFactor}%
        <input type='range' min={1} max={100} bind:value={randomnessFactor}>
    </label>
</form>
<FrameRateDisplay {now} />
<GameOfLife {width} {height} {randomnessFactor} on:tick={captureFps}/>