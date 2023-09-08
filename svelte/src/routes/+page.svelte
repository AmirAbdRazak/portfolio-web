<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
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
</section>
