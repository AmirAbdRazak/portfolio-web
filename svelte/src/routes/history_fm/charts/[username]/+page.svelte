<script lang="ts">
	import type { PageData } from './$types';
	import { getContextClient, queryStore, type OperationResultStore } from '@urql/svelte';
	import { ArtistChartDocument, type ArtistChartQuery } from '../../../../generated/graphql';
	import {
		Chart,
		type ChartConfiguration,
		type ChartTypeRegistry,
		LineController,
		LineElement,
		PointElement,
		LinearScale,
		Title,
		CategoryScale
	} from 'chart.js';
	import { onMount } from 'svelte';

	export let data: PageData;
	let username = data.username;

	type ArtistPlaycountMapping = {
		[artistName: string]: {
			playcount: number[];
			prevTotal: number;
			lastIterationUpdated: number;
		};
	};
	let queryData: OperationResultStore;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ArtistChartDocument,
		variables: { username }
	});

	let artistPlaycountMapping: ArtistPlaycountMapping = {};

	function generateChart(): ChartConfiguration<keyof ChartTypeRegistry, number[]> {
		const rawArtistData: ArtistChartQuery = $queryData.data;
		const artistData = rawArtistData.historyFm.getWeeklyCharts.artist;

		artistData.forEach((artistEntry, iteration) => {
			artistEntry.artist.forEach((artist) => {
				if (!(artist.name in artistPlaycountMapping)) {
					artistPlaycountMapping[artist.name] = {
						playcount: new Array(iteration).fill(0),
						prevTotal: 0,
						lastIterationUpdated: iteration
					};
				}

				const artistMap = artistPlaycountMapping[artist.name];

				const prevTotal = artistMap['prevTotal'];
				const currentPlaycount = parseInt(artist.playcount);
				artistMap['prevTotal'] += currentPlaycount;

				if (iteration > artistMap['lastIterationUpdated'] + 1) {
					artistMap['playcount'].push(
						...new Array(iteration - artistMap['lastIterationUpdated'] - 1).fill(
							artistMap['prevTotal']
						)
					);
				}

				artistMap['playcount'].push(currentPlaycount + prevTotal);
				artistMap['lastIterationUpdated'] = iteration;
			});
		});

		const data = {
			labels: artistData.map((artistEntry) => artistEntry.attr.to),
			datasets: Object.entries(artistPlaycountMapping).map(
				([artistName, { playcount }]: [string, { playcount: number[] }]) => ({
					label: artistName,
					data: playcount
					// borderColor: '#005550',
					// backgroundColor: '#005550',
					// yAxisID: artistName
				})
			)
		};

		return {
			type: 'line',
			data: data,
			options: {
				responsive: true,
				interaction: {
					mode: 'index',
					intersect: false
				},
				plugins: {
					title: {
						display: true,
						text: 'Artist All Time Chart'
					}
				}
			}
		};
	}

	let isMounted = false;
	let ctx: HTMLCanvasElement;

	onMount(() => {
		isMounted = true;
		ctx = document.getElementById('chart') as HTMLCanvasElement;

		Chart.register(LineController, LineElement, PointElement, LinearScale, Title, CategoryScale);
	});
</script>

<div>
	Canvas should have been here
	<canvas id="chart" />
</div>
{#if $queryData.fetching}
	<p>Loading...</p>
{:else if $queryData.error}
	<p>{$queryData.error.message}</p>
{:else if isMounted}
	{new Chart(ctx, generateChart())}
{/if}
