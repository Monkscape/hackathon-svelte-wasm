<script>
    export let now;
    let frames = []
    let content = '';
    let lastFrameTimeStamp = performance.now();

    $: updateFrames(now);

    const updateFrames = () => {
        if (!now) return;
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
        content = `Frames per Second:
                latest = ${Math.round(fps)}
                avg of last 100 = ${Math.round(mean)}
                min of last 100 = ${Math.round(min)}
                max of last 100 = ${Math.round(max)}`.trim();
    }

</script>

<div class="fps">{content}</div>

<style>
    .fps {
        white-space: pre;
		font-family: monospace;
    }
</style>
