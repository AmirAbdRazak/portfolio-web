<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
</script>

<section class="bg-zinc-800">
	<div class="px-20 py-16">
		<h1
			class="mb-4 text-center text-4xl font-extrabold leading-none tracking-tight text-white md:text-5xl xl:text-6xl"
		>
			<!-- svelte-ignore a11y-missing-attribute -->
			Stay motivated with your <a class="text-cyan-200">workouts!</a>
		</h1>
		<p
			class="mb-6 text-center font-light text-zinc-200 md:text-lg lg:mb-8 lg:text-xl"
		>
			See how far you've come along your journey by seeing your progress come to
			life! Analyze your data through graphs and help you stay motivated in your
			road to success.
		</p>
	</div>
</section>

<section class="bg-zinc-800 pb-16">
	<Card.Root class="mx-52 border-0 bg-zinc-700">
		<Card.Header>
			<Card.Title class="text-center text-4xl text-cyan-200">Login</Card.Title>
			<Card.Description class="px-20 text-center text-zinc-200">
				I don't trust myself with passwords and neither should you, here's an
				alternative way to login</Card.Description
			>
		</Card.Header>
		<Card.Content class="mx-auto w-fit space-y-4 text-zinc-200">
			<div class="space-y-2">
				<label for="username">Username: </label>
				<Input
					id="username"
					class="border-2 border-zinc-400 focus-visible:border-cyan-100 focus-visible:ring-0 focus-visible:ring-offset-0"
				/>
			</div>
			<div class="flex flex-row space-x-4">
				<div class="space-y-2">
					<label for="username">Question: </label>
					<Input
						id="username"
						class="border-2 border-zinc-400 focus-visible:border-cyan-100 focus-visible:ring-0 focus-visible:ring-offset-0"
					/>
				</div>
				<div class="space-y-2">
					<label for="username">Question: </label>
					<Input
						id="username"
						class="border-2 border-zinc-400 focus-visible:border-cyan-100 focus-visible:ring-0 focus-visible:ring-offset-0"
					/>
				</div>
			</div>
			<div class="space-y-2">
				<label for="username">Answer: </label>
				<Input
					id="username"
					class="border-2 border-zinc-400 focus-visible:border-cyan-100 focus-visible:ring-0 focus-visible:ring-offset-0"
				/>
			</div>
			<Button class="w-full bg-cyan-200 text-lg text-zinc-800">Log in</Button>
		</Card.Content>
		<Card.Footer class="w-full text-zinc-200 ">
			<p class="mx-auto">
				Or if you don't have an account yet, <a
					href="/workout/register"
					class="underline">register</a
				>
			</p>
		</Card.Footer>
	</Card.Root>
</section>
