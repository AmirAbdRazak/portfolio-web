<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import chartImage from '$lib/assets/chartexample.png';
	import PortfolioCard from './PortfolioCard.svelte';
</script>

<section class="bg-gradient-to-b from-indigo-200 to-violet-300">
	<div class="mx-auto max-w-screen-xl px-4 py-8 text-left lg:px-12 lg:py-16">
		<div class="flex-cols flex justify-around">
			<div>
				<h1
					class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-zinc-900 md:text-5xl lg:text-5xl"
				>
					Backend Engineer
				</h1>
				<p class="text-md mb-8 font-normal text-zinc-800 lg:text-lg">
					I'm Amir Razak, a backend engineer with an insatiable curiosity for
					bleeding-edge technology. My expertise spans a wide range of areas,
					from building robust RESTful APIs to seamlessly managing database
					migrations.
				</p>
				<div class="flex flex-row items-center">
					<Button
						class="rounded-lg bg-zinc-800 px-5 text-lg font-semibold text-violet-200"
						>Download Resume</Button
					>
					<p class="px-5 text-zinc-800 underline">Or see it on the web</p>
				</div>
			</div>
			<Avatar.Root
				class="my-auto ml-20 mr-20 h-52 w-52 border-4 border-zinc-800"
			>
				<Avatar.Image
					src="https://github.com/amirabdrazak.png"
					alt="@amirabdrazak"
				/>
				<Avatar.Fallback>AR</Avatar.Fallback>
			</Avatar.Root>
		</div>
	</div>
</section>
<section class="bg-zinc-800">
	<h1 class="py-10 text-center text-4xl font-semibold text-violet-100">
		Portfolio
	</h1>
	<div class="grid transform grid-cols-2 gap-10 px-10 pb-20">
		<PortfolioCard
			title="LastFM Chart"
			description="Listening history data visualization for LastFM users"
			{chartImage}
			githubRepo="https://github.com/AmirAbdRazak/portfolio-web"
			chartLink="/charts"
		/>
	</div>
</section>
