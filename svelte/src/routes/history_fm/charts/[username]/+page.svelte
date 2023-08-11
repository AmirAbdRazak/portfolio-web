<script lang="ts">
	import type { PageData } from './$types';
	import { getContextClient, queryStore } from '@urql/svelte';
	import { ArtistChartDocument, type ArtistChartQuery } from '../../../../generated/graphql';
	import { error } from '@sveltejs/kit';

	export let data: PageData;
	let username = data.username;

	type ArtistPlaycountMapping = {
		[artist_name: string]: {
			date: string;
			playcount: number;
		}[];
	};

	let artistPlaycountMapping: ArtistPlaycountMapping;

	$: artistPlaycountMapping = {};

	$: queryData = queryStore({
		client: getContextClient(),
		query: ArtistChartDocument,
		variables: { username }
	});
	function generateChart() {
		const rawArtistData: ArtistChartQuery = $queryData.data;
		const artistData = rawArtistData.historyFm.getWeeklyCharts.artist;

		const dateRanges = artistData.map((artistEntry) => artistEntry.attr.to);
		artistData.forEach((artistEntry) => {
			artistEntry.artist.forEach((artist) => {
				if (!(artist.name in artistPlaycountMapping)) {
					artistPlaycountMapping[artist.name] = [];
				}

				artistPlaycountMapping[artist.name].push({
					date: artistEntry.attr.to,
					playcount: parseInt(artist.playcount)
				});
			});
		});

		console.log('===artist play count mapping===');
		console.log(artistPlaycountMapping);

		console.log(dateRanges);

		const m = artistPlaycountMapping;
		return m;
	}
</script>

{#if $queryData.fetching}
	<p>Loading...</p>
{:else if $queryData.error}
	<p>{$queryData.error.message}</p>
{:else}
	{JSON.stringify(generateChart())}
	{JSON.stringify(artistPlaycountMapping)}
{/if}
