<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Github, Globe2 } from 'lucide-svelte';

	import { Button } from '$lib/components/ui/button';

	export let title: string;
	export let description: string;
	export let chartImage: string;
	export let githubRepo: string;
	export let chartLink: string;
</script>

<Card.Root
	class="border-0 bg-opacity-80 bg-gradient-to-br from-indigo-200 to-violet-300 text-zinc-200 transition-transform duration-300 ease-in-out"
>
	<Card.Header>
		<Card.Title class="text-zinc-800">{title}</Card.Title>
		<Card.Description class="text-zinc-700">{description}</Card.Description>
	</Card.Header>
	<Card.Content>
		<img src={chartImage} alt="Chart" class="rounded-md" />
	</Card.Content>
	<Card.Footer class="space-x-5">
		<a href={githubRepo}>
			<Button class="rounded-lg bg-zinc-800 hover:bg-zinc-700">
				<div
					class="flex-cols flex items-center space-x-2 whitespace-nowrap text-violet-100"
				>
					<p>GitHub</p>
					<Github />
				</div>
			</Button>
		</a>
		<a href={chartLink}>
			<Button
				class="rounded-lg border-2 border-zinc-700 bg-transparent hover:bg-violet-200"
			>
				<div
					class="flex-cols flex items-center space-x-2 whitespace-nowrap font-semibold text-zinc-800"
				>
					<p>Live Demo</p>
					<Globe2 />
				</div>
			</Button>
		</a>
	</Card.Footer>
</Card.Root>
