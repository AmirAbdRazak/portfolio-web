<script lang="ts">
	import type { PageData } from './$types';
	import { getContextClient, queryStore } from '@urql/svelte';
	import {
		ChartDocument,
		type ChartDataConfig,
		type ChartQueryStore
	} from '../../../generated/graphql';
	import {
		Chart,
		LineController,
		LineElement,
		PointElement,
		LogarithmicScale,
		LinearScale,
		Title,
		CategoryScale,
		Tooltip,
		Legend,
		TimeSeriesScale
	} from 'chart.js';
	import 'chartjs-adapter-date-fns';

	import { onMount } from 'svelte';
	import { START_TIMESTAMP, formDataStore } from '../FormDataStore';
	import Loader from './Loader.svelte';
	import { goto } from '$app/navigation';
	import zoomPlugin from 'chartjs-plugin-zoom';

	export let data: PageData;
	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';
	let current_chart: Chart;

	let formData: {
		chart_type: string;
		limit: number;
		offset: number;
		start_timestamp: number;
		end_timestamp: number;
		chart_scale: 'linear' | 'logarithmic';
	};

	formDataStore.subscribe((data) => {
		formData = {
			chart_type: data.chart_type,
			limit: data.limit || 10,
			offset: data.offset || 0,
			start_timestamp: data.start_timestamp,
			end_timestamp: data.end_timestamp,
			chart_scale: data.chart_scale
		};
	});

	let reenter_username: string = username;
	let reenter_limit: number = 10;
	let reenter_offset: number = 0;
	let reenter_chart_type: string = 'Artist';
	let reenter_chart_scale: 'linear' | 'logarithmic' = 'linear';
	let reenter_start_timestamp = START_TIMESTAMP;
	let reenter_end_timestamp = Date.now() * 1000;

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chart_type: reenter_chart_type,
			limit: reenter_limit,
			offset: reenter_offset,
			start_timestamp: reenter_start_timestamp,
			end_timestamp: reenter_end_timestamp,
			chart_scale: reenter_chart_scale
		});
		current_chart.destroy();
		goto(`${reenter_username}`, { replaceState: true });
	}

	$: isMounted = false;
	$: isFetched = false;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ChartDocument,
		variables: {
			username,
			chartType: formData['chart_type'].toLowerCase(),
			limit: parseInt(formData['limit'] + ''),
			offset: parseInt(formData['offset'] + ''),
			start_timestamp: 165424,
			end_timestamp: 1654
		}
	});

	let ctx: HTMLCanvasElement;
	let screenSize: number = 768;

	onMount(() => {
		isMounted = true;
		ctx = document.getElementById('chart') as HTMLCanvasElement;

		Chart.register(
			LineController,
			LineElement,
			PointElement,
			LinearScale,
			LogarithmicScale,
			TimeSeriesScale,
			Title,
			CategoryScale,
			Tooltip,
			Legend,
			zoomPlugin
		);
	});

	function getRandomColor() {
		const red = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
		const green = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)
		const blue = Math.floor(Math.random() * 128) + 128; // Bias towards higher values (128-255)

		const color = `#${((red << 16) | (green << 8) | blue)
			.toString(16)
			.padStart(6, '0')}`;

		return color;
	}

	function generateChart(chart_data: ChartDataConfig) {
		isFetched = true;

		current_chart = new Chart(ctx, {
			type: 'line',
			data: {
				labels: chart_data.labels.map(
					(timestamp) => new Date(timestamp * 1000)
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
				aspectRatio:
					screenSize >= 768
						? 2
						: screenSize >= 640
						? 1.5
						: screenSize >= 300
						? 1
						: 0.5,
				interaction: {
					mode: 'index',
					intersect: false
				},
				scales: {
					x: {
						type: 'timeseries',
						time: {
							displayFormats: {
								quarter: 'MMM YYYY'
							}
						},
						ticks: {
							color: '#ffffff',
							font: {
								family: 'Montserrat'
							},
							callback: (value, index) =>
								index % 12 == 0 || index == 0
									? new Date(value).toLocaleDateString(locale, {
											year: 'numeric',
											month: 'short'
									  })
									: null
						}
					},
					y: {
						type: formData.chart_scale,
						min: formData.chart_scale == 'linear' ? 0 : 1,
						ticks: {
							maxTicksLimit: 15,
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
					},
					zoom: {
						limits: {
							y: { min: 'original', max: 'original' },
							x: { min: 'original', max: 'original' }
						},
						zoom: {
							wheel: {
								enabled: true,
								modifierKey: 'ctrl'
							},
							pinch: {
								enabled: true
							},
							mode: 'xy'
						},
						pan: {
							enabled: true,
							mode: 'xy'
						}
					}
				}
			}
		});
		return '';
	}
</script>

<svelte:window bind:innerWidth={screenSize} class="hidden" />

<div
	class="{isFetched
		? 'flex'
		: 'hidden'} h-100 bg-slate-800 px-5 pt-10 text-white md:h-full"
>
	<canvas id="chart" />
</div>
<div
	class="mx-auto flex flex-col items-center justify-center place-self-center bg-slate-800 sm:flex-row lg:col-span-7"
>
	<form method="POST" on:submit={handleSubmit}>
		<div class="flex flex-col py-5">
			<input
				class="focus:ring-3 mb-5 mt-5 flex w-full items-center justify-center rounded-lg border-2 border-slate-700 bg-slate-700 py-3 text-center font-medium text-slate-100 focus:outline-none focus:ring-slate-800 sm:inline-flex md:py-3 md:pr-5"
				bind:value={reenter_username}
				placeholder="Enter your username"
			/>
			<div class="pb-2">
				<label
					for="limit-range"
					class="mb-2 inline-flex text-sm font-medium text-white"
					>Data Limit:
				</label>
				<input
					bind:value={reenter_limit}
					class="text-md mb-2 inline-flex border-none bg-transparent pl-1 pr-5 text-white focus:outline-none"
				/>
			</div>
			<input
				id="limit-range"
				type="range"
				bind:value={reenter_limit}
				class="range-sm mb-6 h-1 w-full cursor-pointer appearance-none rounded-lg bg-slate-700"
			/>
			<div class="pb-2">
				<label
					for="offset-range"
					class="mb-2 inline-flex text-sm font-medium text-white"
					>Data Offset:
				</label>
				<input
					bind:value={reenter_offset}
					class="text-md mb-2 inline-flex border-none bg-transparent pl-1 pr-5 text-white focus:outline-none"
				/>
			</div>
			<input
				id="offset-range"
				type="range"
				bind:value={reenter_offset}
				class="range-sm mb-6 h-1 w-full cursor-pointer appearance-none rounded-lg bg-slate-700"
			/>
			<div
				class="group mx-auto flex rounded-lg pt-5 focus-within:ring-4 focus-within:ring-slate-800 sm:ml-auto sm:inline-flex"
			>
				<button
					class="hidden items-center justify-center rounded-l-lg border-y border-l border-rose-700 bg-rose-400 px-5 py-2 text-center text-sm font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:block md:py-3 md:text-base"
					type="submit"
				>
					Generate chart
				</button>
				<button
					class="items-center justify-center rounded-l-lg border-y border-l border-rose-700 bg-rose-400 px-5 py-2 text-center text-base font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:hidden md:py-3"
					type="submit"
				>
					Generate
				</button>
				<select
					id="chart_type"
					class="cursor-pointer items-center justify-center border border-rose-700 bg-rose-400 px-5 py-2 text-center text-base font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3"
					bind:value={reenter_chart_scale}
				>
					<option selected value="linear">Linear</option>
					<option value="logarithmic">Log</option>
				</select>
				<select
					id="chart_type"
					class="cursor-pointer items-center justify-center rounded-r-lg border-y border-r border-rose-700 bg-rose-400 px-5 py-2 text-center text-base font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3"
					bind:value={reenter_chart_type}
				>
					<option selected value="Artist">Artist</option>
					<option value="Album">Album</option>
					<option value="Track">Track</option>
				</select>
			</div>
		</div>
	</form>
</div>
{#if $queryData.fetching}
	<Loader {username} />
{:else if $queryData.error}
	<div
		class="fixed bottom-0 left-0 right-0 top-0 z-50 flex h-screen w-full flex-col items-center justify-center overflow-hidden bg-slate-800"
	>
		<p class="w-1/3 text-center text-white">
			Some error occurred, either check your username, or its on our side.
			Eitherway, here's the error: <b
				>{$queryData.error.name}: {$queryData.error.message}</b
			>
		</p>
	</div>
{:else if isMounted && $queryData.data}
	{generateChart($queryData.data.historyFm.getWeeklyCharts.chart)}
{/if}
