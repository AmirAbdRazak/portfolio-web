<script lang="ts">
	import chartImage from '$lib/assets/chartexample.png';
	import { START_TIMESTAMP, formDataStore } from './FormDataStore';
	import { goto } from '$app/navigation';
	import * as Popover from '$lib/components/ui/popover';
	import * as Select from '$lib/components/ui/select';
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

	let displayChartScale = 'Linear';

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chartType,
			limit: limit[0],
			offset: offset[0],
			chartScale,
			startTimestamp,
			endTimestamp
		});
		goto(`charts/${username}`, { replaceState: false });
	}
</script>

<section class="bg-slate-800 px-4 py-10 lg:px-10 lg:py-20">
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
					<div class="flex flex-col pb-5 sm:flex-row">
						<input
							class="focus:ring-3 mb-5 mr-auto flex w-80 items-center justify-center rounded-lg border-2 border-slate-700 bg-slate-400 py-2 text-center font-medium text-slate-800 drop-shadow-lg placeholder:text-slate-700 focus:outline-none focus:ring-slate-800 sm:mb-0 sm:inline-flex md:mr-auto md:py-3 md:pr-5"
							bind:value={username}
							placeholder="Enter your username"
						/>
						<div
							class="group mx-auto flex rounded-lg focus-within:ring-4 focus-within:ring-slate-800 sm:ml-auto sm:inline-flex"
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
									<div class="flex flex-row justify-between">
										<div class="grid w-full max-w-sm items-center gap-1.5">
											<Label for="year" class="text-slate-200">Start Year</Label
											>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Year"
												type="number"
												value={2002}
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
												value={1}
											/>
										</div>
									</div>
									<div class="flex flex-row justify-between">
										<div class="grid w-full max-w-sm items-center gap-1.5">
											<Label for="year" class="text-slate-200">End Year</Label>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Year"
												type="number"
												value={new Date().getFullYear()}
											/>
										</div>
										<div class="grid w-full max-w-sm gap-1.5">
											<Label for="month" class="text-slate-200">End Month</Label
											>
											<Input
												class="remove-arrow w-28 text-slate-200"
												placeholder="Month"
												type="number"
												value={new Date().getMonth()}
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
