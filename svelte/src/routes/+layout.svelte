<script>
	import '../app.postcss';
	import NavBar from './NavBar.svelte';
</script>

<svelte:head>
	<style>
		@import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;600;800&display=swap');

		body {
			font-family: 'Montserrat';
			font-style: 'normal';
			font-weight: 400;
			background-color: #18181b;
		}
	</style>
</svelte:head>

<NavBar />
<slot />
