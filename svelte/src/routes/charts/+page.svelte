<script lang="ts">
	import chartImage from '$lib/assets/chartexample.png';
	import { START_TIMESTAMP, formDataStore } from './FormDataStore';
	import { goto } from '$app/navigation';
	import * as Popover from '$lib/components/ui/popover';
	import * as Select from '$lib/components/ui/select';
	import * as Alert from '$lib/components/ui/alert';
	import { Slider } from '$lib/components/ui/slider';
	import { Input } from '$lib/components/ui/input';
	import { Settings } from 'lucide-svelte';
	import { Label } from '$lib/components/ui/label';

	let username: string;
	let chartType: string = 'Artist';
	let limit: number[] = [10];
	let offset: number[] = [0];
	let chartScale: 'linear' | 'logarithmic' = 'linear';
	let startTimestamp = START_TIMESTAMP;
	let endTimestamp = Date.now() / 1000;

	let startYear = 2002;
	let startMonth = 1;
	let endYear = new Date().getFullYear();
	let endMonth = new Date().getMonth();
	let dateRange = 'Month';
	let invalidDateAlert = false;

	let displayChartScale = 'Linear';

	$: {
		invalidDateAlert = false;
		endTimestamp = Date.now() / 1000;
		if (dateRange === 'Week') {
			let dateNow = new Date();
			dateNow.setDate(dateNow.getDate() - 7);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Month') {
			let dateNow = new Date();
			dateNow.setMonth(dateNow.getMonth() - 1);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Quarter') {
			let dateNow = new Date();
			const currMonth = dateNow.getMonth();
			if (currMonth < 4) {
				dateNow.setFullYear(dateNow.getFullYear() - 1);
				dateNow.setMonth(dateNow.getMonth() + 9);
			} else {
				dateNow.setMonth(dateNow.getMonth() - 3);
			}
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Year') {
			let dateNow = new Date();
			dateNow.setFullYear(dateNow.getFullYear() - 1);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Custom') {
			const customStart = new Date(`${startYear}-${startMonth}-01`);
			const customEnd = new Date(`${endYear}-${endMonth}-01`);
			if (!isNaN(customStart.getTime()) && !isNaN(customEnd.getTime())) {
				startTimestamp = customStart.getTime() / 1000;
				endTimestamp = customEnd.getTime() / 1000;
			} else if (!startYear || !startMonth || !endYear || !endMonth) {
				const fixedStart = new Date(
					`${startYear || 2002}-${startMonth || 1}-01`
				);
				const fixedEnd = new Date(
					`${endYear || new Date().getFullYear()}-${
						endMonth || new Date().getMonth()
					}-01`
				);
				startTimestamp = fixedStart.getTime() / 1000;
				endTimestamp = fixedEnd.getTime() / 1000;
			} else {
				invalidDateAlert = true;
			}
		}
	}

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

