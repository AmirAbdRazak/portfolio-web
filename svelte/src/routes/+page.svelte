<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import chartImage from '$lib/assets/chartexample.png';
	import { Github, Globe2 } from 'lucide-svelte';
</script>

<section class="bg-zinc-900">
	<div class="mx-auto max-w-screen-xl px-4 py-8 text-left lg:px-12 lg:py-16">
		<Avatar.Root class="mb-4 h-16 w-16">
			<Avatar.Image
				src="https://github.com/amirabdrazak.png"
				alt="@amirabdrazak"
			/>
			<Avatar.Fallback>AR</Avatar.Fallback>
		</Avatar.Root>
		<h1
			class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-zinc-200 md:text-5xl lg:text-5xl"
		>
			Backend Engineer
		</h1>
		<p class="text-md mb-8 pr-80 font-normal text-zinc-500 lg:text-lg">
			I'm Amir Razak, a backend engineer with an insatiable curiosity for
			bleeding-edge technology. My expertise spans a wide range of areas, from
			building robust RESTful APIs to seamlessly managing database migrations.
		</p>
		<div class="flex flex-row items-center">
			<Button class="rounded-lg bg-violet-300 px-5 text-lg text-zinc-800"
				>Download Resume</Button
			>
			<p class="px-5 text-zinc-500 underline">Or see it on the web</p>
		</div>
	</div>
</section>
<section class="bg-zinc-800">
	<h1 class="py-10 text-center text-4xl font-semibold text-violet-200">
		Portfolio
	</h1>
	<div class="grid transform grid-cols-2 gap-10 px-10 pb-20">
		<Card.Root
			class="border-0 bg-zinc-900 text-zinc-200 transition-transform duration-300 ease-in-out hover:bg-neutral-900"
		>
			<Card.Header>
				<Card.Title class="text-violet-200">LastFM Charts</Card.Title>
				<Card.Description
					>Listening history data visualization for LastFM users
				</Card.Description>
			</Card.Header>
			<Card.Content>
				<img src={chartImage} alt="Chart" />
			</Card.Content>
			<Card.Footer class="space-x-5">
				<a href="https://github.com/AmirAbdRazak/portfolio-web">
					<Button class="rounded-lg bg-zinc-700 hover:bg-zinc-800">
						<div
							class="flex-cols flex items-center space-x-2 whitespace-nowrap text-zinc-200"
						>
							<p>GitHub</p>
							<Github />
						</div>
					</Button>
				</a>
				<a href="/charts">
					<Button class="rounded-lg bg-violet-300 hover:bg-violet-200">
						<div
							class="flex-cols flex items-center space-x-2 whitespace-nowrap text-zinc-800"
						>
							<p>Live Demo</p>
							<Globe2 />
						</div>
					</Button>
				</a>
			</Card.Footer>
		</Card.Root>
	</div>
</section>
