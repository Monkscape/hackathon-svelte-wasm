<script>
    import Alerter from "./Alerter.svelte";

    let name = 'Noah';
</script>

<main>
	<h1>Team Chinchilla World Domination</h1>
	<h2>Rust + WebAssembly & Svelte</h2>
    <h2>Welcome {name}!</h2>
</main>
<Alerter bind:name={name}/>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
        margin: 0.5em 0;
	}

    h2 {
        color: #ff3e00;
		font-size: 2em;
		font-weight: 20;
        margin: 0;
    }

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>