<section class="bg-slate-800 px-4 py-10 lg:px-10 lg:py-20">
	{#if invalidDateAlert}
		<Alert.Root class="border-0 bg-rose-400 text-slate-800">
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
							class="focus:ring-3 mb-5 flex w-80 items-center justify-center rounded-lg border-2 border-slate-700 bg-slate-400 py-2 text-center font-medium text-slate-800 drop-shadow-lg placeholder:text-slate-700 focus:outline-none focus:ring-slate-800 sm:mb-0 sm:inline-flex md:mr-auto md:py-3 md:pr-5"
							bind:value={username}
							placeholder="Enter your username"
						/>
						<div
							class="group flex rounded-lg focus-within:ring-4 focus-within:ring-slate-800 sm:inline-flex"
						>
							<button
								class="items-center justify-center rounded-l-lg border-y border-l border-rose-700 bg-rose-400 px-5 py-2 text-center text-sm font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3 md:text-base"
								type="submit"
							>
								Generate chart
							</button>

							<Popover.Root>
								<Popover.Trigger
									type="button"
									class="cursor-pointer items-center justify-center rounded-r-lg border-y border-r border-rose-700 bg-rose-400 px-5 py-2 text-center text-xl font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3"
								>
									<Settings />
								</Popover.Trigger>
								<Popover.Content class="space-y-5 border-0 bg-slate-900">
									<div class="flex flex-row justify-between">
										<Select.Root
											onSelectedChange={(e) => {
												const val = e?.value;
												if (typeof val == 'string') {
													chartType = val;
												}
											}}
										>
											<Select.Trigger
												class="w-50 min-w-[7.5rem] text-slate-200"
											>
												<Select.Value bind:placeholder={chartType} />
											</Select.Trigger>
											<Select.Content
												class="border-0 bg-slate-700 text-slate-200"
											>
												<Select.Item value="Artist">Artist</Select.Item>
												<Select.Item value="Album">Album</Select.Item>
												<Select.Item value="Track">Track</Select.Item>
											</Select.Content>
										</Select.Root>
										<Select.Root
											onSelectedChange={(e) => {
												const val = e?.value;
												if (
													typeof val == 'string' &&
													(val == 'linear' || val == 'logarithmic')
												) {
													chartScale = val;
													displayChartScale =
														val == 'linear' ? 'Linear' : 'Log';
												}
											}}
										>
											<Select.Trigger
												class="w-50 min-w-[7.5rem] text-slate-200"
											>
												<Select.Value bind:placeholder={displayChartScale} />
											</Select.Trigger>
											<Select.Content
												class="w-50 border-0 bg-slate-700 text-slate-200"
											>
												<Select.Item value="linear">Linear</Select.Item>
												<Select.Item value="logarithmic">Log</Select.Item>
											</Select.Content>
										</Select.Root>
									</div>
									<div class="space-y-2">
										<Label for="limitSlider" class="text-slate-200"
											>Number of Entries: {limit}</Label
										>
										<Slider
											class="mx-auto w-60"
											bind:value={limit}
											max={100}
											step={1}
										/>
									</div>
									<div class="space-y-2">
										<Label for="limitSlider" class="text-slate-200"
											>Offset by: {offset}</Label
										>
										<Slider
											class="mx-auto w-60"
											bind:value={offset}
											max={100}
											step={1}
										/>
									</div>
									<Select.Root
										onSelectedChange={(e) => {
											const val = e?.value;
											if (typeof val == 'string') {
												dateRange = val;
											}
										}}
									>
										<Select.Trigger class="w-50 min-w-[9rem] text-slate-200">
											<Select.Value bind:placeholder={dateRange} />
										</Select.Trigger>
										<Select.Content
											class="border-0 bg-slate-700 text-slate-200"
										>
											<Select.Item value="Week">Weekly</Select.Item>
											<Select.Item value="Month">Monthly</Select.Item>
											<Select.Item value="Quarter">Quarterly</Select.Item>
											<Select.Item value="Year">Annually</Select.Item>
											<Select.Item value="Custom">Custom</Select.Item>
										</Select.Content>
									</Select.Root>
									<div
										class="{dateRange == 'Custom'
											? 'flex'
											: 'hidden'} flex-row justify-between"
									>
										<div class="grid w-full max-w-sm items-center gap-1.5">
											<Label for="year" class="text-slate-200">Start Year</Label
											>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Year"
												type="number"
												bind:value={startYear}
											/>
										</div>
										<div class="grid w-full max-w-sm gap-1.5">
											<Label for="month" class="text-slate-200"
												>Start Month</Label
											>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Month"
												type="number"
												bind:value={startMonth}
											/>
										</div>
									</div>
									<div
										class="{dateRange == 'Custom'
											? 'flex'
											: 'hidden'} flex-row justify-between"
									>
										<div class="grid w-full max-w-sm items-center gap-1.5">
											<Label for="year" class="text-slate-200">End Year</Label>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Year"
												type="number"
												bind:value={endYear}
											/>
										</div>
										<div class="grid w-full max-w-sm gap-1.5">
											<Label for="month" class="text-slate-200">End Month</Label
											>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Month"
												type="number"
												bind:value={endMonth}
											/>
										</div>
									</div>
								</Popover.Content>
							</Popover.Root>
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
