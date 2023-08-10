<script lang="ts">
	import '../app.css';
	import { Client, cacheExchange, fetchExchange, setContextClient } from '@urql/svelte';

	const client = new Client({
		url: 'http://localhost:8000/graphql',
		exchanges: [cacheExchange, fetchExchange]
	})

    setContextClient(client);
</script>

<svelte:head>
	<style>
        @import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;600;800&display=swap');

        body {
            font-family: 'Montserrat';
            font-style: 'normal';
            font-weight: 400;

        }
	</style>
</svelte:head>

<slot />
