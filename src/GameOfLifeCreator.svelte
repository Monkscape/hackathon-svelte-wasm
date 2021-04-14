<script>
    import GameOfLife from './GameOfLife.svelte';

    let framesPerSecond;

    let width = 64;
    let height = 64;
    let randomnessFactor = 50;

    let frames = []
    let lastFrameTimeStamp = performance.now();

    const renderFPS = (event) => {
        const now = event.detail;
        const delta = now - lastFrameTimeStamp;
        lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        frames.push(fps);
        if (frames.length > 100) {
            frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < frames.length; i++) {
            sum += frames[i];
            min = Math.min(frames[i], min);
            max = Math.max(frames[i], max);
        }
        let mean = sum / frames.length;

        // Render the statistics.
        framesPerSecond.textContent = `Frames per Second:
                latest = ${Math.round(fps)}
                avg of last 100 = ${Math.round(mean)}
                min of last 100 = ${Math.round(min)}
                max of last 100 = ${Math.round(max)}`.trim();
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
<div class="fps" bind:this={framesPerSecond}></div>
<GameOfLife {width} {height} {randomnessFactor} on:tick={renderFPS}/>

<style>
    .fps {
        white-space: pre;
        font-family: monospace;
    }
</style>