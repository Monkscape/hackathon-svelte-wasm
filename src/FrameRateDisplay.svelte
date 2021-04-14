<script>
    export let now;
    let frames = []
    let fps = 0;
    let mean;
    let min;
    let max;
    let lastFrameTimeStamp = performance.now();

    $: updateFrames(now);

    const updateFrames = () => {
        if (!now) return;
        const delta = now - lastFrameTimeStamp;
        lastFrameTimeStamp = now;
        const frame = 1 / delta * 1000;

        // Save only the latest 100 timings.
        frames.push(frame);
        if (frames.length > 100) {
            frames.shift();
        }
        fps = Math.round(frame);

        // Find the max, min, and mean of our 100 latest timings.
        let newMin = Infinity;
        let newMax = -Infinity;
        let sum = 0;
        for (let i = 0; i < frames.length; i++) {
            sum += frames[i];
            newMin = Math.min(frames[i], newMin);
            newMax = Math.max(frames[i], newMax);
        }
        mean = Math.round(sum / frames.length);
        min = Math.round(newMin);
        max = Math.round(newMax);
    }

</script>

{#if fps > 0}
<div class="fps">
    <span>fps: {fps}</span>
    <span>min: {min}</span>
    <span>max: {max}</span>
    <span>average: {mean}</span>
</div>
{/if}


<style>
    .fps {
        white-space: pre;
		font-family: monospace;
        font-size: 1.3em;
        text-align: center;
    }
</style>
