<script lang="ts">
	import '../../app.postcss';
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

<slot />
