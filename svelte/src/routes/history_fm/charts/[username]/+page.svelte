<script lang="ts">
	import type { PageData } from './$types';
	import { getContextClient, queryStore, type OperationResultStore } from '@urql/svelte';
	import { ChartDocument, type ChartQueryStore } from '../../../../generated/graphql';
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
	import { split } from 'postcss/lib/list';

	export let data: PageData;
	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';

	$: isMounted = false;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ChartDocument,
		variables: { username, chartType: 'artist', limit: 10 }
	});

	let chart_dataset;

	if (isMounted && !$queryData.fetching && !$queryData.error) {
		chart_dataset = $queryData.data?.historyFm.getWeeklyCharts.chart;
	}
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
{:else if isMounted && $queryData.data}
	{new Chart(ctx, {
		type: 'line',
		data: {
			labels: $queryData.data.historyFm.getWeeklyCharts.chart.labels.map((timestamp) =>
				new Date(timestamp * 1000).toLocaleDateString(locale, {
					year: '2-digit',
					month: 'short',
					day: 'numeric'
				})
			),
			datasets: $queryData.data.historyFm.getWeeklyCharts.chart.datasets.map((dataset) => ({
				label: dataset.chartEntry,
				data: dataset.playcount
			}))
		},
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
	})}
{/if}
