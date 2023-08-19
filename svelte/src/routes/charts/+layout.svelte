<script lang="ts">
	import '../../app.css';
	import {
		Client,
		cacheExchange,
		fetchExchange,
		setContextClient
	} from '@urql/svelte';
	import type { LayoutData } from './$types';

	export let data: LayoutData;

	const client = new Client({
		url: data.backend_url + 'graphql',
		exchanges: [cacheExchange, fetchExchange]
	});

	setContextClient(client);
</script>

<svelte:head>
	<style>
		@import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;600;800&display=swap');

		body {
			font-family: 'Montserrat';
			font-style: 'normal';
			font-weight: 400;
			background-color: #1e293b;
		}
	</style>
</svelte:head>

<slot />
