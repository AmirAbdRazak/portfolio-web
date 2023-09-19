<script lang="ts">
	import * as Card from '$lib/components/ui/card';
</script>

<section class="bg-zinc-800 px-4 py-6 lg:px-10 lg:pb-20">
	<div class="flex flex-row px-10 py-16">
		<div class="pr-20">
			<h1
				class="mb-4 max-w-2xl text-center text-4xl font-extrabold leading-none tracking-tight text-white md:text-left md:text-5xl xl:text-6xl"
			>
				<!-- svelte-ignore a11y-missing-attribute -->
				Stay motivated with your <a class="text-cyan-200">workouts</a> and analyze
				your progress!
			</h1>
			<p
				class="mb-6 max-w-2xl text-center font-light text-zinc-200 md:text-left md:text-lg lg:mb-8 lg:text-xl"
			>
				See how far you've come along your journey by seeing your progress come
				to life! This application will allow you to analyze your data through
				graphs and help you stay motivated in your road to success.
			</p>
		</div>
		<div>
			<Card.Root class="border-0 bg-zinc-700">
				<Card.Header>
					<Card.Title class="text-4xl text-cyan-200">Login</Card.Title>
					<Card.Description class="text-cyan-100" />
				</Card.Header>
				<Card.Content class="text-zinc-200">
					<p>Card Content</p>
				</Card.Content>
				<Card.Footer class="text-zinc-200">
					<p>
						Or if you don't have an account yet <a
							href="/workout/register"
							class="underline">register</a
						>
					</p>
				</Card.Footer>
			</Card.Root>
		</div>
	</div>
</section>
