<script>
	import DogGenerator from "./DogGenerator.svelte";
	import GameOfLifeCreator from './GameOfLifeCreator.svelte';
	import MainPage from './MainPage.svelte';

	let component;

	const hashMap = {
		'#dogs': DogGenerator,
		'#gameoflife': GameOfLifeCreator
	}

	function hashChange() {
		console.log("Changing page to", location.hash);
		component = hashMap[location.hash] || MainPage;
	}

	hashChange();
	
</script>

<svelte:window on:hashchange={hashChange} />

<nav>
	<a href="/" class:active={component === MainPage}>&#127960 Home</a>
	<a href="/#dogs" class:active={component === DogGenerator}>&#128021 Dog Generator</a>
	<a href="/#gameoflife" class:active={component === GameOfLifeCreator}>&#127922 Game of Life</a>
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
 
	main {
	  padding: var(--space);
	}
   
	nav {
	  display: flex;
	  align-items: center;
	  background-color: cornflowerblue;
	  padding: var(--space);
	  margin: 0;
	}
</style>
