<script lang="ts">
	import chart_image from '$lib/assets/chart_image.png';

	let username: string;
</script>

<section class="bg-white dark:bg-slate-900 p-10">
	<div class="grid max-w-screen-xl px-4 py-8 mx-auto lg:gap-8 xl:gap-0 lg:py-16 lg:grid-cols-12">
		<div class="mr-auto place-self-center lg:col-span-7">
			<h1
				class="max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-white"
			>
				<!-- svelte-ignore a11y-missing-attribute -->
				Check out your listening trends for <a class="text-rose-400">LastFM</a> users
			</h1>
			<p
				class="max-w-2xl mb-6 font-light text-slate-700 lg:mb-8 md:text-lg lg:text-xl dark:text-slate-200"
			>
				Compare and observe how your music taste evolved over time, tracks you've been (or had been)
				obsessed with, artists you've been a number one fan of and more!
			</p>
			<form method="POST" action="/history_fm/charts/{username}">
				<div class="flex pb-5">
					<input
						class="inline-flex items-center justify-center pr-5 py-3 mr-3 text-slate-900 font-medium drop-shadow-lg text-center bg-slate-100 border-2 border-slate-700 rounded-lg focus:ring-3 focus:ring-slate-800 focus:outline-none"
						bind:value={username}
						placeholder="Enter your username"
					/>
					<div class="group inline-flex focus-within:ring-4 focus-within:ring-slate-800 rounded-lg">
						<button
							class="items-center justify-center px-5 py-3 text-base font-semibold text-center bg-rose-400 border rounded-l-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none"
						>
							Generate chart
						</button>
						<select
							id="chart_type"
							class="items-center justify-center px-5 py-3 text-base font-semibold text-center bg-rose-400 border-y border-r rounded-r-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none cursor-pointer"
						>
							<option selected value="artist">Artist</option>
							<option value="album">Album</option>
							<option value="track">Track</option>
						</select>
					</div>
				</div>
			</form>
		</div>
		<div class="hidden lg:mt-0 lg:col-span-5 lg:flex">
			<img src={chart_image} alt="temp chart" class="object-contain" />
		</div>
	</div>
</section>
