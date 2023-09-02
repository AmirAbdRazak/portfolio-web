<script lang="ts">
	import chartImage from '$lib/assets/chartexample.png';
	import { formDataStore } from './FormDataStore';
	import { goto } from '$app/navigation';
	import GenerateOptions from './GenerateOptions.svelte';
	import * as Alert from '$lib/components/ui/alert';

	let username: string;
	let chartType: string = 'Artist';
	let limit: number[] = [10];
	let offset: number[] = [0];
	let chartScale: 'linear' | 'logarithmic' = 'linear';
	let oneMonthAgo = new Date();
	oneMonthAgo.setMonth(oneMonthAgo.getMonth() - 1);
	let startTimestamp = oneMonthAgo.getTime() / 1000;
	let endTimestamp = Date.now() / 1000;
	let dateRange: 'Week' | 'Month' | 'Quarter' | 'Year' | 'Custom' = 'Month';
	let invalidDateAlert = false;

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chartType,
			limit: limit[0],
			offset: offset[0],
			chartScale,
			startTimestamp,
			endTimestamp,
			dateRange
		});
		goto(`charts/${username}`, { replaceState: false });
	}
</script>

<section class="bg-slate-800 px-4 py-6 lg:px-10 lg:pb-20">
	{#if invalidDateAlert}
		<Alert.Root
			class="mx-auto w-fit whitespace-nowrap border-0 bg-rose-400 text-slate-800"
		>
			<Alert.Title class="font-semibold"
				>Invalid date input detected!</Alert.Title
			>
			<Alert.Description>
				Your custom date input is invalid, please make sure that the year and
				month inputs are valid.
			</Alert.Description>
		</Alert.Root>
	{/if}
	<div
		class="lgs:gap-8 mx-auto flex max-w-screen-xl flex-col px-4 py-8 md:grid lg:grid-cols-12 lg:py-16 xl:gap-0"
	>
		<div class="mr-auto place-self-center lg:col-span-7">
			<h1
				class="mb-4 max-w-2xl text-center text-4xl font-extrabold leading-none tracking-tight text-white md:text-left md:text-5xl xl:text-6xl"
			>
				<!-- svelte-ignore a11y-missing-attribute -->
				Check out your listening trends for <a class="text-rose-400">LastFM</a> users
			</h1>
			<p
				class="mb-6 max-w-2xl text-center font-light text-slate-200 md:text-left md:text-lg lg:mb-8 lg:text-xl"
			>
				Compare and observe how your music taste evolved over time, tracks
				you've been (or had been) obsessed with, artists you've been a number
				one fan of and more!
			</p>
			<div class="flex px-5 pb-10 lg:hidden">
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class="transform transition-transform duration-300 ease-in-out hover:scale-110 lg:hidden"
					on:click={() => {
						goto('charts/ryzlesalt');
					}}
				>
					<img
						src={chartImage}
						alt="temp chart"
						class="object-contain pb-5 pt-7 drop-shadow-2xl"
					/>
					<p class="text-center text-sm font-light text-slate-100">
						If you want to try out without a LastFM account, click here!
					</p>
				</div>
			</div>
			<form method="POST" on:submit={handleSubmit}>
				<div class="flex flex-col">
					<div
						class="flex flex-col items-center pb-5 sm:flex-row sm:justify-between"
					>
						<input
							class="focus:ring-3 mb-5 flex w-full items-center justify-center rounded-lg border-2 border-slate-700 bg-slate-400 py-2 text-center font-medium text-slate-800 drop-shadow-lg placeholder:text-slate-700 focus:outline-none focus:ring-slate-800 sm:mb-0 sm:inline-flex sm:w-80 md:mr-auto md:py-3 md:pr-5"
							bind:value={username}
							placeholder="Enter your username"
						/>
						<div
							class="group flex w-full rounded-lg focus-within:ring-4 focus-within:ring-slate-800 sm:mr-10 sm:inline-flex sm:w-48"
						>
							<button
								class="w-full items-center justify-center whitespace-nowrap rounded-l-lg border-y border-l border-rose-700 bg-rose-400 px-5 py-2 text-center text-sm font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:w-48 md:py-3 md:text-base"
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
		<div class="hidden lg:col-span-5 lg:flex">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				class="hidden transform transition-transform duration-300 ease-in-out hover:scale-110 lg:mt-20 lg:flex lg:flex-col"
				on:click={() => {
					goto('charts/ryzlesalt');
				}}
			>
				<img
					src={chartImage}
					alt="temp chart"
					class="object-contain pb-5 pt-7 drop-shadow-2xl"
				/>
				<p class="text-center text-sm font-light text-slate-100">
					If you want to try out without a LastFM account, click here!
				</p>
			</div>
		</div>
	</div>
</section>
