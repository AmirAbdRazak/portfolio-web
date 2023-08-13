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
		Tooltip,
		Legend
	} from 'chart.js';
	import { onMount } from 'svelte';
	import { formDataStore, type ChartFormData } from '../FormDataStore';

	export let data: PageData;
	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';

	let formData: {
		chart_type: string;
		limit: number;
		offset: number;
	};

	formDataStore.subscribe((data) => {
		formData = {
			chart_type: data.chart_type,
			limit: data.limit || 10,
			offset: data.offset || 5
		};
	});

	$: isMounted = false;
	$: isFetched = false;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ChartDocument,
		variables: {
			username,
			chartType: formData['chart_type'].toLowerCase(),
			limit: formData['limit'],
			offset: formData['offset']
		}
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
			Tooltip,
			Legend
		);
	});

	function getRandomColor() {
		const red = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
		const green = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
		const blue = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)

		// Convert the RGB components to a hexadecimal color code
		const color = `#${((red << 16) | (green << 8) | blue).toString(16).padStart(6, '0')}`;

		return color;
	}

	function generateChart(chart_data: ChartDataConfig) {
		isFetched = true;
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
						pointHoverBackgroundColor: '#1e293b',
						pointHoverBorderColor: rand_color,
						pointStyle: 'rectRot',
						pointRadius: 0,
						pointHoverRadius: 6,
						borderWidth: 2,
						tension: 0.15
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
						text: `${formData.chart_type} All Time Chart`,
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
					},
					legend: {
						display: true,
						position: 'bottom',
						labels: {
							font: {
								family: 'Montserrat'
							},
							color: '#ffffff'
						}
					}
				}
			}
		});
		return '';
	}
</script>

<div class="{isFetched ? 'flex' : 'hidden'} bg-slate-800 p-40 text-white">
	<canvas id="chart" />
</div>
{#if $queryData.fetching}
	<p>Loading...</p>
{:else if $queryData.error}
	<p>{$queryData.error.message}</p>
{:else if isMounted && $queryData.data}
	{generateChart($queryData.data.historyFm.getWeeklyCharts.chart)}
{/if}
