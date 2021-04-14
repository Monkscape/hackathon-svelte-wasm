<script>
	import DogGenerator from "./DogGenerator.svelte";
	import GameOfLifeText from './GameOfLifeText.svelte';
	import MainPage from './MainPage.svelte';

	let component;

	const hashMap = {
		'#dogs': DogGenerator,
		'#gameoflife': GameOfLifeText
	}

	function hashChange() {
		console.log("Changing page to", location.hash);
		component = hashMap[location.hash] || MainPage;
	}

	hashChange();
	
</script>

<svelte:window on:hashchange={hashChange} />

<nav>
	<a href="/" class:active={component === MainPage}>Home</a>
	<a href="/#dogs" class:active={component === DogGenerator}>Dog Generator</a>
	<a href="/#gameoflife" class:active={component === GameOfLifeText}>Game of Life</a>
</nav>

<main>
	<svelte:component this={component} />
</main>

<style>
	:root {
	  --space: 0.5rem;
	}
   
	a {
	  background-color: white;
	  border-radius: var(--space);
	  margin-right: var(--space);
	  padding: var(--space);
	  text-decoration: none;
	}
   
	a.active {
	  background-color: yellow;
	}
   
	.icon {
	  padding-bottom: 6px;
	  padding-top: 6px;
	}
   
	main {
	  padding: var(--space);
	}
   
	nav {
	  display: flex;
	  align-items: center;
	  background-color: cornflowerblue;
	  padding: var(--space);
	}
</style>
