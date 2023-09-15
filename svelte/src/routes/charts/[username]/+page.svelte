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
	import { formDataStore } from '../FormDataStore';
	import Loader from './Loader.svelte';
	import { goto } from '$app/navigation';
	import zoomPlugin from 'chartjs-plugin-zoom';
	import { _getChartConfig } from './+page';
	import type { formDataType } from './+page';
	import GenerateOptions from '../GenerateOptions.svelte';
	import * as Alert from '$lib/components/ui/alert';

	export let data: PageData;

	let queryData: ChartQueryStore;
	let username = data.username;
	let locale = 'en-us';
	let currentChart: Chart;

	let formData: formDataType;

	formDataStore.subscribe((data) => {
		formData = {
			chartType: data.chartType,
			limit: data.limit || 10,
			offset: data.offset || 0,
			startTimestamp: data.startTimestamp,
			endTimestamp: data.endTimestamp,
			chartScale: data.chartScale,
			dateRange: data.dateRange
		};
	});

	let limit: number[] = [10];
	let offset: number[] = [0];
	let chartType: string = 'Artist';
	let chartScale: 'linear' | 'logarithmic' = 'linear';
	let oneMonthAgo = new Date();
	oneMonthAgo.setMonth(oneMonthAgo.getMonth() - 1);
	let startTimestamp = oneMonthAgo.getTime() / 1000;
	let endTimestamp = Date.now() / 1000;
	let dateRange: 'Week' | 'Month' | 'Quarter' | 'Year' | 'Custom' = 'Custom';
	let invalidDateAlert = false;

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chartType,
			limit: limit[0],
			offset: offset[0],
			startTimestamp,
			endTimestamp,
			chartScale,
			dateRange
		});
		currentChart.destroy();
		goto(`${username}`, { replaceState: true });
	}

	$: isMounted = false;
	$: isFetched = false;
	$: queryData = queryStore({
		client: getContextClient(),
		query: ChartDocument,
		variables: {
			username,
			chartType: formData['chartType'].toLowerCase(),
			limit: parseInt(formData['limit'] + ''),
			offset: parseInt(formData['offset'] + ''),
			startTimestamp: parseInt(formData['startTimestamp'] + ''),
			endTimestamp: parseInt(formData['endTimestamp'] + '')
		}
	});

	let ctx: HTMLCanvasElement;
	let screenSize: number = 768;

	onMount(() => {
		isMounted = true;
		ctx = document.getElementById('chart') as HTMLCanvasElement;

		chartType = formData['chartType'];
		limit = [parseInt(formData['limit'] + '')];
		offset = [parseInt(formData['offset'] + '')];
		startTimestamp = parseInt(formData['startTimestamp'] + '');
		endTimestamp = parseInt(formData['endTimestamp'] + '');
		dateRange = formData['dateRange'];

		formDataStore.set({
			chartType,
			limit: limit[0],
			offset: offset[0],
			startTimestamp,
			endTimestamp,
			chartScale,
			dateRange
		});

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

	function generateChart(chartData: ChartDataConfig) {
		isFetched = true;

		currentChart = new Chart(
			ctx,
			_getChartConfig(chartData, formData, screenSize, locale)
		);
		return '';
	}
</script>

<svelte:window bind:innerWidth={screenSize} class="hidden" />

<div
	class="{isFetched
		? 'flex'
		: 'hidden'} h-100 bg-zinc-800 px-5 pt-10 text-white md:h-full"
>
	<canvas id="chart" />
</div>
{#if invalidDateAlert}
	<Alert.Root
		class="mx-auto mt-10 w-fit whitespace-nowrap border-0 bg-rose-400 text-zinc-800"
	>
		<Alert.Title class="font-semibold">Invalid date input detected!</Alert.Title
		>
		<Alert.Description>
			Your custom date input is invalid, please make sure that the year and
			month inputs are valid.
		</Alert.Description>
	</Alert.Root>
{/if}
<div
	class="mx-auto flex flex-col items-center justify-center place-self-center bg-zinc-800 py-10 sm:flex-row lg:col-span-7"
>
	<form method="POST" on:submit={handleSubmit}>
		<div class="flex flex-col">
			<div
				class="flex flex-col items-center pb-5 sm:flex-row sm:justify-between sm:space-x-10"
			>
				<input
					class="focus:ring-3 mb-5 flex w-full items-center justify-center rounded-lg border-2 border-rose-200 bg-rose-100 bg-transparent bg-opacity-0 py-2 text-center font-medium text-rose-200 drop-shadow-lg placeholder:text-rose-200 focus:bg-opacity-5 focus:outline-none focus:ring-zinc-800 sm:mb-0 sm:inline-flex sm:w-80 md:mr-auto md:py-3 md:pr-5"
					bind:value={username}
					placeholder="Enter your username"
				/>
				<div
					class="group flex w-full rounded-lg focus-within:ring-4 focus-within:ring-zinc-800 sm:mr-10 sm:inline-flex sm:w-48"
				>
					<button
						class="w-full items-center justify-center whitespace-nowrap rounded-l-lg border-y border-l border-rose-700 bg-rose-400 px-5 py-2 text-center text-sm font-semibold text-zinc-100 hover:bg-rose-500 focus:outline-none md:w-48 md:py-3 md:text-base"
						type="submit"
					>
						Generate chart
					</button>
					<GenerateOptions
						bind:chartType
						bind:chartScale
						bind:limit
						bind:offset
						bind:startTimestamp
						bind:endTimestamp
						bind:dateRange
						bind:invalidDateAlert
					/>
				</div>
			</div>
		</div>
	</form>
</div>
{#if $queryData.fetching}
	<Loader {username} />
{:else if $queryData.error}
	<div
		class="fixed bottom-0 left-0 right-0 top-0 z-50 flex h-screen w-full flex-col items-center justify-center overflow-hidden bg-zinc-800"
	>
		<p class="w-1/3 text-center text-white">
			Some error occurred, either check your username, or its on our side.
			Eitherway, here's the error: <b
				>{$queryData.error.name}: {$queryData.error.message}
				{$queryData.error.response}
				{$queryData.error.graphQLErrors}</b
			>
		</p>
	</div>
{:else if isMounted && $queryData.data}
	{generateChart($queryData.data.historyFm.getWeeklyCharts.chart)}
{/if}
