<script lang="ts">
	import chartImage from '$lib/assets/chartexample.png';
	import { START_TIMESTAMP, formDataStore } from './FormDataStore';
	import { goto } from '$app/navigation';

	let username: string;
	let chartType: string;
	let limit: number = 10;
	let offset: number = 0;
	let chartScale: 'linear' | 'logarithmic';
	let startTimestamp = START_TIMESTAMP;
	let endTimestamp = Date.now() * 1000;

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({
			chartType,
			limit,
			offset,
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
							class="focus:ring-3 mx-auto mb-5 flex w-80 items-center justify-center rounded-lg border-2 border-slate-700 bg-slate-100 py-2 text-center font-medium text-slate-800 drop-shadow-lg focus:outline-none focus:ring-slate-800 sm:mb-0 sm:inline-flex md:mr-auto md:py-3 md:pr-5"
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
							<select
								id="chartScale"
								class="cursor-pointer items-center justify-center border border-rose-700 bg-rose-400 px-5 py-2 text-center text-base font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3"
								bind:value={chartScale}
							>
								<option selected value="linear">Linear</option>
								<option value="logarithmic">Log</option>
							</select>
							<select
								id="chartType"
								class="cursor-pointer items-center justify-center rounded-r-lg border-y border-r border-rose-700 bg-rose-400 px-5 py-2 text-center text-base font-semibold text-slate-100 hover:bg-rose-500 focus:outline-none md:py-3"
								bind:value={chartType}
							>
								<option selected value="Artist">Artist</option>
								<option value="Album">Album</option>
								<option value="Track">Track</option>
							</select>
						</div>
					</div>
					<div class="flex flex-col">
						<div class="pb-2">
							<label
								for="limit-range"
								class="mb-2 inline-flex text-sm font-medium text-white"
								>Data Limit:
							</label>
							<input
								bind:value={limit}
								type="number"
								class="text-md mb-2 inline-flex w-24 justify-center rounded-lg border-slate-200 bg-slate-700 px-3 text-white focus:outline-none"
							/>
						</div>
						<input
							id="limit-range"
							type="range"
							bind:value={limit}
							class="range-sm mb-6 h-1 w-full cursor-pointer appearance-none rounded-lg bg-slate-700"
						/>
						<div class="pb-2">
							<label
								for="offset-range"
								class="mb-2 inline-flex text-sm font-medium text-white"
								>Data Offset:
							</label>
							<input
								bind:value={offset}
								type="number"
								class="text-md mb-2 inline-flex w-24 rounded-lg border-none bg-slate-700 px-3 text-white focus:outline-none"
							/>
						</div>
						<input
							id="offset-range"
							type="range"
							bind:value={offset}
							class="range-sm mb-6 h-1 w-full cursor-pointer appearance-none rounded-lg bg-slate-700"
						/>
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
