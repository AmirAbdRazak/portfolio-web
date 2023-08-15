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
		LogarithmicScale,
		LinearScale,
		Title,
		CategoryScale,
		Tooltip,
		Legend,
		TimeSeriesScale
	} from 'chart.js';
	import 'chartjs-adapter-date-fns';
	import CustomLogScale from './CustomLogScale';

	import { onMount } from 'svelte';
	import { formDataStore, type ChartFormData } from '../FormDataStore';
	import Loader from './Loader.svelte';
	import { goto } from '$app/navigation';

	export let data: PageData;
	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';
	let current_chart: Chart;

	let reenter_username: string;
	let reenter_limit: number = 10;
	let reenter_offset: number = 0;
	let reenter_chart_type: string = 'Artist';

	let formData: {
		chart_type: string;
		limit: number;
		offset: number;
	};

	formDataStore.subscribe((data) => {
		formData = {
			chart_type: data.chart_type,
			limit: data.limit || 10,
			offset: data.offset || 0
		};
	});

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chart_type: reenter_chart_type,
			limit: reenter_limit,
			offset: reenter_offset
		});
		current_chart.destroy();
		goto(`${reenter_username}`, { replaceState: true });
	}

	// why is this not working

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
			CustomLogScale,
			TimeSeriesScale,
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

	function getBaseLog(x: number, y: number) {
		return Math.log(y) / Math.log(x);
	}

	function generateChart(chart_data: ChartDataConfig) {
		isFetched = true;

		current_chart = new Chart(ctx, {
			type: 'line',
			data: {
				labels: chart_data.labels.map((timestamp) => new Date(timestamp * 1000)),
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
				aspectRatio: screenSize >= 768 ? 2 : screenSize >= 640 ? 1.5 : screenSize >= 300 ? 1 : 0.5,
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
						//@ts-ignore
						type: 'customLog',
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

<svelte:window bind:innerWidth={screenSize} class="hidden" />

<div class="{isFetched ? 'flex' : 'hidden'} h-100 md:h-full bg-slate-800 pt-10 px-5 text-white">
	<canvas id="chart" />
</div>
<div
	class="flex flex-col items-center justify-center sm:flex-row bg-slate-800 mx-auto place-self-center lg:col-span-7"
>
	<form method="POST" on:submit={handleSubmit}>
		<input
			class="flex sm:inline-flex items-center justify-center py-3 mb-5 mt-10 sm:mb-0 md:pr-5 md:py-3 mr-3 w-80 text-slate-100 font-medium drop-shadow-lg text-center bg-slate-700 border-2 border-slate-700 rounded-lg focus:ring-3 focus:ring-slate-800 focus:outline-none"
			bind:value={reenter_username}
			placeholder="Enter your username"
		/>
		<div class="flex flex-col py-5">
			<div class="pb-2">
				<label for="limit-range" class="inline-flex mb-2 text-sm font-medium text-white"
					>Data Limit:
				</label>
				<input
					bind:value={reenter_limit}
					class="inline-flex mb-2 text-md border-none bg-transparent text-white focus:outline-none pl-1 pr-5"
				/>
			</div>
			<input
				id="limit-range"
				type="range"
				bind:value={reenter_limit}
				class="w-full h-1 mb-6 rounded-lg appearance-none cursor-pointer range-sm bg-slate-700"
			/>
			<div class="pb-2">
				<label for="offset-range" class="inline-flex mb-2 text-sm font-medium text-white"
					>Data Offset:
				</label>
				<input
					bind:value={reenter_offset}
					class="inline-flex mb-2 text-md border-none bg-transparent text-white focus:outline-none pl-1 pr-5"
				/>
			</div>
			<input
				id="offset-range"
				type="range"
				bind:value={reenter_offset}
				class="w-full h-1 mb-6 rounded-lg appearance-none cursor-pointer range-sm bg-slate-700"
			/>
			<div
				class="group pt-5 flex sm:inline-flex focus-within:ring-4 mx-auto focus-within:ring-slate-800 rounded-lg sm:ml-auto"
			>
				<button
					class="hidden md:block items-center justify-center px-5 py-2 md:py-3 text-sm md:text-base font-semibold text-center bg-rose-400 border rounded-l-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none"
					type="submit"
				>
					Generate chart
				</button>
				<button
					class="md:hidden items-center justify-center px-5 py-2 md:py-3 text-base font-semibold text-center bg-rose-400 border rounded-l-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none"
					type="submit"
				>
					Generate
				</button>
				<select
					id="chart_type"
					class="items-center justify-center px-5 py-2 md:py-3 text-base font-semibold text-center bg-rose-400 border-y border-r rounded-r-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none cursor-pointer"
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
		class="fixed top-0 left-0 right-0 bottom-0 w-full h-screen z-50 overflow-hidden bg-slate-800 flex flex-col items-center justify-center"
	>
		<p class="w-1/3 text-center text-white">
			Some error occurred, either check your username, or its on our side. Eitherway, here's the
			error: <b>{$queryData.error.name}: {$queryData.error.message}</b>
		</p>
	</div>
{:else if isMounted && $queryData.data}
	{generateChart($queryData.data.historyFm.getWeeklyCharts.chart)}
{/if}
