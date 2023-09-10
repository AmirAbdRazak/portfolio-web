<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Github, Globe2 } from 'lucide-svelte';

	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';

	export let title: string;
	export let description: string;
	export let cardImage: string;
	export let githubRepo: string;
	export let cardLink: string;
	export let techStack: string[];
</script>

<Tooltip.Root openDelay={2}>
	<Tooltip.Trigger>
		<Card.Root
			class="border-0 bg-opacity-80 bg-gradient-to-br from-indigo-200 to-violet-300 text-zinc-200 transition-transform duration-300 ease-in-out"
		>
			<Card.Header class="text-left">
				<Card.Title class="text-zinc-800">{title}</Card.Title>
				<Card.Description class="text-zinc-700">{description}</Card.Description>
			</Card.Header>
			<Card.Content>
				<img src={cardImage} alt={title} class="rounded-md object-cover" />
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
				<a href={cardLink}>
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
	</Tooltip.Trigger>
	<Tooltip.Content class=" border-2  bg-zinc-700 text-violet-100">
		<p>Tech Stack:</p>
		<ul>
			{#each techStack as tech}
				<li>⇒ {tech}</li>
			{/each}
		</ul>
	</Tooltip.Content>
</Tooltip.Root>
