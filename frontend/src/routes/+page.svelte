<script lang="ts">
	import { Effect } from 'effect';

	let state: { result: string; count: number } = $state({
		result: 'Loading...',
		count: 0
	});

	// Simple Effect program that succeeds with a greeting
	const program = Effect.succeed('Hello from Effect!');

	// More complex Effect program that does some computation
	const countProgram = Effect.gen(function* () {
		// Simulate some async work
		yield* Effect.sleep('100 millis');
		return state.count + 1;
	});

	$effect(() => {
		Effect.runPromise(program).then((greeting) => {
			state.result = greeting;
		});
	});

	// Function to increment counter using Effect
	function increment() {
		Effect.runPromise(countProgram).then((newCount) => {
			state.count = newCount;
		});
	}
</script>

<div class="m-8 p-4 border border-gray-300 rounded-md shadow-sm">
	<h1>Pure bible</h1>
	<p>Open source bible written in swiftui</p>
</div>
