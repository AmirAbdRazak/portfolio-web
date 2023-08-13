<script lang="ts">
	import type { PageData } from './$types';
	import { getContextClient, queryStore, type OperationResultStore } from '@urql/svelte';
	import {
		ChartDocument,
		type ChartDataConfig,
		type ChartQueryStore
	} from '../../../../generated/graphql';
	import {
		Chart,
		LineController,
		LineElement,
		PointElement,
		LinearScale,
		Title,
		CategoryScale,
		Tooltip
	} from 'chart.js';
	import { onMount } from 'svelte';

	export let data: PageData;
	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';

	$: isMounted = false;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ChartDocument,
		variables: { username, chartType: 'album', limit: 20 }
	});

	let ctx: HTMLCanvasElement;

	onMount(() => {
		isMounted = true;
		ctx = document.getElementById('chart') as HTMLCanvasElement;

		Chart.register(
			LineController,
			LineElement,
			PointElement,
			LinearScale,
			Title,
			CategoryScale,
			Tooltip
		);
	});

	function getRandomColor() {
		return '#' + Math.floor(Math.random() * 16777215).toString(16);
	}

	function generateChart(chart_data: ChartDataConfig) {
		new Chart(ctx, {
			type: 'line',
			data: {
				labels: chart_data.labels.map((timestamp) =>
					new Date(timestamp * 1000).toLocaleDateString(locale, {
						year: '2-digit',
						month: 'short',
						day: 'numeric'
					})
				),
				datasets: chart_data.datasets.map((dataset) => {
					const rand_color = getRandomColor();
					return {
						label: dataset.chartEntry,
						data: dataset.playcount,
						borderColor: rand_color,
						backgroundColor: rand_color,
						pointHoverBackgroundColor: rand_color,
						pointHoverBorderColor: rand_color,
						pointRadius: 0,
						pointHoverRadius: 4,
						borderWidth: 2
					};
				})
			},
			options: {
				responsive: true,
				interaction: {
					mode: 'index',
					intersect: false
				},
				scales: {
					x: {
						ticks: {
							maxTicksLimit: 10,
							color: '#ffffff',
							font: {
								family: 'Montserrat'
							}
						}
					},
					y: {
						ticks: {
							color: '#ffffff',
							font: {
								family: 'Montserrat'
							}
						}
					}
				},

				plugins: {
					title: {
						display: true,
						text: 'Artist All Time Chart',
						color: '#ffffff',
						font: {
							family: 'Montserrat'
						}
					},
					tooltip: {
						enabled: true,
						mode: 'index',
						position: 'nearest',
						itemSort: function (a, b) {
							return (b.raw as number) - (a.raw as number);
						},
						titleFont: {
							family: 'Montserrat'
						},
						bodyFont: {
							family: 'Montserrat'
						}
					}
				}
			}
		});
		return '';
	}
</script>

<div class="bg-slate-800 p-4 text-white">
	<canvas id="chart" />
</div>
{#if $queryData.fetching}
	<p>Loading...</p>
{:else if $queryData.error}
	<p>{$queryData.error.message}</p>
{:else if isMounted && $queryData.data}
	{generateChart($queryData.data.historyFm.getWeeklyCharts.chart)}
{/if}
